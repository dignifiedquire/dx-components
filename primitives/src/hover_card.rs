//! HoverCard primitive — matches Radix UI HoverCard structure.
//!
//! - [`HoverCardRoot`] (aliased as [`HoverCard`]): No DOM, pure context provider
//! - [`HoverCardTrigger`]: Anchor element that shows/hides card on hover/focus
//! - [`HoverCardContent`]: The card content, visible on hover

use crate::{use_animated_open, use_controlled, use_unique_id};
use crate::{ContentAlign, ContentSide};
use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct HoverCardCtx {
    open: Memo<bool>,
    set_open: Callback<bool>,
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

    /// Children (should include [`HoverCardTrigger`] and [`HoverCardContent`]).
    pub children: Element,
}

/// No-DOM context provider for a hover card.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::hover_card::{HoverCardRoot, HoverCardTrigger, HoverCardContent};
/// use dioxus_primitives::ContentSide;
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         HoverCardRoot {
///             HoverCardTrigger { "Hover me" }
///             HoverCardContent {
///                 side: ContentSide::Bottom,
///                 "Card content"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn HoverCardRoot(props: HoverCardRootProps) -> Element {
    let content_id = use_unique_id();
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    use_context_provider(|| HoverCardCtx {
        open,
        set_open,
        content_id,
    });

    rsx! { {props.children} }
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

/// The trigger element. Renders as an `<a>` by default (matching Radix `Primitive.a`).
///
/// Shows the hover card on pointer enter / focus, hides on leave / blur.
#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
    let ctx: HoverCardCtx = use_context();

    let handle_pointer_enter = move |_: Event<PointerData>| {
        ctx.set_open.call(true);
    };

    let handle_pointer_leave = move |_: Event<PointerData>| {
        ctx.set_open.call(false);
    };

    let handle_focus = move |_: Event<FocusData>| {
        ctx.set_open.call(true);
    };

    let handle_blur = move |_: Event<FocusData>| {
        ctx.set_open.call(false);
    };

    let is_open = (ctx.open)();

    rsx! {
        a {
            id: props.id.clone(),
            "data-slot": "hover-card-trigger",
            "data-state": if is_open { "open" } else { "closed" },
            onpointerenter: handle_pointer_enter,
            onpointerleave: handle_pointer_leave,
            onfocus: handle_focus,
            onblur: handle_blur,
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
    /// Side of the trigger to place the hover card (default: Bottom).
    #[props(default = ContentSide::Bottom)]
    pub side: ContentSide,

    /// Alignment relative to the trigger (default: Center).
    #[props(default = ContentAlign::Center)]
    pub align: ContentAlign,

    /// Additional attributes for the hover card content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children of the hover card content.
    pub children: Element,
}

/// The hover card content. Only rendered when the card is open.
///
/// Keeps the card open while the pointer is inside the content area.
/// Has `data-state`, `data-side`, `data-align` attributes.
#[component]
pub fn HoverCardContent(props: HoverCardContentProps) -> Element {
    let ctx: HoverCardCtx = use_context();
    let id = ctx.content_id;

    let handle_pointer_enter = move |_: Event<PointerData>| {
        ctx.set_open.call(true);
    };

    let handle_pointer_leave = move |_: Event<PointerData>| {
        ctx.set_open.call(false);
    };

    let render = use_animated_open(id, ctx.open);

    rsx! {
        if render() {
            div {
                id,
                "data-slot": "hover-card-content",
                "data-state": if ctx.open.cloned() { "open" } else { "closed" },
                "data-side": props.side.as_str(),
                "data-align": props.align.as_str(),
                onpointerenter: handle_pointer_enter,
                onpointerleave: handle_pointer_leave,
                ..props.attributes,
                {props.children}
            }
        }
    }
}
