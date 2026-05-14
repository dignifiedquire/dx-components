//! Portal primitive — no-op alias for upstream API parity.
//!
//! Radix's `@radix-ui/react-portal` uses `ReactDOM.createPortal` to teleport
//! children to `document.body`. Our overlay primitives (Dialog, AlertDialog,
//! Popover, DropdownMenu, ContextMenu, Menubar, Select, Combobox, Tooltip,
//! HoverCard, Toast) no longer need that pattern — they render in the
//! browser top layer via the `popover` attribute / `<dialog>` element (see
//! [`crate::top_layer`]). Top-layer rendering escapes ancestor `overflow`,
//! `transform`, `filter`, and stacking-contexts without DOM re-parenting.
//!
//! [`Portal`] is kept as a no-op pass-through so upstream code calling
//! `Portal { ... }` (or `MenuPortal { ... }`, etc.) continues to compile.

use dioxus::prelude::*;

/// Props for [`Portal`].
#[derive(Props, Clone, PartialEq)]
pub struct PortalProps {
    /// Children to render.
    pub children: Element,
}

/// No-op pass-through that matches the `@radix-ui/react-portal` API.
///
/// Renders children inline. Top-layer escape is handled by each overlay
/// primitive via the [`popover`](crate::top_layer) attribute or the
/// `<dialog>` element — placing or omitting this component does not
/// affect rendering location.
#[component]
pub fn Portal(props: PortalProps) -> Element {
    rsx! {
        {props.children}
    }
}

/// Upstream alias for [`Portal`].
pub use Portal as Root;
