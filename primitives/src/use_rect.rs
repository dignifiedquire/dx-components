//! Line-by-line port of `@radix-ui/react-use-rect` + `@radix-ui/rect`.
//!
//! Observes an element's `getBoundingClientRect()` over time using a shared
//! `requestAnimationFrame` polling loop, and returns a reactive signal.
//!
//! ## Upstream API
//!
//! ```ts
//! // @radix-ui/react-use-rect
//! function useRect(measurable: Measurable | null): DOMRect | undefined
//!
//! // @radix-ui/rect
//! function observeElementRect(element: Measurable, callback: (rect: DOMRect) => void): () => void
//! ```
//!
//! - Shared rAF loop across all observers (starts on first, stops on last)
//! - Only fires callbacks when rect actually changes (`rectEquals`)
//! - Multiple callbacks per element (deduplicates observation)
//! - Cleanup: removes callback, stops loop when no elements remain
//!
//! ## Differences from upstream
//!
//! - **Element reference**: Upstream takes `Measurable` (any object with
//!   `getBoundingClientRect()`). In Dioxus, we take `ReadSignal<Option<MountedData>>`
//!   from `onmounted` and extract the DOM element on wasm.
//! - **Shared loop**: Upstream uses a global `Map` and single rAF loop. We use
//!   a `thread_local!` `RefCell<HashMap>` (WASM is single-threaded).
//! - On non-wasm targets this is a no-op that always returns `None`.

use dioxus::prelude::*;
use std::rc::Rc;

/// A rect matching the shape of `DOMRect` from `getBoundingClientRect()`.
///
/// Used instead of `web_sys::DomRect` so the type is available on all targets.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    /// X position (same as `left` for standard boxes).
    pub x: f64,
    /// Y position (same as `top` for standard boxes).
    pub y: f64,
    /// Width in pixels.
    pub width: f64,
    /// Height in pixels.
    pub height: f64,
    /// Distance from top of viewport.
    pub top: f64,
    /// Distance from right of viewport.
    pub right: f64,
    /// Distance from bottom of viewport.
    pub bottom: f64,
    /// Distance from left of viewport.
    pub left: f64,
}

