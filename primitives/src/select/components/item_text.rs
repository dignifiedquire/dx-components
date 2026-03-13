//! SelectItemText component implementation.

use crate::use_effect;
use dioxus::prelude::*;

use super::super::context::{SelectContext, SelectItemTextRegistration};

/// The props for the [`SelectItemText`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectItemTextProps {
    /// Explicit text for typeahead search and trigger display. When provided,
    /// overrides the parent item's `text_value` for typeahead matching and
    /// the display text shown in [`SelectValue`](super::value::SelectValue)
    /// when this item is selected.
    ///
    /// If not provided, the parent item's existing `text_value` (or `value`
    /// fallback) is used for typeahead and trigger display.
    #[props(default)]
    pub text: Option<String>,

    /// Additional attributes for the text span.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The content to display inside the item.
    pub children: Element,
}

/// Wraps the display text of a [`SelectItem`](super::option::SelectItem).
///
/// When the `text` prop is provided, it registers the display text for:
/// - **Typeahead search**: users can search by display text instead of value slugs
/// - **Trigger display**: [`SelectValue`](super::value::SelectValue) shows the
///   display text instead of the value slug when this item is selected
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::select::{
///     Select, SelectContent, SelectGroup, SelectItem, SelectItemText,
///     SelectItemIndicator, SelectTrigger, SelectValue,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Select {
///             placeholder: "Pick a fruit...",
///             SelectTrigger {
///                 aria_label: "Fruit",
///                 SelectValue {}
///             }
///             SelectContent {
///                 aria_label: "Fruits",
///                 SelectGroup {
///                     SelectItem { value: "apple",
///                         SelectItemText { text: "Apple", "🍎 Apple" }
///                         SelectItemIndicator { "✓" }
///                     }
///                     SelectItem { value: "banana",
///                         SelectItemText { text: "Banana", "🍌 Banana" }
///                         SelectItemIndicator { "✓" }
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn SelectItemText(props: SelectItemTextProps) -> Element {
    let mut ctx: SelectContext = use_context();
    let registration: SelectItemTextRegistration = use_context();

    // Register text override for typeahead and trigger display
    if let Some(text) = props.text.clone() {
        let value = registration.value;
        use_effect(move || {
            let parent_value = value.cloned();
            let text = text.clone();

            // Store in persistent overrides (survives SelectItemText unmount
            // across list open/close cycles, since Select context never unmounts)
            {
                let mut overrides = ctx.item_text_overrides.write();
                overrides.retain(|(v, _)| v != &parent_value);
                overrides.push((parent_value.clone(), text.clone()));
            }

            // Also update OptionState for typeahead matching
            {
                let mut options = ctx.options.write();
                if let Some(opt) = options.iter_mut().find(|o| o.value == parent_value) {
                    opt.text_value = text;
                }
            }
        });
    }

    rsx! {
        span {
            "data-slot": "select-item-text",
            ..props.attributes,
            {props.children}
        }
    }
}
