use dioxus::prelude::*;
use crate::components::label::component::Label;

use super::super::component::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        RadioGroup {
            default_value: "comfortable".to_string(),
            RadioGroupItemWithLabel { value: "default".to_string(), label: "Default" }
            RadioGroupItemWithLabel { value: "comfortable".to_string(), label: "Comfortable" }
            RadioGroupItemWithLabel { value: "compact".to_string(), label: "Compact" }
        }
    }
}

#[component]
fn RadioGroupItemWithLabel(value: String, label: String) -> Element {
    let id = format!("radio-{value}");
    rsx! {
        div { class: "flex items-center gap-2",
            RadioGroupItem {
                value: value,
                id: id.clone(),
            }
            Label { html_for: id, "{label}" }
        }
    }
}
