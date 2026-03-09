use crate::components::separator::component::Separator;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        "One thing"
        Separator { class: "my-4 w-1/2" }
        "Another thing"
    }
}
