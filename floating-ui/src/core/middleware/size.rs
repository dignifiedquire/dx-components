//! Port of `@floating-ui/core/src/middleware/size.ts` (131 lines).

use crate::types::*;
use crate::utils::*;

/// Options for the size middleware.
/// Source: size.ts `SizeOptions` (lines 13-25)
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SizeOptions {
    /// Detect overflow options.
    pub detect_overflow: DetectOverflowOptions,
}

/// Compute size middleware result.
/// Returns available width/height data for CSS custom properties.
/// Source: size.ts lines 33-131
pub fn compute(
    state: &MiddlewareState,
    options: &SizeOptions,
    detect_overflow_fn: &dyn Fn(&MiddlewareState, &DetectOverflowOptions) -> SideObject,
) -> MiddlewareReturn {
    let placement = state.placement;
    let overflow = detect_overflow_fn(state, &options.detect_overflow);

    let side = get_side(placement);
    let alignment = get_alignment(placement);
    let is_y_axis = get_side_axis(placement) == Axis::Y;
    let Rect { width, height, .. } = state.rects.floating;

    let height_side: Side = match side {
        Side::Top | Side::Bottom => side,
        _ => {
            if alignment == Some(Alignment::End) {
                Side::Top
            } else {
                Side::Bottom
            }
        }
    };

    let width_side: Side = match side {
        Side::Top | Side::Bottom => {
            // In upstream, rtl would affect this. We default to non-RTL.
            if alignment == Some(Alignment::End) {
                Side::Left
            } else {
                Side::Right
            }
        }
        _ => side,
    };

    let maximum_clipping_height = height - overflow.top - overflow.bottom;
    let maximum_clipping_width = width - overflow.left - overflow.right;

    let overflow_available_height =
        (height - overflow.get(height_side)).min(maximum_clipping_height);
    let overflow_available_width = (width - overflow.get(width_side)).min(maximum_clipping_width);

    let no_shift = state.middleware_data.shift.is_none();

    let mut available_height = overflow_available_height;
    let mut available_width = overflow_available_width;

    if let Some(ref shift_data) = state.middleware_data.shift {
        if shift_data.enabled_x {
            available_width = maximum_clipping_width;
        }
        if shift_data.enabled_y {
            available_height = maximum_clipping_height;
        }
    }

    if no_shift && alignment.is_none() {
        let x_min = overflow.left.max(0.0);
        let x_max = overflow.right.max(0.0);
        let y_min = overflow.top.max(0.0);
        let y_max = overflow.bottom.max(0.0);

        if is_y_axis {
            available_width = width
                - 2.0
                    * (if x_min != 0.0 || x_max != 0.0 {
                        x_min + x_max
                    } else {
                        overflow.left.max(overflow.right)
                    });
        } else {
            available_height = height
                - 2.0
                    * (if y_min != 0.0 || y_max != 0.0 {
                        y_min + y_max
                    } else {
                        overflow.top.max(overflow.bottom)
                    });
        }
    }

    MiddlewareReturn {
        data: MiddlewareDataUpdate::Size(SizeData {
            available_width: available_width.max(0.0),
            available_height: available_height.max(0.0),
        }),
        ..Default::default()
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
                    height: 200.0,
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
    fn test_size_available_space() {
        let state = make_state(100.0, 140.0);
        let result = compute(&state, &SizeOptions::default(), &viewport_overflow);
        match result.data {
            MiddlewareDataUpdate::Size(data) => {
                // available_height = 768 - 140 (top of floating) = 628
                // But it's min(200 - overflow.bottom, max_clip_h)
                assert!(data.available_height > 0.0);
                assert!(data.available_width > 0.0);
            }
            _ => panic!("Expected Size data"),
        }
    }
}
