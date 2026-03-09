use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_lucide::IconItalic;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Toggle { aria_label: "Toggle italic", variant: ToggleVariant::Outline,
            IconItalic { size: 16 }
        }
    }
}
