//! Port of `@floating-ui/core/src/detectOverflow.ts` (119 lines).
//!
//! Computes how much the floating element overflows each side of the clipping boundary.
//! Positive = overflowing, negative = within bounds, 0 = flush.

use crate::types::*;
use crate::utils::*;

/// Detect overflow of the floating element relative to a clipping rect.
///
/// This is the simplified version that takes a pre-computed clipping rect.
/// The DOM platform is responsible for computing the clipping rect from
/// boundary/rootBoundary/element context.
///
/// Source: `detectOverflow.ts` lines 49-119 (simplified for pre-computed clipping rect)
pub fn detect_overflow(
    state: &MiddlewareState,
    clipping_rect: ClientRectObject,
    options: &DetectOverflowOptions,
) -> SideObject {
    let padding = get_padding_object(options.padding);

    // The element rect to check overflow for
    let element_rect = match options.element_context {
        ElementContext::Floating => ClientRectObject {
            x: state.x,
            y: state.y,
            width: state.rects.floating.width,
            height: state.rects.floating.height,
            top: state.y,
            left: state.x,
            right: state.x + state.rects.floating.width,
            bottom: state.y + state.rects.floating.height,
        },
        ElementContext::Reference => rect_to_client_rect(state.rects.reference),
    };

    // Source: detectOverflow.ts lines 101-118
    SideObject {
        top: clipping_rect.top - element_rect.top + padding.top,
        bottom: element_rect.bottom - clipping_rect.bottom + padding.bottom,
        left: clipping_rect.left - element_rect.left + padding.left,
        right: element_rect.right - clipping_rect.right + padding.right,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state(x: f64, y: f64, float_w: f64, float_h: f64) -> MiddlewareState {
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
                    width: float_w,
                    height: float_h,
                },
            },
            middleware_data: MiddlewareData::default(),
        }
    }

    /// Viewport: 0,0 to 1024,768
    fn viewport() -> ClientRectObject {
        rect_to_client_rect(Rect {
            x: 0.0,
            y: 0.0,
            width: 1024.0,
            height: 768.0,
        })
    }

    #[test]
    fn test_no_overflow() {
        // Floating element well within viewport
        let state = make_state(100.0, 150.0, 200.0, 100.0);
        let overflow = detect_overflow(&state, viewport(), &DetectOverflowOptions::default());
        // All negative = within bounds
        assert!(overflow.top < 0.0);
        assert!(overflow.bottom < 0.0);
        assert!(overflow.left < 0.0);
        assert!(overflow.right < 0.0);
    }

    #[test]
    fn test_overflow_bottom() {
        // Floating element extends past bottom of viewport
        let state = make_state(100.0, 700.0, 200.0, 100.0);
        let overflow = detect_overflow(&state, viewport(), &DetectOverflowOptions::default());
        // bottom = (700 + 100) - 768 = 32 (overflowing)
        assert_eq!(overflow.bottom, 32.0);
        // top = 0 - 700 = -700 (well within)
        assert_eq!(overflow.top, -700.0);
    }

    #[test]
    fn test_overflow_with_padding() {
        let state = make_state(100.0, 700.0, 200.0, 50.0);
        let options = DetectOverflowOptions {
            padding: Padding::Uniform(10.0),
            ..Default::default()
        };
        let overflow = detect_overflow(&state, viewport(), &options);
        // bottom = (700 + 50) - 768 + 10 = -8 (within, but padding reduces margin)
        assert_eq!(overflow.bottom, -8.0);
    }

    #[test]
    fn test_reference_context() {
        let state = make_state(100.0, 150.0, 200.0, 100.0);
        let options = DetectOverflowOptions {
            element_context: ElementContext::Reference,
            ..Default::default()
        };
        let overflow = detect_overflow(&state, viewport(), &options);
        // Reference is at (100, 100, 200, 40)
        // top = 0 - 100 = -100
        assert_eq!(overflow.top, -100.0);
        // right = (100+200) - 1024 = -724
        assert_eq!(overflow.right, -724.0);
    }
}
