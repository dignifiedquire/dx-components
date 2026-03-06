use dioxus::prelude::*;
use dioxus_primitives::toggle::Toggle;
use dx_icons_tabler::IconBold;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Toggle { aria_label: "Toggle bold",
            IconBold { size: 16 }
        }
    }
}
