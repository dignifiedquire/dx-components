//! Top-layer rendering — `popover` attribute and `<dialog>` element.
//!
//! Replaces React's `ReactDOM.createPortal(content, document.body)` pattern.
//! The browser renders elements in the [top layer] — a separate rendering
//! surface above the document that escapes ancestor `overflow`, `z-index`,
//! `transform`, `filter`, `contain`, and stacking contexts. The element
//! stays where Dioxus mounted it in the DOM tree, so context propagation,
//! event delegation, and the diff engine all work normally.
//!
//! Two APIs reach the top layer:
//!
//! - **Popover API** (`popover="auto"` or `popover="manual"` + `showPopover()`/
//!   `hidePopover()`): for non-modal overlays — popover, tooltip, menu, select.
//!   `auto` adds light-dismiss (ESC, click-outside); `manual` requires
//!   explicit hide.
//! - **`<dialog>` element** + `showModal()`/`close()`: for modal overlays —
//!   provides native focus trap and inert backdrop.
//!
//! [top layer]: https://developer.mozilla.org/en-US/docs/Glossary/Top_layer
//!
//! [`use_top_layer`] drives the show/hide methods from an `open` `Signal<bool>`,
//! and subscribes to the element's `toggle` event (popover) or `close` event
//! (dialog) so light-dismiss / ESC keeps the signal in sync.

use dioxus::prelude::*;
use std::rc::Rc;

/// Top-layer presentation kind for [`use_top_layer`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopLayerKind {
    /// `popover="auto"` — light-dismisses on ESC and click-outside.
    /// Only one auto popover open at a time globally, unless nested via
    /// the `popovertarget` attribute on an ancestor.
    PopoverAuto,
    /// `popover="manual"` — must be hidden explicitly. No light-dismiss.
    /// Multiple manual popovers can be open simultaneously.
    PopoverManual,
    /// `<dialog>` opened with `showModal()` — modal, native focus trap,
    /// inert backdrop. The mounted element must be a `<dialog>`.
    DialogModal,
}

/// Drive a top-layer element from an `open` `Signal<bool>`.
///
/// The caller must:
/// 1. Track the element via a signal:
///    `let mounted = use_signal(|| None::<Rc<MountedData>>);`
/// 2. Attach `onmounted: move |evt| mounted.set(Some(evt.data()))` and
///    the matching attribute on rsx — either `popover: "auto" | "manual"`
///    on a regular element, or use a `dialog { ... }` element for
///    [`TopLayerKind::DialogModal`].
/// 3. Pass that signal plus the `open` signal and kind to this hook.
///
/// The hook calls `show_popover()`/`show_modal()` when `open` flips to
/// `true`, and `hide_popover()`/`close()` when it flips to `false`. It
/// also subscribes to the element's `toggle` event (popover) or `close`
/// event (dialog) and writes back into `open` on browser-initiated
/// state changes (light-dismiss, ESC, cancel), keeping the signal in sync.
///
/// On non-wasm targets this is a no-op.
pub fn use_top_layer(
    mounted: ReadSignal<Option<Rc<MountedData>>>,
    open: Signal<bool>,
    kind: TopLayerKind,
) {
    #[cfg(target_arch = "wasm32")]
    {
        wasm::use_top_layer_impl(mounted, open, kind);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (mounted, open, kind);
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    pub(super) fn use_top_layer_impl(
        mounted: ReadSignal<Option<Rc<MountedData>>>,
        open: Signal<bool>,
        kind: TopLayerKind,
    ) {
        // Effect 1 — wire the browser → signal sync listener once per
        // mounted element. Re-runs (with cleanup) when `mounted` changes.
        let mut open_for_listener = open;
        crate::use_effect_with_cleanup(move || -> Box<dyn FnOnce()> {
            let Some(md) = mounted.cloned() else {
                return Box::new(|| {});
            };
            let Some(element) = element_from_mounted(&md) else {
                return Box::new(|| {});
            };

            let event_name = match kind {
                TopLayerKind::DialogModal => "close",
                TopLayerKind::PopoverAuto | TopLayerKind::PopoverManual => "toggle",
            };

            let listener = Closure::wrap(Box::new(move |evt: web_sys::Event| {
                let new_open = match kind {
                    // `close` fires only when the dialog is closing.
                    TopLayerKind::DialogModal => false,
                    _ => evt
                        .dyn_ref::<web_sys::ToggleEvent>()
                        .map(|t| t.new_state() == "open")
                        .unwrap_or(false),
                };
                if *open_for_listener.peek() != new_open {
                    open_for_listener.set(new_open);
                }
            }) as Box<dyn FnMut(web_sys::Event)>);

            let _ = element.add_event_listener_with_callback(
                event_name,
                listener.as_ref().unchecked_ref(),
            );

            let element_for_cleanup = element.clone();
            Box::new(move || {
                let _ = element_for_cleanup.remove_event_listener_with_callback(
                    event_name,
                    listener.as_ref().unchecked_ref(),
                );
                drop(listener);
            })
        });

        // Effect 2 — when `open` (or `mounted`) changes, dispatch the
        // appropriate show/hide method on the underlying DOM element.
        // Browser methods throw if called in the wrong state, so we check
        // current state before dispatching.
        use_effect(move || {
            let want_open = open();
            let Some(md) = mounted.cloned() else { return };
            let Some(element) = element_from_mounted(&md) else { return };
            apply_open_state(&element, want_open, kind);
        });
    }

    fn element_from_mounted(md: &Rc<MountedData>) -> Option<web_sys::HtmlElement> {
        md.downcast::<web_sys::Element>()
            .and_then(|e| e.clone().dyn_into::<web_sys::HtmlElement>().ok())
    }

    fn apply_open_state(element: &web_sys::HtmlElement, want_open: bool, kind: TopLayerKind) {
        match kind {
            TopLayerKind::DialogModal => {
                let dialog: &web_sys::HtmlDialogElement = element.unchecked_ref();
                let is_open = dialog.open();
                match (want_open, is_open) {
                    (true, false) => {
                        let _ = dialog.show_modal();
                    }
                    (false, true) => dialog.close(),
                    _ => {}
                }
            }
            TopLayerKind::PopoverAuto | TopLayerKind::PopoverManual => {
                // `:popover-open` is the spec-defined way to check popover state.
                let is_open = element.matches(":popover-open").unwrap_or(false);
                match (want_open, is_open) {
                    (true, false) => {
                        let _ = element.show_popover();
                    }
                    (false, true) => {
                        let _ = element.hide_popover();
                    }
                    _ => {}
                }
            }
        }
    }
}
