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
//!
//! ## Differences from upstream
//!
//! - **No DOM re-parenting**: upstream calls `ReactDOM.createPortal(children,
//!   container)`, defaulting to `document.body`. Dioxus cannot move rendered
//!   nodes — hand-rolled `appendChild` re-parenting breaks event delegation —
//!   so children render where they are written, and overlays reach the browser
//!   [top layer](crate::top_layer) instead.
//! - **What that wins**: top-layer rendering escapes ancestor `overflow`,
//!   `transform`, `filter`, `opacity`, `contain`, and stacking contexts, and,
//!   unlike a portal, it keeps CSS custom properties, `@container` queries,
//!   shadow-root styles, fullscreen interactivity (an overlay portaled to
//!   `document.body` is inert while another element is fullscreen; a
//!   descendant is not), and modal-`<dialog>` ancestry intact. Much of the
//!   upstream `container` traffic exists to restore exactly these.
//! - **What that loses**: an ancestor's `display: none`,
//!   `content-visibility: hidden`, or `inert` still applies; content cannot be
//!   rendered into another document (popped-out window, iframe); and an overlay
//!   cannot be confined to a sub-region of the page. Inherited `visibility` and
//!   `pointer-events` *are* restored — [`crate::popper::PopperContent`] pins
//!   them on the positioning wrapper.
//! - **No `container` prop**: upstream's is `container?: Element |
//!   DocumentFragment | null`. Accepting one here and ignoring it would type-check
//!   while silently doing nothing, which is worse than its absence, so it is
//!   omitted deliberately.

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
