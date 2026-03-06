use dioxus::prelude::*;
use dioxus_primitives::toggle::{Toggle, ToggleVariant};
use dx_icons_tabler::IconItalic;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Toggle { variant: ToggleVariant::Outline, aria_label: "Toggle italic",
            IconItalic { size: 16 }
        }
    }
}
