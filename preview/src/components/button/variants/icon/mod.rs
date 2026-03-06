use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_tabler::IconChevronRight;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex items-center gap-2",
            Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                IconChevronRight { size: 16 }
            }
        }
    }
}
