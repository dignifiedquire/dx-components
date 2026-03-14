//! Port of `@floating-ui/core/src/computePosition.ts` (107 lines).

use crate::core::compute_coords::compute_coords_from_placement;
use crate::core::middleware::Middleware;
use crate::types::*;

/// Maximum number of resets before bailing (prevents infinite loops).
/// Source: computePosition.ts line 11
const MAX_RESET_COUNT: usize = 50;

/// Compute the (x, y) coordinates for positioning the floating element
/// next to the reference element.
///
/// Source: computePosition.ts lines 20-107
pub fn compute_position(
    rects: ElementRects,
    placement: Placement,
    strategy: Strategy,
    middleware: &[Middleware],
    detect_overflow_fn: &dyn Fn(&MiddlewareState, &DetectOverflowOptions) -> SideObject,
) -> ComputePositionReturn {
    let initial_placement = placement;

    let mut coords = compute_coords_from_placement(&rects, placement);
    let mut stateful_placement = placement;
    let mut reset_count: usize = 0;
    let mut middleware_data = MiddlewareData::default();

    let mut i: usize = 0;
    while i < middleware.len() {
        let mw = &middleware[i];

        let state = MiddlewareState {
            x: coords.x,
            y: coords.y,
            initial_placement,
            placement: stateful_placement,
            strategy,
            rects,
            middleware_data: middleware_data.clone(),
        };

        let result = mw.compute(&state, detect_overflow_fn);

        // Apply position updates
        if let Some(x) = result.x {
            coords.x = x;
        }
        if let Some(y) = result.y {
            coords.y = y;
        }

        // Merge middleware data
        middleware_data.merge(&result.data);

        // Handle reset
        match result.reset {
            Reset::None => {
                i += 1;
            }
            _ if reset_count < MAX_RESET_COUNT => {
                reset_count += 1;

                let should_recompute_coords = match result.reset {
                    Reset::WithPlacement(p) => {
                        stateful_placement = p;
                        true // Placement changed → recompute initial coords
                    }
                    Reset::WithRects => {
                        // Upstream would re-measure via platform.getElementRects.
                        // Since we use pre-measured rects, recompute from current placement.
                        true
                    }
                    Reset::Simple => {
                        // reset: true — upstream only restarts the loop, does NOT
                        // recompute coords. Source: computePosition.ts line 96: i = -1
                        false
                    }
                    Reset::None => unreachable!(),
                };

                if should_recompute_coords {
                    coords = compute_coords_from_placement(&rects, stateful_placement);
                }
                i = 0; // Restart the middleware chain
            }
            _ => {
                // Max resets exceeded — move on
                i += 1;
            }
        }
    }

    ComputePositionReturn {
        x: coords.x,
        y: coords.y,
        placement: stateful_placement,
        strategy,
        middleware_data,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::middleware::*;

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
    fn test_basic_bottom_placement() {
        let rects = ElementRects {
            reference: Rect {
                x: 100.0,
                y: 100.0,
                width: 200.0,
                height: 40.0,
            },
            floating: Rect {
                x: 0.0,
                y: 0.0,
                width: 150.0,
                height: 80.0,
            },
        };

        let result = compute_position(
            rects,
            Placement::Bottom,
            Strategy::Fixed,
            &[],
            &viewport_overflow,
        );

        // x = 100 + 200/2 - 150/2 = 125
        assert_eq!(result.x, 125.0);
        // y = 100 + 40 = 140
        assert_eq!(result.y, 140.0);
        assert_eq!(result.placement, Placement::Bottom);
    }

    #[test]
    fn test_with_offset() {
        let rects = ElementRects {
            reference: Rect {
                x: 100.0,
                y: 100.0,
                width: 200.0,
                height: 40.0,
            },
            floating: Rect {
                x: 0.0,
                y: 0.0,
                width: 150.0,
                height: 80.0,
            },
        };

        let middleware = vec![Middleware::Offset(OffsetOptions {
            main_axis: 8.0,
            ..Default::default()
        })];

        let result = compute_position(
            rects,
            Placement::Bottom,
            Strategy::Fixed,
            &middleware,
            &viewport_overflow,
        );

        assert_eq!(result.x, 125.0);
        // y = 140 + 8 = 148
        assert_eq!(result.y, 148.0);
    }

    #[test]
    fn test_flip_at_bottom_edge() {
        let rects = ElementRects {
            reference: Rect {
                x: 100.0,
                y: 680.0,
                width: 200.0,
                height: 40.0,
            },
            floating: Rect {
                x: 0.0,
                y: 0.0,
                width: 150.0,
                height: 100.0,
            },
        };

        let middleware = vec![Middleware::Flip(FlipOptions::default())];

        let result = compute_position(
            rects,
            Placement::Bottom,
            Strategy::Fixed,
            &middleware,
            &viewport_overflow,
        );

        // Reference at y=680, height=40, bottom=720
        // Floating height=100, would go to y=720+0=720, bottom=820 > 768 → overflow
        // Should flip to top: y = 680 - 100 = 580
        assert_eq!(result.placement, Placement::Top);
        assert_eq!(result.y, 580.0);
    }

    #[test]
    fn test_offset_then_shift() {
        let rects = ElementRects {
            reference: Rect {
                x: 900.0,
                y: 100.0,
                width: 100.0,
                height: 40.0,
            },
            floating: Rect {
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 80.0,
            },
        };

        let middleware = vec![
            Middleware::Offset(OffsetOptions {
                main_axis: 4.0,
                ..Default::default()
            }),
            Middleware::Shift(ShiftOptions::default()),
        ];

        let result = compute_position(
            rects,
            Placement::Bottom,
            Strategy::Fixed,
            &middleware,
            &viewport_overflow,
        );

        // Without shift: x = 900 + 50 - 100 = 850, right edge = 1050 > 1024
        // Shift should clamp to x = 1024 - 200 = 824
        assert_eq!(result.x, 824.0);
        // y = 100 + 40 + 4 = 144
        assert_eq!(result.y, 144.0);
    }
}
