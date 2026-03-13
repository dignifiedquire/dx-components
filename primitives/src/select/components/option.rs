//! SelectItem (formerly SelectOption) and SelectItemIndicator component implementations.

use crate::{
    focus::use_focus_controlled_item, select::context::SelectListContext, use_effect,
    use_effect_cleanup, use_id_or, use_unique_id,
};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

use super::super::context::{
    OptionState, SelectContext, SelectItemTextRegistration, SelectOptionContext,
};

/// The props for the [`SelectItem`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectItemProps {
    /// The value of the item
    pub value: ReadSignal<String>,

    /// The text value of the item used for typeahead search.
    /// Defaults to `value` if not provided.
    #[props(default)]
    pub text_value: Option<String>,

    /// Whether the item is disabled
    #[props(default)]
    pub disabled: bool,

    /// Optional ID for the item
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Optional label for the item (for accessibility)
    #[props(default)]
    pub aria_label: Option<String>,

    /// Optional description role for the item (for accessibility)
    #[props(default)]
    pub aria_roledescription: Option<String>,

    /// Additional attributes for the item element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the item
    pub children: Element,
}

/// Backward-compatible alias.
pub type SelectOptionProps = SelectItemProps;

/// An individual selectable item within a [`SelectContent`](super::list::SelectContent).
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::select::{
///     Select, SelectGroup, SelectLabel, SelectItemIndicator, SelectContent, SelectItem,
///     SelectTrigger, SelectValue,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Select {
///             placeholder: "Select a fruit...",
///             SelectTrigger {
///                 aria_label: "Select Trigger",
///                 width: "12rem",
///                 SelectValue {}
///             }
///             SelectContent {
///                 aria_label: "Select Demo",
///                 SelectGroup {
///                     SelectLabel { "Fruits" }
///                     SelectItem {
///                         value: "apple",
///                         "Apple"
///                         SelectItemIndicator { "✔️" }
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn SelectItem(props: SelectItemProps) -> Element {
    let option_id = use_unique_id();
    let id = use_id_or(option_id, props.id);

    let mut ctx: SelectContext = use_context();
    let index = use_hook(|| {
        let idx = (ctx.next_index)();
        ctx.next_index.set(idx + 1);
        idx
    });
    let index_signal = use_hook(|| Signal::new(index));
    let value = props.value;
    let text_value = props.text_value.clone().unwrap_or_else(|| value.cloned());

    use_effect(move || {
        let option_state = OptionState {
            tab_index: index,
            value: value.cloned(),
            text_value: text_value.clone(),
            id: id(),
        };
        ctx.options.write().push(option_state);
    });

    use_effect_cleanup(move || {
        ctx.options.write().retain(|opt| opt.id != *id.read());
    });

    let onmounted = use_focus_controlled_item(index_signal);
    let focused = move || ctx.focus_state.is_focused(index);
    let item_disabled = ctx.disabled || props.disabled;
    let selected = use_memo(move || (ctx.value)() == value.cloned());
    let mut did_drag = use_signal(|| false);

    use_context_provider(|| SelectItemTextRegistration { value: props.value });

    use_context_provider(|| SelectOptionContext {
        selected: selected.into(),
    });

    let render = use_context::<SelectListContext>().render;

    rsx! {
        if render() {
            div {
                role: "option",
                "data-slot": "select-item",
                id,
                tabindex: match (item_disabled, focused()) {
                    (true, _) => None::<&str>,
                    (false, true) => Some("0"),
                    (false, false) => Some("-1"),
                },
                onmounted,

                "data-highlighted": if focused() { "" } else { None::<&str> },
                "data-state": if selected() { "checked" } else { "unchecked" },
                "data-disabled": if item_disabled { "" } else { None::<&str> },
                aria_selected: selected() && focused(),
                aria_disabled: if item_disabled { Some("true") } else { None },
                aria_label: props.aria_label.clone(),
                aria_roledescription: props.aria_roledescription.clone(),

                onpointerdown: move |event| {
                    if !item_disabled && &event.pointer_type() == "mouse" && event.trigger_button() == Some(MouseButton::Primary) {
                        ctx.set_value.call(value.cloned());
                        ctx.open.set(false);
                    }
                },
                ontouchstart: move |_| {
                    did_drag.set(false);
                },
                ontouchend: move |_| {
                    if !item_disabled && !did_drag() {
                        ctx.set_value.call(value.cloned());
                        ctx.open.set(false);
                    }
                },
                ontouchmove: move |_| {
                    did_drag.set(true);
                },
                onblur: move |_| {
                    if focused() {
                        ctx.focus_state.blur();
                        ctx.open.set(false);
                    }
                },

                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// Backward-compatible alias for [`SelectItem`].
#[component]
pub fn SelectOption(props: SelectItemProps) -> Element {
    SelectItem(props)
}

/// The props for the [`SelectItemIndicator`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectItemIndicatorProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the indicator
    pub children: Element,
}

/// Renders only when the parent item is selected. Wraps children in `<span>` with `aria-hidden`.
#[component]
pub fn SelectItemIndicator(props: SelectItemIndicatorProps) -> Element {
    let ctx: SelectOptionContext = use_context();
    if !(ctx.selected)() {
        return rsx! {};
    }
    rsx! {
        span {
            "data-slot": "select-item-indicator",
            aria_hidden: "true",
            ..props.attributes,
            {props.children}
        }
    }
}
