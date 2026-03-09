use crate::components::checkbox::component::Checkbox;
use crate::components::label::component::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex items-center gap-2",
            Checkbox {
                id: "terms",
                name: "terms",
            }
            Label { html_for: "terms", "Accept terms and conditions" }
        }
    }
}
