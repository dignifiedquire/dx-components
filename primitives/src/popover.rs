//! Popover primitive — matches `@radix-ui/react-popover`.
//!
//! Displays rich content in a portal, triggered by a button.

use std::rc::Rc;

use dioxus::prelude::*;

use crate::focus_scope::FocusScope;
use crate::popper::{Align, Popper, PopperContent, PopperContentCtx, PopperCtx, Side};
use crate::portal::Portal;
use crate::use_global_escape_listener;
use crate::{use_controlled, use_id_or, use_presence, use_unique_id};

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
/// Wraps children in [`Popper`] for positioning.
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
        Popper {
            {props.children}
        }
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

/// A button that toggles the popover. Also sets the Popper anchor ref.
#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    let ctx: PopoverCtx = use_context();
    let popper_ctx: PopperCtx = use_context();
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
            onmounted: move |e| {
                let data = e.data();
                trigger_ref.set(Some(data.clone()));
                popper_ctx.set_anchor_ref(data);
            },
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

    /// Side of the trigger to place the popover. Defaults to `Bottom`.
    #[props(default)]
    pub side: Side,

    /// Offset from the trigger edge in pixels. Defaults to 0.
    #[props(default)]
    pub side_offset: f64,

    /// Alignment relative to the trigger. Defaults to `Center`.
    #[props(default)]
    pub align: Align,

    /// Offset along the alignment axis. Defaults to 0.
    #[props(default)]
    pub align_offset: f64,

    /// Whether to avoid viewport edge collisions. Defaults to `true`.
    #[props(default = true)]
    pub avoid_collisions: bool,

    /// Collision padding in pixels. Defaults to 0.
    #[props(default)]
    pub collision_padding: f64,

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
/// Positioned via [`PopperContent`]. Renders with `role="dialog"`,
/// `data-state`, `data-side`, `data-align`. Focus trap when modal, closes on Escape.
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
    let data_state = presence.data_state();

    rsx! {
        Portal {
            PopperContent {
                side: props.side,
                side_offset: props.side_offset,
                align: props.align,
                align_offset: props.align_offset,
                avoid_collisions: props.avoid_collisions,
                collision_padding: props.collision_padding,
                css_var_prefix: "popover",

                PopoverContentInner {
                    id,
                    data_state,
                    is_modal,
                    trapped,
                    on_anim_end: move |_: Event<AnimationData>| presence.on_animation_end(),
                    class: props.class,
                    attributes: props.attributes,
                    children: props.children,
                }
            }
        }
    }
}

/// Inner component that reads [`PopperContentCtx`] for `data-side`/`data-align`.
#[derive(Props, Clone, PartialEq)]
struct PopoverContentInnerProps {
    id: Memo<String>,
    data_state: &'static str,
    is_modal: bool,
    trapped: bool,
    on_anim_end: EventHandler<Event<AnimationData>>,
    #[props(default)]
    class: Option<String>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
fn PopoverContentInner(props: PopoverContentInnerProps) -> Element {
    let popper = use_context::<PopperContentCtx>();
    let side = (popper.placed_side)();
    let align = (popper.placed_align)();

    rsx! {
        FocusScope {
            trapped: props.trapped,
            r#loop: props.trapped,
            div {
                id: props.id,
                "data-slot": "popover-content",
                "data-state": props.data_state,
                "data-side": side.as_str(),
                "data-align": align.as_str(),
                role: "dialog",
                aria_modal: if props.is_modal { "true" },
                class: props.class,
                onanimationend: move |e| props.on_anim_end.call(e),
                ..props.attributes,
                {props.children}
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
