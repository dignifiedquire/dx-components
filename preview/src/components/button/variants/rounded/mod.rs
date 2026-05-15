use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_lucide::IconArrowUp;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8",
            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Icon,
                class: "rounded-full",
                IconArrowUp {}
            }
        }
    }
}
