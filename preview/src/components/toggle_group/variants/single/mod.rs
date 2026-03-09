use dioxus::prelude::*;
use dx_icons_lucide::{IconBold, IconItalic, IconUnderline};

use super::super::component::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        ToggleGroup {
            type_: ToggleGroupType::Single,
            ToggleGroupItem { value: "bold", IconBold { size: 16 } }
            ToggleGroupItem { value: "italic", IconItalic { size: 16 } }
            ToggleGroupItem { value: "underline", IconUnderline { size: 16 } }
        }
    }
}
