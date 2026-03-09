use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_lucide::IconItalic;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Toggle { aria_label: "Toggle italic",
            IconItalic { size: 16 }
            "Italic"
        }
    }
}
