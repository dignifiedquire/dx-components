//! AlertDialog primitive — matches `@radix-ui/react-alert-dialog`.
//!
//! A modal dialog that interrupts the user with important content and expects
//! a response. Built on top of [`DialogRoot`] but always modal, with
//! `role="alertdialog"` and no overlay-click-to-close.
//!
//! Like [`crate::dialog`], the content renders as a native `<dialog>` element
//! in the browser top layer — no portal required. See that module's docs for
//! the rationale.

use std::rc::Rc;

use dioxus::prelude::*;

use crate::dialog::{DialogCtx, DialogRoot};
use crate::presence::Presence;
use crate::scroll_lock::use_scroll_lock;
use crate::top_layer::{use_top_layer, TopLayerKind};
use crate::{use_id_or, use_unique_id};

// ---------------------------------------------------------------------------
// AlertDialogRoot
// ---------------------------------------------------------------------------

/// Props for [`AlertDialogRoot`].
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogRootProps {
    /// The controlled `open` state.
    pub open: ReadSignal<Option<bool>>,

    /// The default `open` state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// The children.
    pub children: Element,
}

/// The root of the alert dialog. Always modal.
///
/// Wraps [`DialogRoot`] with `modal: true` (matching Radix's AlertDialog
/// which omits the `modal` prop and forces it to `true`).
#[component]
pub fn AlertDialogRoot(props: AlertDialogRootProps) -> Element {
    rsx! {
        DialogRoot {
            modal: true,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogTrigger
// ---------------------------------------------------------------------------

/// Props for [`AlertDialogTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogTriggerProps {
    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children.
    pub children: Element,
}

/// A button that opens the alert dialog.
///
/// Matches Radix's `AlertDialogTrigger`.
#[component]
pub fn AlertDialogTrigger(props: AlertDialogTriggerProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open;
    let set_open = ctx.set_open;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "alert-dialog-trigger",
            "data-state": if open() { "open" } else { "closed" },
            aria_haspopup: "dialog",
            aria_expanded: open(),
            class: props.class,
            onclick: move |_| set_open.call(!open()),
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogOverlay
// ---------------------------------------------------------------------------

/// Props for [`AlertDialogOverlay`].
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogOverlayProps {
    /// The ID of the overlay element.
    pub id: ReadSignal<Option<String>>,

    /// Retained for API compatibility — Presence handles unmount on close.
    #[props(default)]
    pub force_mount: bool,

    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The backdrop overlay behind the alert dialog.
///
/// Unlike [`DialogOverlay`](crate::dialog::DialogOverlay), clicking the
/// overlay does **not** close the alert dialog (matching Radix's behavior
/// where `onPointerDownOutside` is prevented). The user must use an explicit
/// `AlertDialogCancel` or `AlertDialogAction` button.
///
/// ## Data Attributes
/// - `data-state`: `"open"` or `"closed"`.
#[component]
pub fn AlertDialogOverlay(props: AlertDialogOverlayProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open;

    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);

    rsx! {
        Presence {
            present: props.force_mount || open(),
            id: id,
            div {
                id,
                "data-slot": "alert-dialog-overlay",
                "data-state": if open() { "open" } else { "closed" },
                class: props.class,
                // No click handler — alert dialog does NOT dismiss on
                // backdrop click. The user must use Cancel or Action.
                ..props.attributes,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogContent
// ---------------------------------------------------------------------------

/// Props for [`AlertDialogContent`].
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogContentProps {
    /// The ID of the content element.
    pub id: ReadSignal<Option<String>>,

    /// Retained for API compatibility — the `<dialog>` is always mounted.
    #[props(default)]
    pub force_mount: bool,

    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children.
    pub children: Element,
}

/// The content panel of the alert dialog.
///
/// Renders as a native `<dialog>` with `role="alertdialog"` opened via
/// `showModal()`. Native ESC closing is preserved (matches Radix's
/// `onEscapeKeyDown` default). No backdrop click closes — the user must
/// use an explicit Cancel or Action button.
///
/// ## Data Attributes
/// - `data-state`: `"open"` or `"closed"`.
#[component]
pub fn AlertDialogContent(props: AlertDialogContentProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open;
    let set_open = ctx.set_open;

    // Always modal — scroll lock matches Radix's `react-remove-scroll`.
    #[allow(clippy::redundant_closure)]
    let scroll_lock_active = use_memo(move || open());
    use_scroll_lock(scroll_lock_active);

    let gen_id = use_unique_id();
    let id = use_id_or(gen_id, props.id);

    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    use_top_layer(
        mounted.into(),
        open.into(),
        set_open,
        TopLayerKind::DialogModal,
    );

    rsx! {
        dialog {
            id,
            "data-slot": "alert-dialog-content",
            "data-state": if open() { "open" } else { "closed" },
            role: "alertdialog",
            aria_modal: "true",
            aria_labelledby: ctx.title_id,
            aria_describedby: ctx.description_id,
            class: props.class,
            onmounted: move |e| mounted.set(Some(e.data())),
            // No backdrop-click handler — alert dialog requires explicit
            // Cancel or Action.
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogTitle
// ---------------------------------------------------------------------------

/// Props for [`AlertDialogTitle`].
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogTitleProps {
    /// The ID of the title element.
    pub id: ReadSignal<Option<String>>,

    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children.
    pub children: Element,
}

/// The title of the alert dialog. Sets `aria-labelledby` on the content.
///
/// Matches Radix's `AlertDialogTitle`. Renders `<h2>`.
#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    let ctx: DialogCtx = use_context();
    let id = use_id_or(ctx.title_id, props.id);

    rsx! {
        h2 {
            id,
            "data-slot": "alert-dialog-title",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogDescription
// ---------------------------------------------------------------------------

/// Props for [`AlertDialogDescription`].
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogDescriptionProps {
    /// The ID of the description element.
    pub id: ReadSignal<Option<String>>,

    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children.
    pub children: Element,
}

/// The description of the alert dialog. Sets `aria-describedby` on the content.
///
/// Matches Radix's `AlertDialogDescription`. Renders `<p>`.
#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    let ctx: DialogCtx = use_context();
    let id = use_id_or(ctx.description_id, props.id);

    rsx! {
        p {
            id,
            "data-slot": "alert-dialog-description",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogAction
// ---------------------------------------------------------------------------

/// Props for [`AlertDialogAction`].
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogActionProps {
    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children.
    pub children: Element,
}

/// An action button that closes the alert dialog.
///
/// Matches Radix's `AlertDialogAction` (wraps `DialogPrimitive.Close`).
#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    let ctx: DialogCtx = use_context();
    let set_open = ctx.set_open;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "alert-dialog-action",
            class: props.class,
            onclick: move |_| set_open.call(false),
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogCancel
// ---------------------------------------------------------------------------

/// Props for [`AlertDialogCancel`].
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogCancelProps {
    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children.
    pub children: Element,
}

/// A cancel button that closes the alert dialog.
///
/// Matches Radix's `AlertDialogCancel` (wraps `DialogPrimitive.Close`).
#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    let ctx: DialogCtx = use_context();
    let set_open = ctx.set_open;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "alert-dialog-cancel",
            class: props.class,
            onclick: move |_| set_open.call(false),
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogFooter (layout helper, matches shadcn)
// ---------------------------------------------------------------------------

/// Props for [`AlertDialogFooter`].
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogFooterProps {
    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children.
    pub children: Element,
}

/// A layout container for alert dialog action buttons.
///
/// This is a convenience component (matching shadcn's `AlertDialogFooter`)
/// for laying out `AlertDialogCancel` and `AlertDialogAction` buttons.
#[component]
pub fn AlertDialogFooter(props: AlertDialogFooterProps) -> Element {
    rsx! {
        div {
            "data-slot": "alert-dialog-footer",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

/// Backward compatibility alias.
pub use AlertDialogFooter as AlertDialogActions;
