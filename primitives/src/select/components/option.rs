//! SelectItem (formerly SelectOption) and SelectItemIndicator component implementations.

use crate::{
    focus::use_focus_controlled_item,
    select::context::{RcPartialEqValue, SelectListContext},
    use_effect, use_effect_cleanup, use_id_or, use_unique_id,
};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

use super::super::context::{OptionState, SelectContext, SelectOptionContext};

/// The props for the [`SelectItem`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectItemProps<T: Clone + PartialEq + 'static> {
    /// The value of the item
    pub value: ReadSignal<T>,

    /// The text value of the item used for typeahead search
    #[props(default)]
    pub text_value: ReadSignal<Option<String>>,

    /// Whether the item is disabled
    #[props(default)]
    pub disabled: bool,

    /// Optional ID for the item
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// The index of the item in the list for keyboard navigation focus order.
    pub index: ReadSignal<usize>,

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
pub type SelectOptionProps<T> = SelectItemProps<T>;

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
///         Select::<String> {
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
///                     SelectItem::<String> {
///                         index: 0usize,
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
pub fn SelectItem<T: PartialEq + Clone + 'static>(props: SelectItemProps<T>) -> Element {
    let option_id = use_unique_id();
    let id = use_id_or(option_id, props.id);

    let index = props.index;
    let value = props.value;
    let text_value = use_memo(move || match (props.text_value)() {
        Some(text) => text,
        None => {
            let value = value.read();
            let as_any: &dyn std::any::Any = &*value;
            as_any
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| as_any.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| {
                    tracing::warn!(
                        "SelectItem with non-string types requires text_value to be set"
                    );
                    String::new()
                })
        }
    });

    let mut ctx: SelectContext = use_context();
    use_effect(move || {
        let option_state = OptionState {
            tab_index: index(),
            value: RcPartialEqValue::new(value.cloned()),
            text_value: text_value.cloned(),
            id: id(),
        };
        ctx.options.write().push(option_state);
    });

    use_effect_cleanup(move || {
        ctx.options.write().retain(|opt| opt.id != *id.read());
    });

    let onmounted = use_focus_controlled_item(props.index);
    let focused = move || ctx.focus_state.is_focused(index());
    let item_disabled = ctx.disabled || props.disabled;
    let selected = use_memo(move || {
        ctx.value.read().as_ref().and_then(|v| v.as_ref::<T>()) == Some(&props.value.read())
    });
    let mut did_drag = use_signal(|| false);

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
                        ctx.set_value.call(Some(RcPartialEqValue::new(props.value.cloned())));
                        ctx.open.set(false);
                    }
                },
                ontouchstart: move |_| {
                    did_drag.set(false);
                },
                ontouchend: move |_| {
                    if !item_disabled && !did_drag() {
                        ctx.set_value.call(Some(RcPartialEqValue::new(props.value.cloned())));
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
pub fn SelectOption<T: PartialEq + Clone + 'static>(props: SelectItemProps<T>) -> Element {
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
