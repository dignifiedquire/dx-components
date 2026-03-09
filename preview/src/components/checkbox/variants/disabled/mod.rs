use crate::components::checkbox::component::Checkbox;
use crate::components::label::component::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex items-center gap-2",
            Checkbox {
                id: "disabled",
                name: "disabled",
                disabled: true,
            }
            Label { html_for: "disabled", "Accept terms and conditions" }
        }
    }
}
