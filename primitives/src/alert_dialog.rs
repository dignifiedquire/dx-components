//! AlertDialog primitive — matches `@radix-ui/react-alert-dialog`.
//!
//! A modal dialog that interrupts the user with important content and expects
//! a response. Built on top of [`DialogRoot`] but always modal, with
//! `role="alertdialog"` and no overlay-click-to-close.

use dioxus::prelude::*;

use crate::aria_hidden::use_aria_hidden;
use crate::dialog::{DialogCtx, DialogRoot};
use crate::focus_scope::FocusScope;
use crate::portal::Portal;
use crate::presence::Presence;
use crate::scroll_lock::use_scroll_lock;
use crate::use_global_escape_listener;
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
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::alert_dialog::*;
/// rsx! {
///     AlertDialogRoot {
///         AlertDialogTrigger { "Delete" }
///         AlertDialogOverlay {}
///         AlertDialogContent {
///             AlertDialogTitle { "Are you sure?" }
///             AlertDialogDescription { "This cannot be undone." }
///             AlertDialogCancel { "Cancel" }
///             AlertDialogAction { "Delete" }
///         }
///     }
/// };
/// ```
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
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::alert_dialog::*;
/// rsx! {
///     AlertDialogRoot {
///         AlertDialogTrigger { "Delete Item" }
///         AlertDialogOverlay {}
///         AlertDialogContent {
///             AlertDialogTitle { "Confirm" }
///             AlertDialogDescription { "Are you sure?" }
///             AlertDialogCancel { "No" }
///             AlertDialogAction { "Yes" }
///         }
///     }
/// };
/// ```
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

    /// When true, the overlay is always rendered in the DOM.
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
/// where `onPointerDownOutside` is prevented).
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::alert_dialog::*;
/// rsx! {
///     AlertDialogRoot {
///         AlertDialogTrigger { "Open" }
///         AlertDialogOverlay {}
///         AlertDialogContent {
///             AlertDialogTitle { "Title" }
///             AlertDialogDescription { "Desc" }
///             AlertDialogCancel { "Cancel" }
///             AlertDialogAction { "OK" }
///         }
///     }
/// };
/// ```
///
/// ## Data Attributes
/// - `data-state`: `"open"` or `"closed"`.
#[component]
pub fn AlertDialogOverlay(props: AlertDialogOverlayProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open;

    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);

    // No onclick handler — alert dialog overlay does NOT close on click
    // Radix deviation: Radix uses ReactDOM.createPortal to render the overlay
    // at document.body. We use our Portal component which teleports content to
    // the nearest PortalHost via context-based signal system.
    rsx! {
        Presence {
            present: props.force_mount || open(),
            id: id,
            Portal {
                div {
                    id,
                    "data-slot": "alert-dialog-overlay",
                    "data-state": if open() { "open" } else { "closed" },
                    class: props.class,
                    ..props.attributes,
                }
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

    /// When true, the content is always rendered in the DOM.
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
/// Matches Radix's `AlertDialogContent`. Uses `role="alertdialog"` instead
/// of `role="dialog"`. Traps focus and closes on Escape.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::alert_dialog::*;
/// rsx! {
///     AlertDialogRoot {
///         AlertDialogTrigger { "Open" }
///         AlertDialogOverlay {}
///         AlertDialogContent {
///             AlertDialogTitle { "Confirm deletion" }
///             AlertDialogDescription { "This action cannot be undone." }
///             AlertDialogCancel { "Cancel" }
///             AlertDialogAction { "Delete" }
///         }
///     }
/// };
/// ```
///
/// ## Data Attributes
/// - `data-state`: `"open"` or `"closed"`.
#[component]
pub fn AlertDialogContent(props: AlertDialogContentProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open;
    let set_open = ctx.set_open;

    // Escape key listener
    use_global_escape_listener(move || set_open.call(false));

    // Prevent body scrolling when alert dialog is open (always modal).
    // Matches Radix's react-remove-scroll integration.
    #[allow(clippy::redundant_closure)]
    let scroll_lock_active = use_memo(move || open());
    use_scroll_lock(scroll_lock_active);

    let gen_id = use_unique_id();
    let id = use_id_or(gen_id, props.id);

    // Hide sibling elements from assistive technology (always modal).
    // Matches Radix's aria-hidden integration.
    use_aria_hidden(id, scroll_lock_active);

    let trapped = open();

    // Radix deviation: Radix uses ReactDOM.createPortal to render the content
    // at document.body. We use our Portal component which teleports content to
    // the nearest PortalHost via context-based signal system.
    rsx! {
        Presence {
            present: props.force_mount || open(),
            id: id,
            Portal {
                FocusScope {
                    trapped: trapped,
                    r#loop: trapped,
                    div {
                        id,
                        "data-slot": "alert-dialog-content",
                        "data-state": if open() { "open" } else { "closed" },
                        role: "alertdialog",
                        aria_modal: "true",
                        aria_labelledby: ctx.title_id,
                        aria_describedby: ctx.description_id,
                        class: props.class,
                        ..props.attributes,
                        {props.children}
                    }
                }
            }
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
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::alert_dialog::*;
/// rsx! {
///     AlertDialogRoot {
///         AlertDialogTrigger { "Open" }
///         AlertDialogOverlay {}
///         AlertDialogContent {
///             AlertDialogTitle { "Confirm" }
///             AlertDialogDescription { "Are you sure?" }
///             AlertDialogCancel { "No" }
///             AlertDialogAction { "Yes" }
///         }
///     }
/// };
/// ```
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
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::alert_dialog::*;
/// rsx! {
///     AlertDialogRoot {
///         AlertDialogTrigger { "Open" }
///         AlertDialogOverlay {}
///         AlertDialogContent {
///             AlertDialogTitle { "Title" }
///             AlertDialogDescription { "Description text." }
///             AlertDialogCancel { "Cancel" }
///             AlertDialogAction { "OK" }
///         }
///     }
/// };
/// ```
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
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::alert_dialog::*;
/// rsx! {
///     AlertDialogRoot {
///         AlertDialogTrigger { "Open" }
///         AlertDialogOverlay {}
///         AlertDialogContent {
///             AlertDialogTitle { "Confirm" }
///             AlertDialogDescription { "Are you sure?" }
///             AlertDialogCancel { "No" }
///             AlertDialogAction { "Yes, do it" }
///         }
///     }
/// };
/// ```
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
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::alert_dialog::*;
/// rsx! {
///     AlertDialogRoot {
///         AlertDialogTrigger { "Open" }
///         AlertDialogOverlay {}
///         AlertDialogContent {
///             AlertDialogTitle { "Confirm" }
///             AlertDialogDescription { "Are you sure?" }
///             AlertDialogCancel { "Cancel" }
///             AlertDialogAction { "OK" }
///         }
///     }
/// };
/// ```
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
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::alert_dialog::*;
/// rsx! {
///     AlertDialogRoot {
///         AlertDialogTrigger { "Open" }
///         AlertDialogOverlay {}
///         AlertDialogContent {
///             AlertDialogTitle { "Confirm" }
///             AlertDialogDescription { "Are you sure?" }
///             AlertDialogFooter {
///                 AlertDialogCancel { "Cancel" }
///                 AlertDialogAction { "OK" }
///             }
///         }
///     }
/// };
/// ```
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
