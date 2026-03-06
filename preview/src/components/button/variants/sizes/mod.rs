use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            Button { size: ButtonSize::Sm, "Small" }
            Button { "Default" }
            Button { size: ButtonSize::Lg, "Large" }
        }
    }
}
