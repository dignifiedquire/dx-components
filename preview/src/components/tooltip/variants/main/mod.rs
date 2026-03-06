use dioxus::prelude::*;
use dioxus_primitives::button::{Button, ButtonVariant};
use dioxus_primitives::tooltip::{Tooltip, TooltipContent, TooltipTrigger};

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tooltip {
            TooltipTrigger {
                Button { variant: ButtonVariant::Outline, "Hover" }
            }
            TooltipContent {
                p { "Add to library" }
            }
        }
    }
}
