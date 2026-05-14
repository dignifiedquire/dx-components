//! Dialog primitive — matches `@radix-ui/react-dialog`.
//!
//! A window overlaid on the primary content. Renders a modal dialog using
//! the native `<dialog>` element opened with `showModal()`, which provides
//! a focus trap, ESC-to-close, and inert siblings as native browser
//! behaviours. The element is rendered in the [top layer], escaping all
//! ancestor `overflow`, `transform`, `filter`, and stacking-context
//! constraints — no portal required.
//!
//! [top layer]: https://developer.mozilla.org/en-US/docs/Glossary/Top_layer
//!
//! ## Differences from upstream
//!
//! - **No portal**: Upstream uses `ReactDOM.createPortal(content, document.body)`.
//!   We render content as a native `<dialog>` in the top layer, which solves
//!   the same problem without DOM re-parenting.
//! - **Native focus trap**: `<dialog>.showModal()` traps Tab/Shift+Tab inside
//!   the dialog and restores focus to the previously focused element on close.
//!   We do not run our own `FocusScope` wrapper inside the dialog.
//! - **Native inert backdrop**: `<dialog>.showModal()` makes everything outside
//!   the dialog inert (clicks, focus, ARIA). We do not run our own
//!   `aria-hidden` outsider machinery.
//! - **Native ESC handling**: ESC dispatches a `cancel` event on the dialog,
//!   then closes it. The `close` event syncs back into our `open` signal via
//!   [`crate::top_layer::use_top_layer`].
//! - **Backdrop click closes**: A click whose `event.target` is the dialog
//!   element itself (rather than a descendant) is a backdrop click — the
//!   browser routes `::backdrop` clicks to the dialog. We close on this.
//! - **`DialogOverlay` is a styling marker**: it still renders a sibling div
//!   for API compatibility and to host the visible backdrop animation, but
//!   the actual modal/inert semantics live on `<dialog>`. Apps may
//!   alternatively style the native `::backdrop` pseudo-element directly.

use std::rc::Rc;

use dioxus::prelude::*;

use crate::presence::Presence;
use crate::scroll_lock::use_scroll_lock;
use crate::top_layer::{use_top_layer, TopLayerKind};
use crate::{use_controlled, use_id_or, use_unique_id};

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
    pub(crate) trigger_ref: Signal<Option<Rc<MountedData>>>,
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
    let trigger_ref = use_signal(|| None);

    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    use_context_provider(|| DialogCtx {
        open,
        set_open,
        is_modal: props.modal,
        content_id,
        title_id,
        description_id,
        trigger_ref,
    });

    rsx! {
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
#[component]
pub fn DialogTrigger(props: DialogTriggerProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open;
    let set_open = ctx.set_open;
    let mut trigger_ref = ctx.trigger_ref;

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
            onmounted: move |e| trigger_ref.set(Some(e.data())),
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

    /// Retained for API compatibility — always-in-DOM overlay is unconditional.
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
/// Renders a sibling `<div>` with `data-state="open" | "closed"` for CSS
/// animation. Click on this overlay closes the dialog (matching Radix).
///
/// The native `<dialog>::backdrop` pseudo-element also exists in the top
/// layer behind the dialog content; applications that prefer styling that
/// pseudo instead can omit this component and target
/// `dialog[data-slot="dialog-content"]::backdrop` in CSS.
///
/// Only renders in modal mode (matches Radix).
#[component]
pub fn DialogOverlay(props: DialogOverlayProps) -> Element {
    let ctx: DialogCtx = use_context();
    if !ctx.is_modal {
        return rsx! {};
    }
    let open = ctx.open;
    let set_open = ctx.set_open;
    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);

    rsx! {
        Presence {
            present: props.force_mount || open(),
            id: id,
            div {
                id,
                "data-slot": "dialog-overlay",
                "data-state": if open() { "open" } else { "closed" },
                class: props.class,
                // Only close on primary (left) click — matches Radix
                onpointerdown: move |e: PointerEvent| {
                    if e.trigger_button() == Some(dioxus_elements::input_data::MouseButton::Primary) {
                        set_open.call(false);
                    }
                },
                ..props.attributes,
            }
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

    /// Retained for API compatibility — the `<dialog>` element is always
    /// kept in the DOM and toggled via the native `open` attribute.
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
/// Renders as a native `<dialog>` opened with `showModal()` when in modal
/// mode. The browser provides:
///
/// - **Focus trap**: Tab/Shift+Tab cycle inside the dialog.
/// - **Focus restoration**: focus returns to the previously focused element
///   on close.
/// - **Inert siblings**: outside content cannot be clicked, focused, or read
///   by assistive tech.
/// - **ESC dismissal**: ESC fires `cancel` then `close` events, which our
///   [`crate::top_layer::use_top_layer`] sync back into the
///   open signal.
/// - **Top-layer rendering**: escapes ancestor overflow, transform, filter,
///   and stacking contexts without DOM re-parenting.
///
/// Backdrop click (where `event.target === <dialog>`, i.e. the user clicked
/// the `::backdrop` area) closes the dialog, matching Radix's overlay click
/// behaviour.
///
/// ## Data Attributes
/// - `data-state`: `"open"` or `"closed"`.
#[component]
pub fn DialogContent(props: DialogContentProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open;
    let set_open = ctx.set_open;
    let is_modal = ctx.is_modal;

    // Prevent body scroll when modal — `<dialog>` does not lock scroll natively.
    let scroll_lock_active = use_memo(move || is_modal && open());
    use_scroll_lock(scroll_lock_active);

    let id = use_id_or(ctx.content_id, props.id);

    // Drive show_modal() / close() from the controlled open state.
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    let kind = if is_modal {
        TopLayerKind::DialogModal
    } else {
        // Non-modal dialogs fall back to plain `popover="manual"` semantics.
        TopLayerKind::PopoverManual
    };
    use_top_layer(mounted.into(), open.into(), set_open, kind);

    rsx! {
        dialog {
            id,
            "data-slot": "dialog-content",
            "data-state": if open() { "open" } else { "closed" },
            role: "dialog",
            aria_modal: if is_modal { "true" },
            aria_labelledby: ctx.title_id,
            aria_describedby: ctx.description_id,
            class: props.class,
            onmounted: move |e| mounted.set(Some(e.data())),
            // Backdrop click — when `event.target` is the dialog itself
            // (rather than a descendant), the user clicked the ::backdrop area.
            onclick: move |e: MouseEvent| {
                #[cfg(target_arch = "wasm32")]
                {
                    use wasm_bindgen::JsCast;
                    if let Some(web_evt) = e.data().downcast::<web_sys::MouseEvent>() {
                        if let Some(el) = web_evt
                            .target()
                            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                        {
                            if el.get_attribute("data-slot").as_deref()
                                == Some("dialog-content")
                            {
                                set_open.call(false);
                            }
                        }
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                let _ = e;
            },
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
