use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_lucide::IconBold;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Toggle { aria_label: "Toggle bold",
            IconBold { size: 16 }
        }
    }
}
