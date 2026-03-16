//! Line-by-line port of `@radix-ui/react-use-size`.
//!
//! Observes an element's border-box size using `ResizeObserver` and returns
//! a reactive signal that updates whenever the element resizes.
//!
//! ## Upstream API
//!
//! ```ts
//! function useSize(element: HTMLElement | null): { width: number; height: number } | undefined
//! ```
//!
//! - Initial size via `offsetWidth` / `offsetHeight`
//! - Continuous updates via `ResizeObserver` with `{ box: 'border-box' }`
//! - Reads `borderBoxSize.inlineSize` / `blockSize` when available
//! - Falls back to `offsetWidth` / `offsetHeight` for older browsers
//! - Returns `undefined` when element is `null`
//! - Cleanup: `resizeObserver.unobserve(element)` on unmount / element change
//!
//! ## Differences from upstream
//!
//! - **`useLayoutEffect`**: Upstream uses `@radix-ui/react-use-layout-effect`
//!   (SSR-safe wrapper). In Dioxus, `use_effect` is already SSR-safe.
//! - **Element reference**: Upstream takes `HTMLElement | null`. In Dioxus,
//!   we take `ReadSignal<Option<MountedData>>` from `onmounted`.
//! - On non-wasm targets this is a no-op that always returns `None`.

use dioxus::prelude::*;
use std::rc::Rc;

/// Element size returned by [`use_size`].
///
/// Matches upstream's `{ width: number; height: number }` return shape.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElementSize {
    /// Width in pixels (border-box).
    pub width: f64,
    /// Height in pixels (border-box).
    pub height: f64,
}

/// Observes an element's border-box size via `ResizeObserver`.
///
/// Matches upstream `useSize(element)`. Returns `None` until the element is
/// mounted, then provides continuous size updates. When the element becomes
/// `None`, resets to `None`.
///
/// Pass a `ReadSignal<Option<Rc<MountedData>>>` obtained via `onmounted`.
pub fn use_size(mounted: ReadSignal<Option<Rc<MountedData>>>) -> ReadSignal<Option<ElementSize>> {
    let size: Signal<Option<ElementSize>> = use_signal(|| None);

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        // Upstream: useLayoutEffect(() => { ... }, [element])
        // Dioxus's use_effect_with_cleanup re-runs when reactive signals
        // (mounted) change, and calls the returned cleanup before re-running.
        crate::use_effect_with_cleanup(move || {
            if let Some(md) = mounted.cloned() {
                // Get the raw DOM element from MountedData.
                let element: web_sys::HtmlElement = match md
                    .downcast::<web_sys::Element>()
                    .map(|e| e.dyn_into::<web_sys::HtmlElement>())
                {
                    Some(Ok(el)) => el,
                    _ => {
                        return Box::new(|| {}) as Box<dyn FnOnce()>;
                    }
                };

                // Upstream: provide size as early as possible
                // setSize({ width: element.offsetWidth, height: element.offsetHeight })
                size.set(Some(ElementSize {
                    width: element.offset_width() as f64,
                    height: element.offset_height() as f64,
                }));

                // Upstream: const resizeObserver = new ResizeObserver((entries) => { ... })
                let el_for_fallback = element.clone();
                let closure = Closure::wrap(Box::new(move |entries: js_sys::Array| {
                    // Upstream: if (!Array.isArray(entries)) return;
                    // js_sys::Array is always an array in Rust.

                    // Upstream: if (!entries.length) return;
                    if entries.length() == 0 {
                        return;
                    }

                    // Upstream: const entry = entries[0]!;
                    let entry: web_sys::ResizeObserverEntry = entries.get(0).unchecked_into();

                    let (width, height) =
                        // Upstream: if ('borderBoxSize' in entry)
                        if let Ok(border_box_size) =
                            js_sys::Reflect::get(&entry, &JsValue::from_str("borderBoxSize"))
                        {
                            if !border_box_size.is_undefined() && !border_box_size.is_null() {
                                // Upstream: iron out differences between browsers
                                // const borderSize = Array.isArray(borderSizeEntry)
                                //   ? borderSizeEntry[0] : borderSizeEntry;
                                let border_size = if js_sys::Array::is_array(&border_box_size) {
                                    js_sys::Array::from(&border_box_size).get(0)
                                } else {
                                    border_box_size
                                };

                                // Upstream: width = borderSize['inlineSize'];
                                //           height = borderSize['blockSize'];
                                let inline_size = js_sys::Reflect::get(
                                    &border_size,
                                    &JsValue::from_str("inlineSize"),
                                )
                                .ok()
                                .and_then(|v| v.as_f64())
                                .unwrap_or(el_for_fallback.offset_width() as f64);

                                let block_size = js_sys::Reflect::get(
                                    &border_size,
                                    &JsValue::from_str("blockSize"),
                                )
                                .ok()
                                .and_then(|v| v.as_f64())
                                .unwrap_or(el_for_fallback.offset_height() as f64);

                                (inline_size, block_size)
                            } else {
                                // Upstream: fallback for browsers that don't support borderBoxSize
                                (
                                    el_for_fallback.offset_width() as f64,
                                    el_for_fallback.offset_height() as f64,
                                )
                            }
                        } else {
                            (
                                el_for_fallback.offset_width() as f64,
                                el_for_fallback.offset_height() as f64,
                            )
                        };

                    // Upstream: setSize({ width, height })
                    size.set(Some(ElementSize { width, height }));
                }) as Box<dyn FnMut(js_sys::Array)>);

                let observer =
                    web_sys::ResizeObserver::new(closure.as_ref().unchecked_ref()).unwrap();

                // Upstream: resizeObserver.observe(element, { box: 'border-box' })
                let mut options = web_sys::ResizeObserverOptions::new();
                options.box_(web_sys::ResizeObserverBoxOptions::BorderBox);
                observer.observe_with_options(&element, &options);

                // Upstream: return () => resizeObserver.unobserve(element)
                // Move both observer and closure into cleanup so neither is
                // dropped prematurely (the closure must live as long as the
                // observer is observing).
                Box::new(move || {
                    observer.disconnect();
                    drop(closure);
                }) as Box<dyn FnOnce()>
            } else {
                // Upstream: setSize(undefined) when element becomes null
                size.set(None);
                Box::new(|| {}) as Box<dyn FnOnce()>
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = mounted;
    }

    size.into()
}
