//! DOM Platform — port of `@floating-ui/dom/src/platform/`.
//!
//! Provides the `detect_overflow_fn` for `compute_position` by computing
//! clipping rects from the actual DOM.

use super::utils as dom_utils;
use crate::types::*;
use crate::utils::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

/// Get the viewport rect for strategy: fixed.
/// Source: dom/src/utils/getViewportRect.ts (simplified)
pub fn get_viewport_rect() -> Rect {
    let win = web_sys::window().expect("no window");
    let width = win
        .inner_width()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(1024.0);
    let height = win
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(768.0);

    // Check for visualViewport (pinch zoom on mobile)
    if let Some(vv) = win.visual_viewport() {
        let vv_width = vv.width();
        let vv_height = vv.height();
        if vv_width > 0.0 && vv_height > 0.0 {
            return Rect {
                x: vv.offset_left(),
                y: vv.offset_top(),
                width: vv_width,
                height: vv_height,
            };
        }
    }

    Rect {
        x: 0.0,
        y: 0.0,
        width,
        height,
    }
}

/// Get element bounding client rect.
/// Source: dom/src/utils/getBoundingClientRect.ts (simplified for strategy: fixed)
pub fn get_bounding_client_rect(element: &Element) -> ClientRectObject {
    let rect = element.get_bounding_client_rect();
    ClientRectObject {
        x: rect.x(),
        y: rect.y(),
        width: rect.width(),
        height: rect.height(),
        top: rect.top(),
        right: rect.right(),
        bottom: rect.bottom(),
        left: rect.left(),
    }
}

/// Get the clipping rect — the most restrictive visible area.
/// Source: dom/src/platform/getClippingRect.ts (simplified)
///
/// For `strategy: fixed` with default boundary, this intersects:
/// 1. All overflow ancestors' inner rects
/// 2. The viewport rect
pub fn get_clipping_rect(element: &Element, _strategy: Strategy) -> Rect {
    // For strategy: fixed, the base clipping rect is the viewport
    let viewport = get_viewport_rect();
    let mut clip = rect_to_client_rect(viewport);

    // If top layer element, only clip to viewport
    if dom_utils::is_top_layer(element) {
        return viewport;
    }

    // Walk overflow ancestors and intersect their inner rects
    let ancestors = dom_utils::get_overflow_ancestors(element.unchecked_ref());
    for ancestor in &ancestors {
        if let Some(el) = ancestor.dyn_ref::<Element>() {
            if dom_utils::is_overflow_element(el) {
                let ancestor_rect = get_inner_bounding_client_rect(el);
                let ancestor_clip = rect_to_client_rect(ancestor_rect);
                clip = ClientRectObject {
                    top: clip.top.max(ancestor_clip.top),
                    right: clip.right.min(ancestor_clip.right),
                    bottom: clip.bottom.min(ancestor_clip.bottom),
                    left: clip.left.max(ancestor_clip.left),
                    x: clip.left.max(ancestor_clip.left),
                    y: clip.top.max(ancestor_clip.top),
                    width: clip.right.min(ancestor_clip.right) - clip.left.max(ancestor_clip.left),
                    height: clip.bottom.min(ancestor_clip.bottom) - clip.top.max(ancestor_clip.top),
                };
            }
        }
    }

    Rect {
        x: clip.left,
        y: clip.top,
        width: (clip.right - clip.left).max(0.0),
        height: (clip.bottom - clip.top).max(0.0),
    }
}

/// Get inner client rect (subtracting scrollbars).
/// Source: dom/src/platform/getClippingRect.ts `getInnerBoundingClientRect`
fn get_inner_bounding_client_rect(element: &Element) -> Rect {
    let rect = element.get_bounding_client_rect();
    let html_el = element.dyn_ref::<web_sys::HtmlElement>();
    let (client_top, client_left) = html_el
        .map(|el| (el.client_top() as f64, el.client_left() as f64))
        .unwrap_or((0.0, 0.0));

    let top = rect.top() + client_top;
    let left = rect.left() + client_left;

    // Use clientWidth/clientHeight which exclude scrollbars
    let width = html_el
        .map(|el| el.client_width() as f64)
        .unwrap_or(rect.width());
    let height = html_el
        .map(|el| el.client_height() as f64)
        .unwrap_or(rect.height());

    Rect {
        x: left,
        y: top,
        width,
        height,
    }
}

/// Get element rects for compute_position.
/// Source: dom/src/platform/getElementRects.ts
///
/// For strategy: fixed, reference rect = getBoundingClientRect().
/// Floating rect has x=0, y=0 with measured dimensions.
pub fn get_element_rects(reference: &Element, floating: &Element) -> ElementRects {
    let ref_rect = reference.get_bounding_client_rect();
    let float_rect = floating.get_bounding_client_rect();

    ElementRects {
        reference: Rect {
            x: ref_rect.x(),
            y: ref_rect.y(),
            width: ref_rect.width(),
            height: ref_rect.height(),
        },
        floating: Rect {
            x: 0.0,
            y: 0.0,
            width: float_rect.width(),
            height: float_rect.height(),
        },
    }
}

/// Create a detect_overflow function bound to specific DOM elements.
///
/// This is what the core's `compute_position` uses via the `detect_overflow_fn` parameter.
/// It captures the floating element and strategy to compute the clipping rect.
pub fn make_detect_overflow_fn(
    reference_element: Element,
    floating_element: Element,
    strategy: Strategy,
) -> impl Fn(&MiddlewareState, &DetectOverflowOptions) -> SideObject {
    move |state: &MiddlewareState, options: &DetectOverflowOptions| {
        let padding = get_padding_object(options.padding);

        // alt_boundary: use the other element's clipping context.
        // Source: detectOverflow.ts line 64-65
        let clip_element = if options.alt_boundary {
            match options.element_context {
                ElementContext::Floating => &reference_element,
                ElementContext::Reference => &floating_element,
            }
        } else {
            match options.element_context {
                ElementContext::Floating => &floating_element,
                ElementContext::Reference => &reference_element,
            }
        };
        let clipping_rect = get_clipping_rect(clip_element, strategy);
        let clip = rect_to_client_rect(clipping_rect);

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

        SideObject {
            top: clip.top - element_rect.top + padding.top,
            bottom: element_rect.bottom - clip.bottom + padding.bottom,
            left: clip.left - element_rect.left + padding.left,
            right: element_rect.right - clip.right + padding.right,
        }
    }
}
