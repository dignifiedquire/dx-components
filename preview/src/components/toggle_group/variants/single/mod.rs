use dioxus::prelude::*;
use dioxus_primitives::toggle_group::{ToggleGroup, ToggleItem};
use dx_icons_tabler::{IconBold, IconItalic, IconUnderline};

#[component]
pub fn Demo() -> Element {
    rsx! {
        ToggleGroup { horizontal: true,
            ToggleItem { index: 0usize,
                IconBold { size: 16 }
            }
            ToggleItem { index: 1usize,
                IconItalic { size: 16 }
            }
            ToggleItem { index: 2usize,
                IconUnderline { size: 16 }
            }
        }
    }
}
