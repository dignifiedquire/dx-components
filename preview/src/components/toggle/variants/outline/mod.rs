use dioxus::prelude::*;
use dioxus_primitives::toggle::Toggle;
use dx_icons_tabler::IconItalic;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Toggle { aria_label: "Toggle italic",
            class: "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium border border-input bg-transparent shadow-xs hover:bg-accent hover:text-accent-foreground h-9 min-w-9 px-2",
            IconItalic { size: 16 }
        }
    }
}
