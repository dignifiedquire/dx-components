use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_lucide::IconCircleFadingArrowUp;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
            IconCircleFadingArrowUp {}
        }
    }
}
