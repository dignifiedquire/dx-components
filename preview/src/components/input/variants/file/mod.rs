use dioxus::prelude::*;
use crate::components::label::component::Label;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "grid w-full max-w-sm gap-1.5",
            Label { html_for: "picture", "Picture" }
            input { id: "picture", class: "input", r#type: "file" }
        }
    }
}
