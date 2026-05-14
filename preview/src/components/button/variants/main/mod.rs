use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_lucide::IconArrowUp;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2 md:flex-row",
            Button { variant: ButtonVariant::Outline, "Button" }
            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Icon,
                aria_label: "Submit",
                IconArrowUp { size: 16 }
            }
        }
    }
}
