//! DOM platform — port of `@floating-ui/dom`.
//!
//! Provides DOM measurement and auto-update functionality using `web-sys`.
//! Only compiled on `target_arch = "wasm32"`.

pub mod auto_update;
pub mod platform;
pub mod utils;

pub use auto_update::{auto_update, AutoUpdateOptions};
pub use platform::{
    get_bounding_client_rect, get_clipping_rect, get_element_rects, get_viewport_rect,
    make_detect_overflow_fn,
};

/// Get the document element. Convenience for callers that don't have web-sys directly.
pub fn get_document_element() -> web_sys::Element {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .document_element()
        .unwrap()
}
