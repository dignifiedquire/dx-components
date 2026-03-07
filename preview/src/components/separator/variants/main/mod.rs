use dioxus::prelude::*;
use dioxus_primitives::separator::Separator;

#[component]
pub fn Demo() -> Element {
    rsx! {
        "One thing"
        Separator {
            style: "margin: 15px 0; width: 50%; height: 1px; background-color: var(--border);",
            decorative: true,
        }
        "Another thing"
    }
}
