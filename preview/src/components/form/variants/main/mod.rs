use crate::components::checkbox::component::Checkbox;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        form {
            class: "form-example",
            onsubmit: move |e| {
                tracing::info!("{:?}", e.values());
            },
            Checkbox { id: "tos-check", name: "tos-check" }
            label { r#for: "tos-check", "I agree to the terms presented." }
            br {}
            button { r#type: "submit", "Submit" }
        }
    }
}
