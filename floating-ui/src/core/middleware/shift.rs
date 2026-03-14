//! Port of `@floating-ui/core/src/middleware/shift.ts` (216 lines).

use crate::types::*;
use crate::utils::*;

/// Options for the shift middleware.
/// Source: shift.ts `ShiftOptions` (lines 14-35)
#[derive(Debug, Clone, PartialEq)]
pub struct ShiftOptions {
    /// Check overflow on the main axis (cross-axis of the side). Default: true
    pub main_axis: bool,
    /// Check overflow on the cross axis (side axis). Default: false
    pub cross_axis: bool,
    /// Limiter to prevent detachment. Default: None
    pub limiter: Option<LimitShift>,
    /// Detect overflow options.
    pub detect_overflow: DetectOverflowOptions,
}

impl Default for ShiftOptions {
    fn default() -> Self {
        Self {
            main_axis: true,
            cross_axis: false,
            limiter: None,
            detect_overflow: DetectOverflowOptions::default(),
        }
    }
}

/// Options for the limitShift limiter.
/// Source: shift.ts `LimitShiftOptions` (lines 121-138)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LimitShift {
    /// Offset when limiting starts. Default: 0
    pub offset_main: f64,
    pub offset_cross: f64,
    /// Whether to limit the main axis. Default: true
    pub main_axis: bool,
    /// Whether to limit the cross axis. Default: true
    pub cross_axis: bool,
}

impl Default for LimitShift {
    fn default() -> Self {
        Self {
            offset_main: 0.0,
            offset_cross: 0.0,
            main_axis: true,
            cross_axis: true,
        }
    }
}

/// Compute shift middleware result.
/// Source: shift.ts lines 42-104
pub fn compute(
    state: &MiddlewareState,
    options: &ShiftOptions,
    detect_overflow_fn: &dyn Fn(&MiddlewareState, &DetectOverflowOptions) -> SideObject,
) -> MiddlewareReturn {
    let placement = state.placement;
    let overflow = detect_overflow_fn(state, &options.detect_overflow);

    // In shift, crossAxis = the axis along the side (e.g., Y for top/bottom)
    // mainAxis = the axis along the alignment (e.g., X for top/bottom)
    let cross_axis = get_side_axis(placement);
    let main_axis = get_opposite_axis(cross_axis);

    let mut main_axis_coord = if main_axis == Axis::X {
        state.x
    } else {
        state.y
    };
    let mut cross_axis_coord = if cross_axis == Axis::X {
        state.x
    } else {
        state.y
    };

    if options.main_axis {
        let min_side = if main_axis == Axis::Y {
            Side::Top
        } else {
            Side::Left
        };
        let max_side = if main_axis == Axis::Y {
            Side::Bottom
        } else {
            Side::Right
        };
        let min = main_axis_coord + overflow.get(min_side);
        let max = main_axis_coord - overflow.get(max_side);
        main_axis_coord = clamp(min, main_axis_coord, max);
    }

    if options.cross_axis {
        let min_side = if cross_axis == Axis::Y {
            Side::Top
        } else {
            Side::Left
        };
        let max_side = if cross_axis == Axis::Y {
            Side::Bottom
        } else {
            Side::Right
        };
        let min = cross_axis_coord + overflow.get(min_side);
        let max = cross_axis_coord - overflow.get(max_side);
        cross_axis_coord = clamp(min, cross_axis_coord, max);
    }

    // Apply limiter
    let (final_main, final_cross) = if let Some(ref limiter) = options.limiter {
        let limited = apply_limit_shift(
            state,
            limiter,
            main_axis_coord,
            cross_axis_coord,
            main_axis,
            cross_axis,
        );
        (limited.0, limited.1)
    } else {
        (main_axis_coord, cross_axis_coord)
    };

    let (result_x, result_y) = if main_axis == Axis::X {
        (final_main, final_cross)
    } else {
        (final_cross, final_main)
    };

    MiddlewareReturn {
        x: Some(result_x),
        y: Some(result_y),
        data: MiddlewareDataUpdate::Shift(ShiftData {
            x: result_x - state.x,
            y: result_y - state.y,
            enabled_x: if main_axis == Axis::X {
                options.main_axis
            } else {
                options.cross_axis
            },
            enabled_y: if main_axis == Axis::Y {
                options.main_axis
            } else {
                options.cross_axis
            },
        }),
        reset: Reset::None,
    }
}

