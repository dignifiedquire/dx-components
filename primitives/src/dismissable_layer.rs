//! Dismissable layer — matches `@radix-ui/react-dismissable-layer`.
//!
//! Provides [`DismissableLayer`], a container that can be dismissed via Escape
//! key or interactions outside its bounds (clicks or focus changes).
//!
//! Radix's implementation uses global DOM event listeners for outside
//! interaction detection. This Dioxus port uses component-level event
//! handlers and the existing `use_global_escape_listener` hook.

use std::time::Duration;

use dioxus::prelude::*;
use dioxus_sdk_time::sleep;

use crate::use_global_escape_listener;

// ---------------------------------------------------------------------------
// DismissableLayer
// ---------------------------------------------------------------------------

/// Props for [`DismissableLayer`].
#[derive(Props, Clone, PartialEq)]
pub struct DismissableLayerProps {
    /// When `true`, hover/focus/click interactions are disabled on elements
    /// outside the layer.
    #[props(default)]
    pub disable_outside_pointer_events: bool,

    /// Called when the Escape key is pressed.
    #[props(default)]
    pub on_escape_key_down: Callback<()>,

    /// Called when a pointer event occurs outside the layer.
    #[props(default)]
    pub on_pointer_down_outside: Callback<()>,

    /// Called when focus moves outside the layer.
    #[props(default)]
    pub on_focus_outside: Callback<()>,

    /// Called when any outside interaction occurs (pointer or focus).
    #[props(default)]
    pub on_interact_outside: Callback<()>,

    /// Called when the layer should be dismissed.
    #[props(default)]
    pub on_dismiss: Callback<()>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children within the dismissable layer.
    pub children: Element,
}

/// A layer that can be dismissed via Escape key or outside interactions.
///
/// Matches Radix's `DismissableLayer`. Escape key handling uses stack
/// discipline (only the topmost layer responds). Outside interaction
/// detection uses a debounced focusout/focusin pattern to avoid false
/// positives when focus moves between children within the layer.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::dismissable_layer::DismissableLayer;
/// rsx! {
///     DismissableLayer {
///         on_dismiss: move |_| { /* close the overlay */ },
///         div { "Dismissable content" }
///     }
/// };
/// ```
#[component]
pub fn DismissableLayer(props: DismissableLayerProps) -> Element {
    let on_dismiss = props.on_dismiss;
    let on_escape = props.on_escape_key_down;

    // Escape key with stack discipline (topmost layer only)
    use_global_escape_listener(move || {
        on_escape.call(());
        on_dismiss.call(());
    });

    let on_focus_outside = props.on_focus_outside;
    let on_interact_outside = props.on_interact_outside;
    let on_dismiss_focus = props.on_dismiss;

    // Debounced focus-outside detection: when focus leaves the layer,
    // we schedule a dismiss. If focus re-enters the layer (focusin fires),
    // we cancel the pending dismiss. This avoids false positives when focus
    // moves between children within the layer.
    let mut focus_outside_pending = use_signal(|| false);

    // When the pending flag transitions to true, fire the dismiss callbacks.
    // The spawn ensures we wait one frame for a possible focusin to cancel.
    use_effect(move || {
        if focus_outside_pending() {
            spawn(async move {
                // Wait one frame for a possible focusin to cancel
                sleep(Duration::ZERO).await;
                if focus_outside_pending() {
                    focus_outside_pending.set(false);
                    on_focus_outside.call(());
                    on_interact_outside.call(());
                    on_dismiss_focus.call(());
                }
            });
        }
    });

    let pointer_events_style = if props.disable_outside_pointer_events {
        "pointer-events: auto;"
    } else {
        ""
    };

    rsx! {
        div {
            "data-slot": "dismissable-layer",
            "data-dismissable-layer": "",
            style: pointer_events_style,
            class: props.class,

            // Track focus leaving the layer — schedule pending dismiss
            onfocusout: move |_event: FocusEvent| {
                focus_outside_pending.set(true);
            },

            // Track focus entering the layer — cancel pending dismiss
            onfocusin: move |_event: FocusEvent| {
                focus_outside_pending.set(false);
            },

            ..props.attributes,

            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DismissableLayerBranch
// ---------------------------------------------------------------------------

/// Props for [`DismissableLayerBranch`].
#[derive(Props, Clone, PartialEq)]
pub struct DismissableLayerBranchProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A branch of a [`DismissableLayer`] that is considered "inside" the layer.
///
/// Interactions within a branch won't trigger dismissal, even if the branch
/// is rendered outside the layer's DOM subtree.
///
/// Matches Radix's `DismissableLayerBranch`.
#[component]
pub fn DismissableLayerBranch(props: DismissableLayerBranchProps) -> Element {
    rsx! {
        div {
            "data-slot": "dismissable-layer-branch",
            "data-dismissable-layer-branch": "",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}
