use super::super::component::*;
use dioxus::prelude::*;
use crate::components::label::component::Label;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "grid w-full max-w-sm gap-1.5",
            Label { html_for: "email", "Email" }
            Input { id: "email", r#type: "email", placeholder: "Email" }
        }
    }
}
