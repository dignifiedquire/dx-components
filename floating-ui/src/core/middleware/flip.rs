//! Port of `@floating-ui/core/src/middleware/flip.ts` (229 lines).

use crate::types::*;
use crate::utils::*;

/// Options for the flip middleware.
/// Source: flip.ts `FlipOptions` (lines 15-53)
#[derive(Debug, Clone, PartialEq)]
pub struct FlipOptions {
    /// Check overflow on the main axis. Default: true
    pub main_axis: bool,
    /// Check overflow on the cross axis. Default: true
    pub cross_axis: bool,
    /// Explicit fallback placements to try.
    pub fallback_placements: Option<Vec<Placement>>,
    /// Strategy when no placements fit. Default: BestFit
    pub fallback_strategy: FlipFallbackStrategy,
    /// Whether to try perpendicular axis placements. Default: None
    pub fallback_axis_side_direction: Option<Alignment>,
    /// Whether to flip alignment too. Default: true
    pub flip_alignment: bool,
    /// Detect overflow options.
    pub detect_overflow: DetectOverflowOptions,
}

impl Default for FlipOptions {
    fn default() -> Self {
        Self {
            main_axis: true,
            cross_axis: true,
            fallback_placements: None,
            fallback_strategy: FlipFallbackStrategy::BestFit,
            fallback_axis_side_direction: None,
            flip_alignment: true,
            detect_overflow: DetectOverflowOptions::default(),
        }
    }
}

/// What to do when no placement fits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlipFallbackStrategy {
    /// Pick the placement with least overflow.
    #[default]
    BestFit,
    /// Fall back to the initial placement.
    InitialPlacement,
}

