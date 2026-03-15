//! Tooltip primitive — matches Radix UI Tooltip structure.
//!
//! - [`TooltipProvider`]: Wraps tooltip group, controls delay behavior
//! - [`TooltipRoot`] (aliased as [`Tooltip`]): No DOM, pure context provider
//! - [`TooltipTrigger`]: Button element that shows/hides tooltip on hover/focus
//! - [`TooltipContent`]: The tooltip content, rendered with `role="tooltip"`

use crate::popper::{Align, Popper, PopperContent, PopperCtx, Side};
use crate::portal::Portal;
use crate::presence::Presence;
use crate::use_unique_id;
use crate::{merge_attributes, use_delayed_open, use_id_or};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

// ---------------------------------------------------------------------------
// TooltipProvider context
// ---------------------------------------------------------------------------

/// Context provided by [`TooltipProvider`].
#[derive(Clone, Copy)]
struct TooltipProviderCtx {
    delay_duration: u64,
    skip_delay_duration: u64,
    disable_hoverable_content: bool,
    /// Tracks whether we're within the skip-delay window (recently closed a tooltip).
    is_open_delayed: Signal<bool>,
    /// Generation counter for skip-delay timer cancellation.
    skip_delay_gen: Signal<u64>,
}

// ---------------------------------------------------------------------------
// TooltipProvider
// ---------------------------------------------------------------------------

/// Props for [`TooltipProvider`].
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProviderProps {
    /// Duration in ms before tooltip opens on hover. Defaults to 700.
    #[props(default = 700)]
    pub delay_duration: u64,

    /// Duration in ms during which subsequent tooltips skip the delay. Defaults to 300.
    #[props(default = 300)]
    pub skip_delay_duration: u64,

    /// When true, tooltip content is not hoverable. Defaults to false.
    #[props(default)]
    pub disable_hoverable_content: bool,

    /// Children.
    pub children: Element,
}

/// Provider that wraps a group of tooltips to coordinate delay behavior.
///
/// Matches Radix's `TooltipProvider`. When one tooltip closes, subsequent
/// tooltips within `skip_delay_duration` open instantly.
#[component]
pub fn TooltipProvider(props: TooltipProviderProps) -> Element {
    let is_open_delayed = use_signal(|| true);
    let skip_delay_gen = use_signal(|| 0u64);

    use_context_provider(|| TooltipProviderCtx {
        delay_duration: props.delay_duration,
        skip_delay_duration: props.skip_delay_duration,
        disable_hoverable_content: props.disable_hoverable_content,
        is_open_delayed,
        skip_delay_gen,
    });

    rsx! { {props.children} }
}

// ---------------------------------------------------------------------------
// Tooltip context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub(crate) struct TooltipCtx {
    pub(crate) open: Memo<bool>,
    pub(crate) set_open: Callback<bool>,
    pub(crate) handle_delayed_open: Callback<()>,
    pub(crate) handle_immediate_open: Callback<()>,
    pub(crate) handle_immediate_close: Callback<()>,
    pub(crate) disabled: Signal<bool>,
    pub(crate) content_id: Signal<String>,
    pub(crate) disable_hoverable_content: bool,
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

    /// Override delay duration for this specific tooltip.
    #[props(default)]
    pub delay_duration: Option<u64>,

    /// Whether the tooltip is disabled (prevents open on hover/focus).
    #[props(default)]
    pub disabled: bool,

    /// Children (should include [`TooltipTrigger`] and [`TooltipContent`]).
    pub children: Element,
}

