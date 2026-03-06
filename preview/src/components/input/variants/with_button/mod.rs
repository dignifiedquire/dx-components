use super::super::component::*;
use dioxus::prelude::*;
use dioxus_primitives::button::Button;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex w-full max-w-sm items-center gap-2",
            Input { r#type: "email", placeholder: "Email" }
            Button { "Subscribe" }
        }
    }
}
