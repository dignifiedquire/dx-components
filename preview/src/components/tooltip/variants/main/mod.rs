use dioxus::prelude::*;
use dioxus_primitives::tooltip::{TooltipContent, TooltipRoot, TooltipTrigger};

#[component]
pub fn Demo() -> Element {
    rsx! {
        TooltipRoot {
            TooltipTrigger {
                class: "inline-flex items-center justify-center gap-2 rounded-md border border-input bg-background px-4 py-2 text-sm font-medium shadow-xs hover:bg-accent hover:text-accent-foreground",
                "Hover"
            }
            TooltipContent {
                class: "z-50 w-fit rounded-md bg-primary px-3 py-1.5 text-xs text-balance text-primary-foreground animate-in fade-in-0 zoom-in-95",
                p { "Add to library" }
            }
        }
    }
}