/// Apply the limitShift limiter.
/// Source: shift.ts lines 143-216
fn apply_limit_shift(
    state: &MiddlewareState,
    limiter: &LimitShift,
    main_axis_coord: f64,
    cross_axis_coord: f64,
    main_axis: Axis,
    cross_axis: Axis,
) -> (f64, f64) {
    let placement = state.placement;
    let rects = &state.rects;
    let mut result_main = main_axis_coord;
    let mut result_cross = cross_axis_coord;

    if limiter.main_axis {
        let len = if main_axis == Axis::Y {
            Length::Height
        } else {
            Length::Width
        };
        let limit_min =
            rects.reference.axis_pos(main_axis) - rects.floating.length(len) + limiter.offset_main;
        let limit_max =
            rects.reference.axis_pos(main_axis) + rects.reference.length(len) - limiter.offset_main;

        if result_main < limit_min {
            result_main = limit_min;
        } else if result_main > limit_max {
            result_main = limit_max;
        }
    }

    if limiter.cross_axis {
        let len = if main_axis == Axis::Y {
            Length::Width
        } else {
            Length::Height
        };
        let is_origin = is_origin_side(get_side(placement));
        let offset_data = state.middleware_data.offset.as_ref();

        let limit_min = rects.reference.axis_pos(cross_axis) - rects.floating.length(len)
            + if is_origin {
                offset_data.map(|d| d.get(cross_axis)).unwrap_or(0.0)
            } else {
                0.0
            }
            + if is_origin { 0.0 } else { limiter.offset_cross };
        let limit_max = rects.reference.axis_pos(cross_axis)
            + rects.reference.length(len)
            + if is_origin {
                0.0
            } else {
                offset_data.map(|d| d.get(cross_axis)).unwrap_or(0.0)
            }
            - if is_origin { limiter.offset_cross } else { 0.0 };

        if result_cross < limit_min {
            result_cross = limit_min;
        } else if result_cross > limit_max {
            result_cross = limit_max;
        }
    }

    (result_main, result_cross)
}

impl OffsetData {
    fn get(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state(x: f64, y: f64) -> MiddlewareState {
        MiddlewareState {
            x,
            y,
            initial_placement: Placement::Bottom,
            placement: Placement::Bottom,
            strategy: Strategy::Fixed,
            rects: ElementRects {
                reference: Rect {
                    x: 100.0,
                    y: 100.0,
                    width: 200.0,
                    height: 40.0,
                },
                floating: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 300.0,
                    height: 100.0,
                },
            },
            middleware_data: MiddlewareData::default(),
        }
    }

    fn viewport_overflow(state: &MiddlewareState, opts: &DetectOverflowOptions) -> SideObject {
        let vp = crate::utils::rect_to_client_rect(Rect {
            x: 0.0,
            y: 0.0,
            width: 1024.0,
            height: 768.0,
        });
        crate::core::detect_overflow::detect_overflow(state, vp, opts)
    }

    #[test]
    fn test_shift_within_viewport() {
        // Floating is centered and within viewport - no shift needed
        let state = make_state(50.0, 140.0);
        let result = compute(&state, &ShiftOptions::default(), &viewport_overflow);
        assert_eq!(result.x, Some(50.0));
        assert_eq!(result.y, Some(140.0));
    }

    #[test]
    fn test_shift_left_overflow() {
        // Floating extends past left edge
        let state = make_state(-50.0, 140.0);
        let result = compute(&state, &ShiftOptions::default(), &viewport_overflow);
        // Should be clamped to x=0 (left edge)
        assert_eq!(result.x, Some(0.0));
    }

    #[test]
    fn test_shift_right_overflow() {
        // Floating extends past right edge (x=800, w=300 → right=1100 > 1024)
        let state = make_state(800.0, 140.0);
        let result = compute(&state, &ShiftOptions::default(), &viewport_overflow);
        // Should be clamped so right edge = 1024: x = 1024 - 300 = 724
        assert_eq!(result.x, Some(724.0));
    }
}
