use crate::components::tooltip::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tooltip {
            TooltipTrigger { "Hover" }
            TooltipContent {
                p { "Add to library" }
            }
        }
    }
}
