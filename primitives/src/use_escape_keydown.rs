//! Line-by-line port of `@radix-ui/react-use-escape-keydown`.
//!
//! Listens for when the escape key is pressed on the document in the capture
//! phase. Calls the provided callback with a `web_sys::KeyboardEvent` when
//! the Escape key is detected.
//!
//! ## Differences from upstream
//!
//! - **`useCallbackRef`**: Not needed in Dioxus — `Callback` is already
//!   referentially stable across renders.

/// Type alias for the owner document parameter.
/// On wasm, this is `web_sys::Document`. On other targets, a no-op placeholder.
#[cfg(target_arch = "wasm32")]
pub type OwnerDocument = web_sys::Document;

/// Type alias for the owner document parameter.
/// On non-wasm targets, this is a no-op placeholder.
#[cfg(not(target_arch = "wasm32"))]
pub type OwnerDocument = ();

/// Listens for Escape keydown events on the given document (capture phase).
///
/// Matches upstream `useEscapeKeydown(onEscapeKeyDown, ownerDocument)`.
///
/// The `owner_document` parameter matches upstream's second argument
/// (defaults to `globalThis?.document`). Pass `None` to use the default
/// document.
///
/// The callback receives the native keyboard event so the consumer can call
/// `event.prevent_default()` if needed — matching upstream's pattern where
/// `DismissableLayer` calls `event.preventDefault()` to prevent further
/// handlers from acting on the same Escape press.
///
/// On non-wasm targets this is a no-op.
pub fn use_escape_keydown(
    on_escape_key_down: impl Fn(EscapeKeydownEvent) + 'static,
    #[allow(unused)] owner_document: Option<OwnerDocument>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        use std::rc::Rc;

        // Wrap in Rc so the callback can be shared between the FnMut
        // effect closure and the inner FnMut event listener closure.
        let callback = Rc::new(on_escape_key_down);

        // Wrap in Cell to allow moving out of FnMut closure (which may be
        // called multiple times by Dioxus, though only once matters here).
        let owner_doc_cell = std::cell::Cell::new(owner_document);

        crate::use_effect_with_cleanup(move || {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;

            // Upstream: ownerDocument = globalThis?.document
            let document = owner_doc_cell.take().unwrap_or_else(|| {
                web_sys::window()
                    .and_then(|w| w.document())
                    .expect("document")
            });

            // Upstream:
            //   const handleKeyDown = (event: KeyboardEvent) => {
            //     if (event.key === 'Escape') { onEscapeKeyDown(event); }
            //   };
            let cb = callback.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                if event.key() == "Escape" {
                    cb(EscapeKeydownEvent { inner: event });
                }
            }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

            // Upstream: ownerDocument.addEventListener('keydown', handleKeyDown, { capture: true })
            let mut opts = web_sys::AddEventListenerOptions::new();
            opts.capture(true);
            let _ = document.add_event_listener_with_callback_and_add_event_listener_options(
                "keydown",
                closure.as_ref().unchecked_ref(),
                &opts,
            );

            let closure_fn = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();
            let _prevent_drop = closure;

            // Cleanup: ownerDocument.removeEventListener('keydown', handleKeyDown, { capture: true })
            Box::new(move || {
                let mut opts = web_sys::EventListenerOptions::new();
                opts.capture(true);
                let _ = document.remove_event_listener_with_callback_and_event_listener_options(
                    "keydown",
                    &closure_fn,
                    &opts,
                );
                drop(_prevent_drop);
            }) as Box<dyn FnOnce()>
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = on_escape_key_down;
    }
}

/// Wrapper around a keyboard event from the escape keydown listener.
///
/// Provides `prevent_default()` matching upstream's pattern.
pub struct EscapeKeydownEvent {
    #[cfg(target_arch = "wasm32")]
    inner: web_sys::KeyboardEvent,
}

impl EscapeKeydownEvent {
    /// Prevent the default browser action for this key event.
    pub fn prevent_default(&self) {
        #[cfg(target_arch = "wasm32")]
        self.inner.prevent_default();
    }
}
