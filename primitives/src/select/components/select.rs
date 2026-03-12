//! Main Select component implementation.

use std::time::Duration;

use crate::{use_controlled, use_effect};
use dioxus::prelude::*;
use dioxus_core::Task;

use super::super::context::SelectContext;
use crate::focus::use_focus_provider;

/// Props for the main Select component
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    /// The controlled value of the select.
    /// `None` = uncontrolled, `Some(value)` = controlled.
    #[props(default)]
    pub value: ReadSignal<Option<String>>,

    /// The default value when uncontrolled. Empty string = no selection.
    #[props(default)]
    pub default_value: String,

    /// Callback when the value changes. Receives the selected value string.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Whether the select is disabled
    #[props(default)]
    pub disabled: bool,

    /// Name of the select for form submission. When set, a hidden native
    /// `<select>` element is rendered alongside the custom UI so the
    /// value participates in HTML form submissions (matching Radix's
    /// `BubbleSelect` pattern).
    #[props(default)]
    pub name: ReadSignal<String>,

    /// Whether the native select is required for form validation.
    /// Only has effect when `name` is set.
    #[props(default)]
    pub required: bool,

    /// Optional placeholder text
    #[props(default = ReadSignal::new(Signal::new(String::from("Select an option"))))]
    pub placeholder: ReadSignal<String>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Timeout in milliseconds before clearing typeahead buffer
    #[props(default = ReadSignal::new(Signal::new(Duration::from_millis(1000))))]
    pub typeahead_timeout: ReadSignal<Duration>,

    /// The children of the Select component
    pub children: Element,
}

/// No-DOM context provider for a select.
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
///                     SelectItem {
///                         value: "banana",
///                         "Banana"
///                         SelectItemIndicator { "✔️" }
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Select(props: SelectProps) -> Element {
    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let open = use_signal(|| false);
    let mut typeahead_buffer = use_signal(String::new);
    let options = use_signal(Vec::default);
    let adaptive_keyboard = use_signal(super::super::text_search::AdaptiveKeyboard::new);
    let list_id = use_signal(|| None);
    let mut typeahead_clear_task: Signal<Option<Task>> = use_signal(|| None);

    let focus_state = use_focus_provider(props.roving_loop);

    // Clear the typeahead buffer when the select is closed
    use_effect(move || {
        if !open() {
            // Cancel any pending clear task
            if let Some(task) = typeahead_clear_task.write().take() {
                task.cancel();
            }
            // Clear the buffer immediately
            typeahead_buffer.take();
        }
    });
    let initial_focus = use_signal(|| None);
    let next_index = use_signal(|| 0usize);

    use_context_provider(|| SelectContext {
        typeahead_buffer,
        open,
        value,
        set_value,
        options,
        adaptive_keyboard,
        list_id,
        focus_state,
        disabled: props.disabled,
        placeholder: props.placeholder,
        typeahead_clear_task,
        typeahead_timeout: props.typeahead_timeout,
        initial_focus,
        next_index,
    });

    let name = props.name;
    let required = props.required;
    let disabled = props.disabled;

    rsx! {
        {props.children}

        // Hidden native <select> for form participation (matching Radix's BubbleSelect).
        if !name().is_empty() {
            {
                let current_value = value();
                let current_text = options.read().iter()
                    .find(|opt| opt.value == current_value)
                    .map(|opt| opt.text_value.clone())
                    .unwrap_or_default();

                rsx! {
                    select {
                        "data-slot": "select-native",
                        name: name,
                        required: required,
                        disabled: disabled,
                        aria_hidden: "true",
                        tabindex: "-1",
                        style: "position: absolute; border: 0; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; word-wrap: normal;",
                        value: current_text.clone(),

                        option {
                            value: "",
                            disabled: true,
                        }

                        for opt in options.read().iter() {
                            option {
                                key: "{opt.id}",
                                value: opt.text_value.clone(),
                                selected: opt.value == current_value,
                                {opt.text_value.clone()}
                            }
                        }
                    }
                }
            }
        }
    }
}
