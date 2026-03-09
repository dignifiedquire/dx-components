use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_lucide::IconUnderline;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Toggle { disabled: true, aria_label: "Toggle underline",
            IconUnderline { size: 16 }
        }
    }
}