/// Observes an element's bounding client rect over time via a shared
/// `requestAnimationFrame` polling loop.
///
/// Matches upstream `useRect(measurable)`. Returns `None` until the element
/// is mounted, then provides continuous rect updates. Resets to `None` when
/// the element is removed.
///
/// Pass a `ReadSignal<Option<Rc<MountedData>>>` obtained via `onmounted`.
pub fn use_rect(mounted: ReadSignal<Option<Rc<MountedData>>>) -> ReadSignal<Option<Rect>> {
    let rect: Signal<Option<Rect>> = use_signal(|| None);

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;

        crate::use_effect_with_cleanup(move || {
            if let Some(md) = mounted.cloned() {
                // Get the raw DOM element.
                let element: web_sys::Element = match md.downcast::<web_sys::Element>() {
                    Some(el) => el.clone(),
                    _ => return Box::new(|| {}) as Box<dyn FnOnce()>,
                };

                // Upstream: const unobserve = observeElementRect(measurable, setRect)
                let mut rect_sig = rect;
                let unobserve = observe_element_rect(element, move |r| {
                    rect_sig.set(Some(r));
                });

                // Upstream: return () => { setRect(undefined); unobserve(); }
                Box::new(move || {
                    rect_sig.set(None);
                    unobserve();
                }) as Box<dyn FnOnce()>
            } else {
                Box::new(|| {}) as Box<dyn FnOnce()>
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = mounted;
    }

    rect.into()
}

// =========================================================================
// observe_element_rect — port of @radix-ui/rect
// =========================================================================

#[cfg(target_arch = "wasm32")]
mod observe {
    use super::Rect;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    type CallbackFn = Box<dyn FnMut(Rect)>;

    struct ObservedData {
        rect: Rect,
        callbacks: Vec<CallbackFn>,
    }

    // Element identity key: we use the raw pointer value of the JS object
    // (via `JsValue::as_f64` of a WeakRef or direct identity). Since
    // web_sys::Element is !Hash, we use its unchecked_ref pointer.
    type ElementKey = usize;

    fn element_key(el: &web_sys::Element) -> ElementKey {
        let js: &JsValue = el.as_ref();
        // Use the JsValue's internal pointer bits as a unique key.
        // This is stable for the lifetime of the JS object.
        js.as_f64().map(|f| f as usize).unwrap_or_else(|| {
            // Fallback: use bit pattern of the reference
            let ptr = js as *const JsValue as usize;
            ptr
        })
    }

    struct LoopState {
        observed: HashMap<ElementKey, (web_sys::Element, ObservedData)>,
        raf_id: Option<i32>,
        /// The rAF closure must be kept alive.
        _raf_closure: Option<Closure<dyn FnMut()>>,
    }

    thread_local! {
        static STATE: RefCell<LoopState> = RefCell::new(LoopState {
            observed: HashMap::new(),
            raf_id: None,
            _raf_closure: None,
        });
    }

    fn dom_rect_to_rect(r: web_sys::DomRect) -> Rect {
        Rect {
            x: r.x(),
            y: r.y(),
            width: r.width(),
            height: r.height(),
            top: r.top(),
            right: r.right(),
            bottom: r.bottom(),
            left: r.left(),
        }
    }

    fn rect_equals(a: &Rect, b: &Rect) -> bool {
        a.width == b.width
            && a.height == b.height
            && a.top == b.top
            && a.right == b.right
            && a.bottom == b.bottom
            && a.left == b.left
    }

    fn run_loop() {
        STATE.with(|state| {
            let mut s = state.borrow_mut();

            // Process all DOM reads first (getBoundingClientRect).
            let mut changed_keys: Vec<ElementKey> = Vec::new();

            for (key, (element, data)) in s.observed.iter_mut() {
                let new_rect = dom_rect_to_rect(element.get_bounding_client_rect());
                if !rect_equals(&data.rect, &new_rect) {
                    data.rect = new_rect;
                    changed_keys.push(*key);
                }
            }

            // Group DOM writes (callbacks) after DOM reads.
            for key in changed_keys {
                if let Some((_el, data)) = s.observed.get_mut(&key) {
                    let rect = data.rect;
                    for cb in data.callbacks.iter_mut() {
                        cb(rect);
                    }
                }
            }

            // Schedule next frame.
            schedule_raf(&mut s);
        });
    }

    fn schedule_raf(s: &mut LoopState) {
        if s.observed.is_empty() {
            s.raf_id = None;
            s._raf_closure = None;
            return;
        }

        let closure = Closure::once(|| run_loop());
        let window = web_sys::window().unwrap();
        let id = window
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();
        s.raf_id = Some(id);
        s._raf_closure = Some(closure);
    }

    /// Observe an element's bounding client rect via a shared rAF polling loop.
    ///
    /// Returns an unobserve function. Calling it removes this callback; when
    /// no callbacks remain for the element, the element is removed from
    /// observation. When no elements remain, the rAF loop stops.
    pub(super) fn observe_element_rect(
        element: web_sys::Element,
        callback: impl FnMut(Rect) + 'static,
    ) -> impl FnOnce() {
        let key = element_key(&element);
        let callback_id = STATE.with(|state| {
            let mut s = state.borrow_mut();

            if let Some((_el, data)) = s.observed.get_mut(&key) {
                // Element already observed — add callback and fire immediately.
                let cb_idx = data.callbacks.len();
                let initial_rect = dom_rect_to_rect(element.get_bounding_client_rect());
                let mut cb = Box::new(callback) as CallbackFn;
                cb(initial_rect);
                data.callbacks.push(cb);
                cb_idx
            } else {
                // First observer for this element.
                let initial_rect = Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 0.0,
                    height: 0.0,
                    top: 0.0,
                    right: 0.0,
                    bottom: 0.0,
                    left: 0.0,
                };
                let was_empty = s.observed.is_empty();
                s.observed.insert(
                    key,
                    (
                        element,
                        ObservedData {
                            rect: initial_rect,
                            callbacks: vec![Box::new(callback)],
                        },
                    ),
                );
                if was_empty {
                    schedule_raf(&mut s);
                }
                0
            }
        });

        // Return unobserve closure.
        move || {
            STATE.with(|state| {
                let mut s = state.borrow_mut();
                if let Some((_el, data)) = s.observed.get_mut(&key) {
                    if callback_id < data.callbacks.len() {
                        data.callbacks.remove(callback_id);
                    }
                    if data.callbacks.is_empty() {
                        s.observed.remove(&key);
                        if s.observed.is_empty() {
                            if let Some(id) = s.raf_id.take() {
                                let _ = web_sys::window().unwrap().cancel_animation_frame(id);
                            }
                            s._raf_closure = None;
                        }
                    }
                }
            });
        }
    }
}

#[cfg(target_arch = "wasm32")]
use observe::observe_element_rect;
