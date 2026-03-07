use dioxus::prelude::*;
use dioxus_primitives::toggle_group::{ToggleGroup, ToggleGroupItem, ToggleGroupType};
use dx_icons_tabler::{IconBold, IconItalic, IconUnderline};

#[component]
pub fn Demo() -> Element {
    rsx! {
        ToggleGroup {
            type_: ToggleGroupType::Single,
            orientation: dioxus_primitives::direction::Orientation::Horizontal,
            class: "flex items-center gap-1",
            ToggleGroupItem {
                value: "bold",
                class: "inline-flex items-center justify-center rounded-md text-sm font-medium h-9 min-w-9 px-2 bg-transparent hover:bg-muted hover:text-muted-foreground data-[state=on]:bg-accent data-[state=on]:text-accent-foreground",
                IconBold { size: 16 }
            }
            ToggleGroupItem {
                value: "italic",
                class: "inline-flex items-center justify-center rounded-md text-sm font-medium h-9 min-w-9 px-2 bg-transparent hover:bg-muted hover:text-muted-foreground data-[state=on]:bg-accent data-[state=on]:text-accent-foreground",
                IconItalic { size: 16 }
            }
            ToggleGroupItem {
                value: "underline",
                class: "inline-flex items-center justify-center rounded-md text-sm font-medium h-9 min-w-9 px-2 bg-transparent hover:bg-muted hover:text-muted-foreground data-[state=on]:bg-accent data-[state=on]:text-accent-foreground",
                IconUnderline { size: 16 }
            }
        }
    }
}
