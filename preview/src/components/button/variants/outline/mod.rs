use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Button { variant: ButtonVariant::Outline, "Outline" }
    }
}
