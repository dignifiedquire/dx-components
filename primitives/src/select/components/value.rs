//! SelectValue component implementation.

use dioxus::prelude::*;

use super::super::context::SelectContext;

/// The props for the [`SelectValue`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectValueProps {
    /// Additional attributes for the value element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Displays the currently selected value or the placeholder text.
///
/// Must be used inside a [`Select`](super::select::Select) component,
/// typically within a [`SelectTrigger`](super::trigger::SelectTrigger).
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::select::{
///     Select, SelectGroup, SelectLabel, SelectContent, SelectItem,
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
///                     SelectItem { value: "apple", "Apple" }
///                     SelectItem { value: "banana", "Banana" }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
///
/// ## Styling
///
/// The [`SelectValue`] component defines a span with a `data-placeholder` attribute if a placeholder is set.
#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
    let ctx = use_context::<SelectContext>();

    let current_value = (ctx.value)();
    let has_value = !current_value.is_empty();

    let display_value = if has_value {
        // Prefer text registered by SelectItemText (persists across list open/close)
        ctx.item_text_overrides
            .read()
            .iter()
            .find(|(v, _)| v == &current_value)
            .map(|(_, t)| t.clone())
            .or_else(|| {
                ctx.options
                    .read()
                    .iter()
                    .find(|opt| opt.value == current_value)
                    .map(|opt| opt.text_value.clone())
            })
            .unwrap_or_else(|| ctx.placeholder.cloned())
    } else {
        ctx.placeholder.cloned()
    };

    rsx! {
        span {
            "data-slot": "select-value",
            "data-placeholder": if !has_value { "" } else { None::<&str> },
            pointer_events: "none",
            ..props.attributes,
            {display_value}
        }
    }
}
