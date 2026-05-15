use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_lucide::{IconMinus, IconPlus};

#[component]
pub fn Demo() -> Element {
    rsx! {
        ButtonGroup {
            orientation: Orientation::Vertical,
            aria_label: "Media controls",
            class: "h-fit",
            Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                IconPlus {}
            }
            Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                IconMinus {}
            }
        }
    }
}
