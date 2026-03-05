use crate::components::input::component::Input;
use dioxus::prelude::*;
use dioxus_primitives::label::Label;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { display: "flex", flex_direction: "column", gap: ".5rem",
            Label { html_for: "name", "Name" }

            Input { id: "name", placeholder: "Enter your name" }
        }

    }
}
