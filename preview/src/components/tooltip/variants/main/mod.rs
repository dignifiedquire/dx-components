use dioxus::prelude::*;
use dioxus_primitives::tooltip::{Tooltip, TooltipContent, TooltipTrigger};
use dioxus_primitives::ContentSide;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tooltip {
            TooltipTrigger { "Rich content" }
            TooltipContent { side: ContentSide::Left,
                h4 { class: "mt-0 mb-2", "Tooltip title" }
                p { class: "m-0", "This tooltip contains rich HTML content with styling." }
            }
        }
    }
}
