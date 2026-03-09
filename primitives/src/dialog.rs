//! Dialog primitive — matches `@radix-ui/react-dialog`.
//!
//! A window overlaid on the primary content. Renders a modal dialog with
//! focus trapping, escape-to-close, and overlay click-to-close.

use dioxus::document;
use dioxus::prelude::*;

use crate::use_global_escape_listener;
use crate::{use_controlled, use_id_or, use_presence, use_unique_id, FOCUS_TRAP_JS};

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

/// Context shared by all Dialog sub-components.
#[derive(Clone, Copy)]
pub struct DialogCtx {
    pub(crate) open: Memo<bool>,
    pub(crate) set_open: Callback<bool>,
    pub(crate) is_modal: bool,
    pub(crate) content_id: Signal<String>,
    pub(crate) title_id: Signal<String>,
    pub(crate) description_id: Signal<String>,
}

impl DialogCtx {
    /// Returns whether the dialog is open.
    pub fn is_open(&self) -> bool {
        self.open.cloned()
    }

    /// Sets the open state of the dialog.
    pub fn set_open(&self, open: bool) {
        self.set_open.call(open);
    }
}

// ---------------------------------------------------------------------------
// DialogRoot
// ---------------------------------------------------------------------------

/// Props for [`DialogRoot`].
#[derive(Props, Clone, PartialEq)]
pub struct DialogRootProps {
    /// Whether the dialog is modal (traps focus, shows overlay). Defaults to `true`.
    #[props(default = true)]
    pub modal: bool,

    /// The controlled `open` state.
    pub open: ReadSignal<Option<bool>>,

    /// The default `open` state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// The children of the dialog.
    pub children: Element,
}

/// The root of the dialog. Manages state and provides context.
///
/// Renders **no DOM element** — it is purely a context provider matching
/// Radix's `Dialog.Root`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::dialog::*;
/// rsx! {
///     DialogRoot {
///         DialogTrigger { "Open" }
///         DialogOverlay {}
///         DialogContent {
///             DialogTitle { "Title" }
///             DialogDescription { "Description" }
///             DialogClose { "Close" }
///         }
///     }
/// };
/// ```
#[component]
pub fn DialogRoot(props: DialogRootProps) -> Element {
    let content_id = use_unique_id();
    let title_id = use_unique_id();
    let description_id = use_unique_id();

    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    use_context_provider(|| DialogCtx {
        open,
        set_open,
        is_modal: props.modal,
        content_id,
        title_id,
        description_id,
    });

    rsx! {
        document::Script { src: FOCUS_TRAP_JS, defer: true }
        {props.children}
    }
}

// ---------------------------------------------------------------------------
// DialogTrigger
// ---------------------------------------------------------------------------

/// Props for [`DialogTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct DialogTriggerProps {
    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children.
    pub children: Element,
}

