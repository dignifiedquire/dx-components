use dioxus::prelude::*;
use dioxus_primitives::separator::Separator;

#[component]
pub fn Demo() -> Element {
    rsx! {
        "One thing"
        Separator {
            style: "margin: 15px 0; width: 50%;",
            decorative: true,
        }
        "Another thing"
    }
}
