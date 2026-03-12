//! Popover primitive — matches `@radix-ui/react-popover`.
//!
//! Displays rich content in a portal, triggered by a button.

use std::rc::Rc;

use dioxus::prelude::*;

use crate::focus_scope::FocusScope;
use crate::portal::Portal;
use crate::use_global_escape_listener;
use crate::{use_controlled, use_id_or, use_presence, use_unique_id, ContentAlign, ContentSide};

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

/// Context shared by all Popover sub-components.
#[derive(Clone, Copy)]
pub struct PopoverCtx {
    pub(crate) open: Memo<bool>,
    pub(crate) set_open: Callback<bool>,
    pub(crate) is_modal: bool,
    pub(crate) content_id: Signal<String>,
    pub(crate) trigger_ref: Signal<Option<Rc<MountedData>>>,
}

impl PopoverCtx {
    /// Returns whether the popover is open.
    pub fn is_open(&self) -> bool {
        self.open.cloned()
    }

    /// Sets the open state of the popover.
    pub fn set_open(&self, open: bool) {
        self.set_open.call(open);
    }
}

// ---------------------------------------------------------------------------
// PopoverRoot
// ---------------------------------------------------------------------------

/// Props for [`PopoverRoot`].
#[derive(Props, Clone, PartialEq)]
pub struct PopoverRootProps {
    /// Whether the popover is modal (traps focus). Defaults to `false` (matching Radix).
    #[props(default)]
    pub modal: bool,

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

/// The root of the popover. Manages state and provides context.
///
/// Renders **no DOM element** — purely a context provider matching
/// Radix's `Popover.Root`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::popover::*;
/// rsx! {
///     PopoverRoot {
///         PopoverTrigger { "Open" }
///         PopoverContent {
///             p { "Content here" }
///             PopoverClose { "Close" }
///         }
///     }
/// };
/// ```
#[component]
pub fn PopoverRoot(props: PopoverRootProps) -> Element {
    let content_id = use_unique_id();
    let trigger_ref = use_signal(|| None);

    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    use_context_provider(|| PopoverCtx {
        open,
        set_open,
        is_modal: props.modal,
        content_id,
        trigger_ref,
    });

    rsx! {
        {props.children}
    }
}

// ---------------------------------------------------------------------------
// PopoverTrigger
// ---------------------------------------------------------------------------

/// Props for [`PopoverTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct PopoverTriggerProps {
    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children.
    pub children: Element,
}

/// A button that toggles the popover.
///
/// Matches Radix's `PopoverTrigger`. Renders `<button>` with
/// `aria-haspopup="dialog"`, `aria-expanded`, `aria-controls`, `data-state`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::popover::*;
/// rsx! {
///     PopoverRoot {
///         PopoverTrigger { "Toggle" }
///         PopoverContent {
///             p { "Popover content" }
///         }
///     }
/// };
/// ```
#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    let ctx: PopoverCtx = use_context();
    let open = ctx.open;
    let set_open = ctx.set_open;
    let mut trigger_ref = ctx.trigger_ref;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "popover-trigger",
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
// PopoverContent
// ---------------------------------------------------------------------------

/// Props for [`PopoverContent`].
#[derive(Props, Clone, PartialEq)]
pub struct PopoverContentProps {
    /// The ID of the content element.
    pub id: ReadSignal<Option<String>>,

    /// When true, the content is always rendered in the DOM.
    #[props(default)]
    pub force_mount: bool,

    /// Side of the trigger to place the popover.
    #[props(default = ContentSide::Bottom)]
    pub side: ContentSide,

    /// Alignment relative to the trigger.
    #[props(default = ContentAlign::Center)]
    pub align: ContentAlign,

    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children.
    pub children: Element,
}

/// The content panel of the popover.
///
/// Matches Radix's `PopoverContent`. Renders with `role="dialog"`,
/// `data-state`, `data-side`, `data-align`. Focus trap when modal,
/// closes on Escape.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::popover::*;
/// rsx! {
///     PopoverRoot {
///         PopoverTrigger { "Open" }
///         PopoverContent {
///             side: dioxus_primitives::ContentSide::Bottom,
///             p { "Content" }
///             PopoverClose { "Close" }
///         }
///     }
/// };
/// ```
///
/// ## Data Attributes
/// - `data-state`: `"open"` or `"closed"`.
/// - `data-side`: `"top"`, `"right"`, `"bottom"`, or `"left"`.
/// - `data-align`: `"start"`, `"center"`, or `"end"`.
#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
    let ctx: PopoverCtx = use_context();
    let open = ctx.open;
    let set_open = ctx.set_open;
    let is_modal = ctx.is_modal;

    let id = use_id_or(ctx.content_id, props.id);
    let mut presence = use_presence(open, id);

    // Escape key listener
    use_global_escape_listener(move || set_open.call(false));

    // Restore focus to trigger when popover closes
    let mut was_open = use_signal(|| false);
    use_effect(move || {
        let is_open = open();
        // Use peek() to avoid subscribing to was_open — we only want to
        // re-run when `open` changes, not when we write was_open below.
        if *was_open.peek() && !is_open {
            if let Some(ref trigger) = *ctx.trigger_ref.read() {
                let trigger = trigger.clone();
                spawn(async move {
                    let _ = trigger.set_focus(true).await;
                });
            }
        }
        was_open.set(is_open);
    });

    if !presence.is_present() && !props.force_mount {
        return rsx! {};
    }

    let trapped = is_modal && open();

    // Radix deviation: Radix uses ReactDOM.createPortal to render the content
    // at document.body. We use our Portal component which teleports content to
    // the nearest PortalHost via context-based signal system.
    rsx! {
        Portal {
            FocusScope {
                trapped: trapped,
                r#loop: trapped,
                div {
                    id,
                    "data-slot": "popover-content",
                    "data-state": presence.data_state(),
                    "data-side": props.side.as_str(),
                    "data-align": props.align.as_str(),
                    role: "dialog",
                    aria_modal: if is_modal { "true" },
                    class: props.class,
                    onanimationend: move |_| presence.on_animation_end(),
                    ..props.attributes,
                    {props.children}
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// PopoverClose
// ---------------------------------------------------------------------------

/// Props for [`PopoverClose`].
#[derive(Props, Clone, PartialEq)]
pub struct PopoverCloseProps {
    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children.
    pub children: Element,
}

/// A button that closes the popover.
///
/// Matches Radix's `PopoverClose`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::popover::*;
/// rsx! {
///     PopoverRoot {
///         PopoverTrigger { "Open" }
///         PopoverContent {
///             p { "Content" }
///             PopoverClose { "Close" }
///         }
///     }
/// };
/// ```
#[component]
pub fn PopoverClose(props: PopoverCloseProps) -> Element {
    let ctx: PopoverCtx = use_context();
    let set_open = ctx.set_open;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "popover-close",
            class: props.class,
            onclick: move |_| set_open.call(false),
            ..props.attributes,
            {props.children}
        }
    }
}
