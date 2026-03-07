//! Tooltip primitive — matches Radix UI Tooltip structure.
//!
//! - [`TooltipRoot`] (aliased as [`Tooltip`]): No DOM, pure context provider
//! - [`TooltipTrigger`]: Button element that shows/hides tooltip on hover/focus
//! - [`TooltipContent`]: The tooltip content, rendered with `role="tooltip"`

use crate::{merge_attributes, use_animated_open, use_controlled, use_unique_id};
use crate::{ContentAlign, ContentSide};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub(crate) struct TooltipCtx {
    pub(crate) open: Memo<bool>,
    pub(crate) set_open: Callback<bool>,
    pub(crate) disabled: bool,
    pub(crate) content_id: Signal<String>,
}

// ---------------------------------------------------------------------------
// TooltipRoot (no DOM — pure context provider)
// ---------------------------------------------------------------------------

/// Props for [`TooltipRoot`].
#[derive(Props, Clone, PartialEq)]
pub struct TooltipRootProps {
    /// Controlled open state.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// Default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Whether the tooltip is disabled (prevents open on hover/focus).
    #[props(default)]
    pub disabled: bool,

    /// Children (should include [`TooltipTrigger`] and [`TooltipContent`]).
    pub children: Element,
}

/// No-DOM context provider for a tooltip.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::tooltip::{TooltipRoot, TooltipTrigger, TooltipContent};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         TooltipRoot {
///             TooltipTrigger { "Hover me" }
///             TooltipContent { "Tooltip text" }
///         }
///     }
/// }
/// ```
#[component]
pub fn TooltipRoot(props: TooltipRootProps) -> Element {
    let content_id = use_unique_id();
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    use_context_provider(|| TooltipCtx {
        open,
        set_open,
        disabled: props.disabled,
        content_id,
    });

    rsx! { {props.children} }
}

/// Backward-compatible alias for [`TooltipRoot`].
#[component]
pub fn Tooltip(props: TooltipRootProps) -> Element {
    TooltipRoot(props)
}

// ---------------------------------------------------------------------------
// TooltipTrigger
// ---------------------------------------------------------------------------

/// Props for [`TooltipTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct TooltipTriggerProps {
    /// Optional ID for the trigger element.
    #[props(default)]
    pub id: Option<String>,

    /// Render the trigger as a custom element (asChild pattern).
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,

    /// Additional attributes for the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children of the trigger.
    pub children: Element,
}

/// The trigger element. Renders as a `<button>` by default.
///
/// Shows the tooltip on hover/focus, hides on leave/blur/escape.
/// `aria-describedby` is set only when the tooltip is open (matching Radix).
#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
    let ctx: TooltipCtx = use_context();

    let handle_pointer_enter = move |_: Event<PointerData>| {
        if !ctx.disabled {
            ctx.set_open.call(true);
        }
    };

    let handle_pointer_leave = move |_: Event<PointerData>| {
        if !ctx.disabled {
            ctx.set_open.call(false);
        }
    };

    let handle_focus = move |_: Event<FocusData>| {
        if !ctx.disabled {
            ctx.set_open.call(true);
        }
    };

    let handle_blur = move |_: Event<FocusData>| {
        if !ctx.disabled {
            ctx.set_open.call(false);
        }
    };

    let handle_keydown = move |event: Event<KeyboardData>| {
        if event.key() == Key::Escape && (ctx.open)() {
            event.prevent_default();
            ctx.set_open.call(false);
        }
    };

    let is_open = (ctx.open)();
    let described_by = if is_open {
        Some(ctx.content_id.cloned())
    } else {
        None
    };

    let base = attributes!(button {
        id: props.id.clone(),
        "data-slot": "tooltip-trigger",
        "data-state": if is_open { "open" } else { "closed" },
        "aria-describedby": described_by,
        onpointerenter: handle_pointer_enter,
        onpointerleave: handle_pointer_leave,
        onfocus: handle_focus,
        onblur: handle_blur,
        onkeydown: handle_keydown,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    if let Some(dynamic) = props.r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            button { ..merged, {props.children} }
        }
    }
}

// ---------------------------------------------------------------------------
// TooltipContent
// ---------------------------------------------------------------------------

/// Props for [`TooltipContent`].
#[derive(Props, Clone, PartialEq)]
pub struct TooltipContentProps {
    /// Side of the trigger to place the tooltip (default: Top).
    #[props(default = ContentSide::Top)]
    pub side: ContentSide,

    /// Alignment relative to the trigger (default: Center).
    #[props(default = ContentAlign::Center)]
    pub align: ContentAlign,

    /// Additional attributes for the tooltip content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children of the tooltip content.
    pub children: Element,
}

/// The tooltip content. Only rendered when the tooltip is open.
///
/// Has `role="tooltip"`, `data-state`, `data-side`, `data-align`.
#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
    let ctx: TooltipCtx = use_context();
    let id = ctx.content_id;

    let render = use_animated_open(id, ctx.open);

    rsx! {
        if render() {
            div {
                id,
                role: "tooltip",
                "data-slot": "tooltip-content",
                "data-state": if ctx.open.cloned() { "open" } else { "closed" },
                "data-side": props.side.as_str(),
                "data-align": props.align.as_str(),
                ..props.attributes,
                {props.children}
            }
        }
    }
}
