use dioxus::prelude::*;
use dioxus_primitives::toggle::Toggle;
use dx_icons_tabler::IconUnderline;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Toggle { disabled: true, aria_label: "Toggle underline",
            IconUnderline { size: 16 }
        }
    }
}