/// Compute flip middleware result.
/// Source: flip.ts lines 61-229
pub fn compute(
    state: &MiddlewareState,
    options: &FlipOptions,
    detect_overflow_fn: &dyn Fn(&MiddlewareState, &DetectOverflowOptions) -> SideObject,
) -> MiddlewareReturn {
    let placement = state.placement;
    let initial_placement = state.initial_placement;

    // Skip if arrow caused alignment offset reset
    if let Some(ref arrow_data) = state.middleware_data.arrow {
        if arrow_data.alignment_offset.is_some() {
            return MiddlewareReturn::default();
        }
    }

    let side = get_side(placement);
    let initial_side_axis = get_side_axis(initial_placement);
    let is_base_placement = get_alignment(initial_placement).is_none();

    let fallback_placements = options.fallback_placements.clone().unwrap_or_else(|| {
        if is_base_placement || !options.flip_alignment {
            vec![get_opposite_placement(initial_placement)]
        } else {
            get_expanded_placements(initial_placement)
        }
    });

    let has_fallback_axis = options.fallback_axis_side_direction.is_some();

    let mut all_fallbacks = fallback_placements.clone();
    if options.fallback_placements.is_none() {
        if let Some(direction) = options.fallback_axis_side_direction {
            all_fallbacks.extend(get_opposite_axis_placements(
                initial_placement,
                options.flip_alignment,
                direction,
                false, // rtl
            ));
        }
    }

    let placements: Vec<Placement> = {
        let mut v = vec![initial_placement];
        v.extend(all_fallbacks);
        v
    };

    let overflow = detect_overflow_fn(state, &options.detect_overflow);

    let mut overflows: Vec<f64> = Vec::new();
    let mut overflows_data: Vec<PlacementOverflow> = state
        .middleware_data
        .flip
        .as_ref()
        .map(|f| f.overflows.clone())
        .unwrap_or_default();

    if options.main_axis {
        overflows.push(overflow.get(side));
    }

    if options.cross_axis {
        let (s1, s2) = get_alignment_sides(placement, &state.rects, false /* rtl */);
        overflows.push(overflow.get(s1));
        overflows.push(overflow.get(s2));
    }

    overflows_data.push(PlacementOverflow {
        placement,
        overflows: overflows.clone(),
    });

    // Check if any side is overflowing
    if !overflows.iter().all(|&v| v <= 0.0) {
        let flip_index = state
            .middleware_data
            .flip
            .as_ref()
            .map(|f| f.index)
            .unwrap_or(0);
        let next_index = flip_index + 1;

        if let Some(&next_placement) = placements.get(next_index) {
            // Check cross-axis override for 'alignment' mode
            let ignore_cross = if options.cross_axis {
                false
            } else {
                // cross_axis could be 'alignment' in TS — we treat bool true/false only
                false
            };

            let should_try_next = if ignore_cross {
                // Check if every placement on the initial axis overflows main
                overflows_data.iter().all(|d| {
                    if get_side_axis(d.placement) == initial_side_axis {
                        d.overflows.first().map(|&v| v > 0.0).unwrap_or(true)
                    } else {
                        true
                    }
                })
            } else {
                true
            };

            if should_try_next {
                return MiddlewareReturn {
                    data: MiddlewareDataUpdate::Flip(FlipData {
                        index: next_index,
                        overflows: overflows_data,
                    }),
                    reset: Reset::WithPlacement(next_placement),
                    ..Default::default()
                };
            }
        }

        // No more placements to try — find best fit
        let mut reset_placement = overflows_data
            .iter()
            .filter(|d| d.overflows.first().map(|&v| v <= 0.0).unwrap_or(false))
            .min_by(|a, b| {
                let a_cross = a.overflows.get(1).unwrap_or(&0.0);
                let b_cross = b.overflows.get(1).unwrap_or(&0.0);
                a_cross
                    .partial_cmp(b_cross)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|d| d.placement);

        if reset_placement.is_none() {
            match options.fallback_strategy {
                FlipFallbackStrategy::BestFit => {
                    let best = overflows_data
                        .iter()
                        .filter(|d| {
                            if has_fallback_axis {
                                let axis = get_side_axis(d.placement);
                                axis == initial_side_axis || axis == Axis::Y
                            } else {
                                true
                            }
                        })
                        .map(|d| {
                            let total: f64 = d.overflows.iter().filter(|&&v| v > 0.0).sum();
                            (d.placement, total)
                        })
                        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                        .map(|(p, _)| p);
                    if let Some(p) = best {
                        reset_placement = Some(p);
                    }
                }
                FlipFallbackStrategy::InitialPlacement => {
                    reset_placement = Some(initial_placement);
                }
            }
        }

        if let Some(rp) = reset_placement {
            if placement != rp {
                return MiddlewareReturn {
                    reset: Reset::WithPlacement(rp),
                    ..Default::default()
                };
            }
        }
    }

    MiddlewareReturn::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state(placement: Placement) -> MiddlewareState {
        MiddlewareState {
            x: 100.0,
            y: 700.0, // near bottom of viewport
            initial_placement: Placement::Bottom,
            placement,
            strategy: Strategy::Fixed,
            rects: ElementRects {
                reference: Rect {
                    x: 100.0,
                    y: 660.0,
                    width: 200.0,
                    height: 40.0,
                },
                floating: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 200.0,
                    height: 150.0,
                },
            },
            middleware_data: MiddlewareData::default(),
        }
    }

    fn viewport_overflow(state: &MiddlewareState, _opts: &DetectOverflowOptions) -> SideObject {
        // Viewport 0,0 to 1024,768
        let vp = crate::utils::rect_to_client_rect(Rect {
            x: 0.0,
            y: 0.0,
            width: 1024.0,
            height: 768.0,
        });
        crate::core::detect_overflow::detect_overflow(state, vp, _opts)
    }

    #[test]
    fn test_flip_when_overflowing_bottom() {
        let state = make_state(Placement::Bottom);
        let result = compute(&state, &FlipOptions::default(), &viewport_overflow);
        // Should flip to top since bottom overflows
        match result.reset {
            Reset::WithPlacement(p) => assert_eq!(p, Placement::Top),
            _ => panic!("Expected flip to Top, got {:?}", result.reset),
        }
    }
}
