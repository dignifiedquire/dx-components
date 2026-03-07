//! Defines the [`Collapsible`] component and its sub-components.

use crate::{
    merge_attributes, use_collapsible_content_dimensions, use_controlled, use_id_or, use_presence,
    use_unique_id,
};
use dioxus::prelude::*;
use dioxus_attributes::attributes;
use tailwind_fuse::*;

#[derive(Clone, Copy)]
struct CollapsibleCtx {
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadSignal<bool>,
    keep_mounted: ReadSignal<bool>,
    aria_controls_id: Signal<String>,
}

/// The props for the [`Collapsible`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleProps {
    /// Keep [`CollapsibleContent`] mounted in the DOM when the collapsible is closed.
    ///
    /// This does not apply any special ARIA or other attributes.
    #[props(default)]
    pub keep_mounted: ReadSignal<bool>,

    /// The default `open` state.
    ///
    /// This will be overridden if the component is controlled.
    #[props(default)]
    pub default_open: bool,

    /// The disabled state of the collapsible.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The controlled `open` state of the collapsible.
    ///
    /// If this is provided, you must use `on_open_change`.
    pub open: ReadSignal<Option<bool>>,

    /// A callback for when the open state changes.
    ///
    /// The provided argument is a bool of whether the collapsible is open or closed.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Render the root element as a custom component/element.
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes for the collapsible element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the collapsible component.
    pub children: Element,
}

/// # Collapsible
///
/// A container that can be expanded or collapsed to show or hide its content.
/// Matches the Radix UI Collapsible primitive.
///
/// ## Styling
///
/// - `data-state`: `"open"` or `"closed"`
/// - `data-disabled`: Present when disabled
#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    let aria_controls_id = use_unique_id();
    use_context_provider(|| CollapsibleCtx {
        open,
        set_open,
        disabled: props.disabled,
        keep_mounted: props.keep_mounted,
        aria_controls_id,
    });

    let class = tw_merge!(props.class);
    let data_state = if open() { "open" } else { "closed" };
    let base = attributes!(div {
        "data-slot": "collapsible",
        "data-state": data_state,
        "data-disabled": props.disabled,
        class: class,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    if let Some(dynamic) = props.r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            div {
                ..merged,
                {props.children}
            }
        }
    }
}

/// The props for the [`CollapsibleContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleContentProps {
    /// The ID of the collapsible content element.
    pub id: ReadSignal<Option<String>>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Additional inline styles, merged with computed CSS variable styles.
    ///
    /// Used by AccordionContent to alias `--radix-accordion-content-height/width`.
    #[props(default)]
    pub style: Option<String>,

    /// Additional attributes for the collapsible content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the collapsible content.
    pub children: Element,
}

/// # CollapsibleContent
///
/// The collapsible content that shows/hides. Matches Radix UI's `CollapsibleContentImpl`.
///
/// The outer div is always mounted in the DOM (matching Radix's render-prop `forceMount`
/// behavior in `Presence`). Visibility is controlled via the `hidden` attribute.
///
/// Sets `--radix-collapsible-content-height` and `--radix-collapsible-content-width` CSS variables
/// on the element for use in animations.
///
/// ## Styling
///
/// - `data-state`: `"open"` or `"closed"`
/// - `data-disabled`: Present when disabled
#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element {
    let ctx: CollapsibleCtx = use_context();
    let id = use_id_or(ctx.aria_controls_id, props.id);
    let open = ctx.open;

    let mut presence = use_presence(open, id);
    let dims = use_collapsible_content_dimensions(id, open);

    let style = dims.style(props.style.as_deref());

    // Radix: isOpen = context.open || isPresent
    let is_open = open() || presence.is_present();
    let force = (ctx.keep_mounted)();
    let class = tw_merge!(props.class);

    // Outer div is always rendered (matches Radix's Presence render-prop forceMount).
    // Visibility controlled by `hidden` attribute.
    rsx! {
        div {
            id: id,
            "data-slot": "collapsible-content",
            "data-state": presence.data_state(),
            "data-disabled": ctx.disabled,
            class: class,
            style: "{style}",
            hidden: !is_open,

            onanimationend: move |_| {
                presence.on_animation_end();
            },

            ..props.attributes,

            // Radix: {isOpen && children} — children rendered directly, no wrapper div.
            if is_open || force {
                {props.children}
            }
        }
    }
}

/// The props for the [`CollapsibleTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleTriggerProps {
    /// The ID of the trigger element.
    pub id: ReadSignal<Option<String>>,

    /// Render the trigger element as a custom component/element.
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,

    // Explicit event props — required because `extends = GlobalAttributes`
    // does not capture event handlers (https://github.com/DioxusLabs/dioxus/issues/2467).
    /// Callback fired when the trigger is mounted.
    #[props(default)]
    pub onmounted: Callback<Event<MountedData>>,
    /// Callback fired when the trigger receives focus.
    #[props(default)]
    pub onfocus: Callback<Event<FocusData>>,
    /// Callback fired when a key is pressed on the trigger.
    #[props(default)]
    pub onkeydown: Callback<Event<KeyboardData>>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes for the collapsible trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the collapsible trigger.
    pub children: Element,
}

/// # CollapsibleTrigger
///
/// The button that toggles the collapsible open/closed state.
/// Matches the Radix UI CollapsibleTrigger primitive.
///
/// ## Styling
///
/// - `data-state`: `"open"` or `"closed"`
/// - `data-disabled`: Present when disabled
#[component]
pub fn CollapsibleTrigger(props: CollapsibleTriggerProps) -> Element {
    let ctx: CollapsibleCtx = use_context();

    let open = ctx.open;
    let data_state = if open() { "open" } else { "closed" };

    let id = use_id_or(use_unique_id(), props.id);
    let class = tw_merge!(props.class);
    let base = attributes!(button {
        r#type: "button",
        id: id,
        "data-slot": "collapsible-trigger",
        class: class,
        "data-state": data_state,
        "data-disabled": ctx.disabled,
        disabled: ctx.disabled,
        "aria-controls": ctx.aria_controls_id,
        "aria-expanded": open,
        onclick: move |_| {
            let new_open = !open();
            ctx.set_open.call(new_open);
        },
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    if let Some(dynamic) = props.r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            button {
                onmounted: props.onmounted,
                onfocus: props.onfocus,
                onkeydown: props.onkeydown,
                ..merged,
                {props.children}
            }
        }
    }
}