/// A button that toggles the dialog open state.
///
/// Matches Radix's `DialogTrigger`. Renders `<button>` with
/// `aria-haspopup="dialog"`, `aria-expanded`, `aria-controls`, and `data-state`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::dialog::*;
/// rsx! {
///     DialogRoot {
///         DialogTrigger { "Open Dialog" }
///         DialogOverlay {}
///         DialogContent {
///             DialogTitle { "Hello" }
///             DialogDescription { "World" }
///         }
///     }
/// };
/// ```
#[component]
pub fn DialogTrigger(props: DialogTriggerProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open;
    let set_open = ctx.set_open;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "dialog-trigger",
            "data-state": if open() { "open" } else { "closed" },
            aria_haspopup: "dialog",
            aria_expanded: open(),
            aria_controls: ctx.content_id,
            class: props.class,
            onclick: move |_| set_open.call(!open()),
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DialogOverlay
// ---------------------------------------------------------------------------

/// Props for [`DialogOverlay`].
#[derive(Props, Clone, PartialEq)]
pub struct DialogOverlayProps {
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

/// The backdrop overlay behind the dialog content.
///
/// Matches Radix's `DialogOverlay`. Only renders in modal mode.
/// Clicking the overlay closes the dialog.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::dialog::*;
/// rsx! {
///     DialogRoot {
///         DialogTrigger { "Open" }
///         DialogOverlay {}
///         DialogContent {
///             DialogTitle { "Title" }
///             DialogDescription { "Desc" }
///         }
///     }
/// };
/// ```
///
/// ## Data Attributes
/// - `data-state`: `"open"` or `"closed"`.
#[component]
pub fn DialogOverlay(props: DialogOverlayProps) -> Element {
    let ctx: DialogCtx = use_context();

    // Overlay only renders in modal mode (matching Radix)
    if !ctx.is_modal {
        return rsx! {};
    }

    let open = ctx.open;
    let set_open = ctx.set_open;

    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);
    let mut presence = use_presence(open, id);

    if !presence.is_present() && !props.force_mount {
        return rsx! {};
    }

    rsx! {
        div {
            id,
            "data-slot": "dialog-overlay",
            "data-state": presence.data_state(),
            class: props.class,
            onclick: move |_| set_open.call(false),
            onanimationend: move |_| presence.on_animation_end(),
            ..props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// DialogContent
// ---------------------------------------------------------------------------

/// Props for [`DialogContent`].
#[derive(Props, Clone, PartialEq)]
pub struct DialogContentProps {
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

/// The content panel of the dialog.
///
/// Matches Radix's `DialogContent`. Renders with `role="dialog"`,
/// `aria-modal`, `aria-labelledby`, `aria-describedby`. Traps focus
/// when modal and closes on Escape.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::dialog::*;
/// rsx! {
///     DialogRoot {
///         DialogTrigger { "Open" }
///         DialogOverlay {}
///         DialogContent {
///             DialogTitle { "Title" }
///             DialogDescription { "Description" }
///             DialogClose { "Close" }
///         }
///     }
/// };
/// ```
///
/// ## Data Attributes
/// - `data-state`: `"open"` or `"closed"`.
#[component]
pub fn DialogContent(props: DialogContentProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open;
    let set_open = ctx.set_open;
    let is_modal = ctx.is_modal;

    // Escape key listener
    use_global_escape_listener(move || set_open.call(false));

    let id = use_id_or(ctx.content_id, props.id);
    let mut presence = use_presence(open, id);

    // Focus trap for modal dialogs
    use_effect(move || {
        if !is_modal {
            return;
        }
        let eval = document::eval(
            r#"let id = await dioxus.recv();
            let is_open = await dioxus.recv();
            let dialog = document.getElementById(id);

            if (is_open) {
                dialog.trap = window.createFocusTrap(dialog);
            }
            if (!is_open && dialog.trap) {
                dialog.trap.remove();
                dialog.trap = null;
            }"#,
        );
        let _ = eval.send(id.to_string());
        let _ = eval.send(open.cloned());
    });

    if !presence.is_present() && !props.force_mount {
        return rsx! {};
    }

    rsx! {
        div {
            id,
            "data-slot": "dialog-content",
            "data-state": presence.data_state(),
            role: "dialog",
            aria_modal: if is_modal { "true" },
            aria_labelledby: ctx.title_id,
            aria_describedby: ctx.description_id,
            class: props.class,
            onclick: move |e| e.stop_propagation(),
            onanimationend: move |_| presence.on_animation_end(),
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DialogClose
// ---------------------------------------------------------------------------

/// Props for [`DialogClose`].
#[derive(Props, Clone, PartialEq)]
pub struct DialogCloseProps {
    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children.
    pub children: Element,
}

/// A button that closes the dialog.
///
/// Matches Radix's `DialogClose`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::dialog::*;
/// rsx! {
///     DialogRoot {
///         DialogTrigger { "Open" }
///         DialogOverlay {}
///         DialogContent {
///             DialogTitle { "Title" }
///             DialogDescription { "Desc" }
///             DialogClose { "Close" }
///         }
///     }
/// };
/// ```
#[component]
pub fn DialogClose(props: DialogCloseProps) -> Element {
    let ctx: DialogCtx = use_context();
    let set_open = ctx.set_open;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "dialog-close",
            class: props.class,
            onclick: move |_| set_open.call(false),
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DialogTitle
// ---------------------------------------------------------------------------

/// Props for [`DialogTitle`].
#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleProps {
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

/// The title of the dialog. Sets `aria-labelledby` on the content.
///
/// Matches Radix's `DialogTitle`. Renders `<h2>`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::dialog::*;
/// rsx! {
///     DialogRoot {
///         DialogTrigger { "Open" }
///         DialogOverlay {}
///         DialogContent {
///             DialogTitle { "My Title" }
///             DialogDescription { "My Description" }
///         }
///     }
/// };
/// ```
#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let ctx: DialogCtx = use_context();
    let id = use_id_or(ctx.title_id, props.id);

    rsx! {
        h2 {
            id,
            "data-slot": "dialog-title",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DialogDescription
// ---------------------------------------------------------------------------

/// Props for [`DialogDescription`].
#[derive(Props, Clone, PartialEq)]
pub struct DialogDescriptionProps {
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

/// The description of the dialog. Sets `aria-describedby` on the content.
///
/// Matches Radix's `DialogDescription`. Renders `<p>`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::dialog::*;
/// rsx! {
///     DialogRoot {
///         DialogTrigger { "Open" }
///         DialogOverlay {}
///         DialogContent {
///             DialogTitle { "Title" }
///             DialogDescription { "Description text here." }
///         }
///     }
/// };
/// ```
#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let ctx: DialogCtx = use_context();
    let id = use_id_or(ctx.description_id, props.id);

    rsx! {
        p {
            id,
            "data-slot": "dialog-description",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}
