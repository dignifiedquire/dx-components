//! Port of `@floating-ui/core/src/middleware/hide.ts` (77 lines).

use crate::types::*;

/// Options for the hide middleware.
/// Source: hide.ts `HideOptions` (lines 20-25)
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HideOptions {
    /// Strategy: check if reference is hidden or floating escaped.
    pub strategy: HideStrategy,
    /// Detect overflow options.
    pub detect_overflow: DetectOverflowOptions,
}

/// Hide detection strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HideStrategy {
    /// Check if the reference element is fully clipped.
    #[default]
    ReferenceHidden,
    /// Check if the floating element has escaped the boundary.
    Escaped,
}

/// Source: hide.ts lines 7-13
fn get_side_offsets(overflow: SideObject, rect: Rect) -> SideObject {
    SideObject {
        top: overflow.top - rect.height,
        right: overflow.right - rect.width,
        bottom: overflow.bottom - rect.height,
        left: overflow.left - rect.width,
    }
}

/// Source: hide.ts lines 15-18
fn is_any_side_fully_clipped(overflow: SideObject) -> bool {
    [overflow.top, overflow.right, overflow.bottom, overflow.left]
        .iter()
        .any(|&v| v >= 0.0)
}

/// Compute hide middleware result.
/// Source: hide.ts lines 32-77
pub fn compute(
    state: &MiddlewareState,
    options: &HideOptions,
    detect_overflow_fn: &dyn Fn(&MiddlewareState, &DetectOverflowOptions) -> SideObject,
) -> MiddlewareReturn {
    match options.strategy {
        HideStrategy::ReferenceHidden => {
            let overflow_opts = DetectOverflowOptions {
                element_context: ElementContext::Reference,
                ..options.detect_overflow.clone()
            };
            let overflow = detect_overflow_fn(state, &overflow_opts);
            let offsets = get_side_offsets(overflow, state.rects.reference);

            MiddlewareReturn {
                data: MiddlewareDataUpdate::Hide(HideData {
                    reference_hidden: is_any_side_fully_clipped(offsets),
                    reference_hidden_offsets: Some(offsets),
                    ..Default::default()
                }),
                ..Default::default()
            }
        }
        HideStrategy::Escaped => {
            let overflow_opts = DetectOverflowOptions {
                alt_boundary: true,
                ..options.detect_overflow.clone()
            };
            let overflow = detect_overflow_fn(state, &overflow_opts);
            let floating_rect = Rect {
                x: state.x,
                y: state.y,
                width: state.rects.floating.width,
                height: state.rects.floating.height,
            };
            let offsets = get_side_offsets(overflow, floating_rect);

            MiddlewareReturn {
                data: MiddlewareDataUpdate::Hide(HideData {
                    escaped: is_any_side_fully_clipped(offsets),
                    escaped_offsets: Some(offsets),
                    ..Default::default()
                }),
                ..Default::default()
            }
        }
    }
}