/// No-DOM context provider for a tooltip. Wraps children in [`Popper`].
#[component]
pub fn TooltipRoot(props: TooltipRootProps) -> Element {
    use std::time::Duration;

    let content_id = use_unique_id();

    // Get provider context if available, otherwise use defaults
    let provider: Option<TooltipProviderCtx> = try_consume_context();
    let delay = props
        .delay_duration
        .unwrap_or_else(|| provider.map_or(700, |p| p.delay_duration));
    let disable_hoverable = provider.is_some_and(|p| p.disable_hoverable_content);

    let delayed = use_delayed_open(
        props.open,
        props.default_open,
        props.on_open_change,
        delay,
        0,
    );

    let mut disabled_signal = use_signal(|| props.disabled);
    if *disabled_signal.peek() != props.disabled {
        disabled_signal.set(props.disabled);
    }

    // Skip-delay: notify provider when tooltip opens/closes.
    {
        let open = delayed.open;
        use_effect(move || {
            let is_open = open();
            if let Some(mut provider) = provider {
                if is_open {
                    *provider.skip_delay_gen.write() += 1;
                    provider.is_open_delayed.set(false);
                } else {
                    let gen = {
                        let mut g = provider.skip_delay_gen.write();
                        *g += 1;
                        *g
                    };
                    let skip_duration = provider.skip_delay_duration;
                    let mut is_open_delayed = provider.is_open_delayed;
                    let skip_delay_gen = provider.skip_delay_gen;
                    spawn(async move {
                        dioxus_sdk_time::sleep(Duration::from_millis(skip_duration)).await;
                        if skip_delay_gen() == gen {
                            is_open_delayed.set(true);
                        }
                    });
                }
            }
        });
    }

    use_context_provider(|| TooltipCtx {
        open: delayed.open,
        set_open: delayed.set_open,
        handle_delayed_open: delayed.handle_delayed_open,
        handle_immediate_open: delayed.handle_immediate_open,
        handle_immediate_close: delayed.handle_immediate_close,
        disabled: disabled_signal,
        content_id,
        disable_hoverable_content: disable_hoverable,
    });

    rsx! {
        Popper {
            {props.children}
        }
    }
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
/// Also sets the Popper anchor ref for positioning.
#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
    let ctx: TooltipCtx = use_context();
    let popper_ctx: PopperCtx = use_context();

    let mut pointer_in = use_signal(|| false);
    let provider: Option<TooltipProviderCtx> = try_use_context();

    let handle_pointer_move = move |_: Event<PointerData>| {
        if !(ctx.disabled)() && !pointer_in() {
            pointer_in.set(true);
            let should_skip = provider.is_some_and(|p| !(p.is_open_delayed)());
            if should_skip {
                ctx.handle_immediate_open.call(());
            } else {
                ctx.handle_delayed_open.call(());
            }
        }
    };

    let handle_pointer_leave = move |_: Event<PointerData>| {
        pointer_in.set(false);
        if !(ctx.disabled)() {
            ctx.handle_immediate_close.call(());
        }
    };

    let handle_focus = move |_: Event<FocusData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(true);
        }
    };

    let handle_blur = move |_: Event<FocusData>| {
        if !(ctx.disabled)() {
            ctx.handle_immediate_close.call(());
        }
    };

    let handle_keydown = move |event: Event<KeyboardData>| {
        if event.key() == Key::Escape && (ctx.open)() {
            event.prevent_default();
            ctx.handle_immediate_close.call(());
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
        onpointermove: handle_pointer_move,
        onpointerleave: handle_pointer_leave,
        onfocus: handle_focus,
        onblur: handle_blur,
        onkeydown: handle_keydown,
        onmounted: move |e: Event<MountedData>| {
            popper_ctx.set_anchor_ref(e.data());
        },
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
    /// The ID of the content element.
    pub id: ReadSignal<Option<String>>,

    /// When true, the content is always rendered in the DOM.
    #[props(default)]
    pub force_mount: bool,

    /// Side of the trigger to place the tooltip (default: Top).
    #[props(default = Side::Top)]
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

    /// Additional attributes for the tooltip content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children of the tooltip content.
    pub children: Element,
}

/// The tooltip content. Only rendered when the tooltip is open.
///
/// Positioned via [`PopperContent`]. Has `role="tooltip"`, `data-state`,
/// `data-side`, `data-align`. When hoverable content is enabled (default),
/// moving pointer into the content keeps the tooltip open.
#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
    let ctx: TooltipCtx = use_context();
    let id = use_id_or(ctx.content_id, props.id);

    let disable_hover = ctx.disable_hoverable_content;
    let data_state = if (ctx.open)() { "open" } else { "closed" };

    let content_attrs = attributes!(div {
        id: id,
        role: "tooltip",
        "data-slot": "tooltip-content",
        "data-state": data_state,
    });
    let merged = merge_attributes(vec![content_attrs, props.attributes]);

    rsx! {
        Presence {
            present: props.force_mount || (ctx.open)(),
            id: id,
            Portal {
                PopperContent {
                    side: props.side,
                    side_offset: props.side_offset,
                    align: props.align,
                    align_offset: props.align_offset,
                    avoid_collisions: props.avoid_collisions,
                    collision_padding: props.collision_padding,
                    css_var_prefix: "tooltip",
                    class: props.class,
                    content_attributes: merged,
                    on_pointer_enter: move |_| {
                        if !disable_hover {
                            ctx.set_open.call(true);
                        }
                    },
                    on_pointer_leave: move |_| {
                        if !disable_hover {
                            ctx.handle_immediate_close.call(());
                        }
                    },

                    {props.children}
                }
            }
        }
    }
}
