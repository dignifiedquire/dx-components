use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        ButtonGroup {
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Copy" }
            ButtonGroupSeparator {}
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Paste" }
        }
    }
}
