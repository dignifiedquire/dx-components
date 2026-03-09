use super::super::component::*;
use crate::components::button::component::Button;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex w-full max-w-sm items-center gap-2",
            Input { r#type: "email", placeholder: "Email" }
            Button { "Subscribe" }
        }
    }
}
