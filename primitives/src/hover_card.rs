//! HoverCard primitive — matches Radix UI HoverCard structure.
//!
//! - [`HoverCardRoot`] (aliased as [`HoverCard`]): No DOM, pure context provider
//! - [`HoverCardTrigger`]: Anchor element that shows/hides card on hover/focus
//! - [`HoverCardContent`]: The card content, visible on hover

use crate::popper::{Align, Popper, PopperContent, PopperCtx, Side};
use crate::portal::Portal;
use crate::{merge_attributes, use_delayed_open, use_id_or, use_presence, use_unique_id};
use dioxus_attributes::attributes;
use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct HoverCardCtx {
    open: Memo<bool>,
    set_open: Callback<bool>,
    handle_delayed_open: Callback<()>,
    handle_delayed_close: Callback<()>,
    handle_immediate_close: Callback<()>,
    content_id: Signal<String>,
}

// ---------------------------------------------------------------------------
// HoverCardRoot (no DOM — pure context provider)
// ---------------------------------------------------------------------------

/// Props for [`HoverCardRoot`].
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardRootProps {
    /// Controlled open state.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// Default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Delay in ms before hover card opens. Defaults to 700 (matching Radix).
    #[props(default = 700)]
    pub open_delay: u64,

    /// Delay in ms before hover card closes. Defaults to 300 (matching Radix).
    #[props(default = 300)]
    pub close_delay: u64,

    /// Children (should include [`HoverCardTrigger`] and [`HoverCardContent`]).
    pub children: Element,
}

/// No-DOM context provider for a hover card. Wraps children in [`Popper`].
#[component]
pub fn HoverCardRoot(props: HoverCardRootProps) -> Element {
    let content_id = use_unique_id();
    let delayed = use_delayed_open(
        props.open,
        props.default_open,
        props.on_open_change,
        props.open_delay,
        props.close_delay,
    );

    use_context_provider(|| HoverCardCtx {
        open: delayed.open,
        set_open: delayed.set_open,
        handle_delayed_open: delayed.handle_delayed_open,
        handle_delayed_close: delayed.handle_delayed_close,
        handle_immediate_close: delayed.handle_immediate_close,
        content_id,
    });

    rsx! {
        Popper {
            {props.children}
        }
    }
}

/// Backward-compatible alias for [`HoverCardRoot`].
#[component]
pub fn HoverCard(props: HoverCardRootProps) -> Element {
    HoverCardRoot(props)
}

// ---------------------------------------------------------------------------
// HoverCardTrigger
// ---------------------------------------------------------------------------

/// Props for [`HoverCardTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardTriggerProps {
    /// Optional ID for the trigger element.
    #[props(default)]
    pub id: Option<String>,

    /// Render the trigger as a custom element (asChild pattern).
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,

    /// Additional attributes for the trigger element.
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    pub attributes: Vec<Attribute>,

    /// Children of the trigger.
    pub children: Element,
}

/// The trigger element. Renders as an `<a>` by default (matching Radix).
///
/// Shows the hover card on pointer enter / focus, hides on leave / blur.
/// Also sets the Popper anchor ref for positioning.
#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
    let ctx: HoverCardCtx = use_context();
    let popper_ctx: PopperCtx = use_context();

    let is_open = (ctx.open)();

    rsx! {
        a {
            id: props.id.clone(),
            "data-slot": "hover-card-trigger",
            "data-state": if is_open { "open" } else { "closed" },
            onpointerenter: move |e: Event<PointerData>| {
                if e.data().pointer_type() == "touch" {
                    return;
                }
                ctx.handle_delayed_open.call(());
            },
            onpointerleave: move |e: Event<PointerData>| {
                if e.data().pointer_type() == "touch" {
                    return;
                }
                ctx.handle_delayed_close.call(());
            },
            onfocus: move |_: Event<FocusData>| {
                ctx.set_open.call(true);
            },
            onblur: move |_: Event<FocusData>| {
                ctx.handle_immediate_close.call(());
            },
            onmounted: move |e: Event<MountedData>| {
                popper_ctx.set_anchor_ref(e.data());
            },
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// HoverCardContent
// ---------------------------------------------------------------------------

/// Props for [`HoverCardContent`].
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardContentProps {
    /// The ID of the content element.
    pub id: ReadSignal<Option<String>>,

    /// When true, the content is always rendered in the DOM.
    #[props(default)]
    pub force_mount: bool,

    /// Side of the trigger to place the hover card (default: Bottom).
    #[props(default)]
    pub side: Side,

    /// Offset from the trigger edge in pixels. Defaults to 0.
    #[props(default)]
    pub side_offset: f64,

    /// Alignment relative to the trigger (default: Center).
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

    /// Additional attributes for the hover card content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children of the hover card content.
    pub children: Element,
}

/// The hover card content. Only rendered when the card is open.
///
/// Positioned via [`PopperContent`]. Keeps the card open while the pointer
/// is inside the content area.
#[component]
pub fn HoverCardContent(props: HoverCardContentProps) -> Element {
    let ctx: HoverCardCtx = use_context();
    let id = use_id_or(ctx.content_id, props.id);
    let mut presence = use_presence(ctx.open, id);

    if !presence.is_present() && !props.force_mount {
        return rsx! {};
    }

    let data_state = presence.data_state();

    let content_attrs = attributes!(div {
        id: id,
        "data-slot": "hover-card-content",
        "data-state": data_state,
    });
    let merged = merge_attributes(vec![content_attrs, props.attributes]);

    rsx! {
        Portal {
            PopperContent {
                side: props.side,
                side_offset: props.side_offset,
                align: props.align,
                align_offset: props.align_offset,
                avoid_collisions: props.avoid_collisions,
                collision_padding: props.collision_padding,
                css_var_prefix: "hover-card",
                class: props.class,
                content_attributes: merged,
                on_animation_end: move |_: Event<AnimationData>| presence.on_animation_end(),
                on_pointer_enter: move |_| {
                    ctx.set_open.call(true);
                },
                on_pointer_leave: move |_| {
                    ctx.handle_delayed_close.call(());
                },

                {props.children}
            }
        }
    }
}
