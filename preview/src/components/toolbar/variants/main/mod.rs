use dioxus::prelude::*;
use dioxus_primitives::toggle_group::ToggleGroupItem;

use super::super::component::*;

#[component]
pub fn Demo() -> Element {
    let btn_class = "inline-flex items-center justify-center gap-2 rounded-md px-3 py-1.5 text-sm font-medium hover:bg-muted hover:text-muted-foreground data-[state=on]:bg-accent data-[state=on]:text-accent-foreground";

    rsx! {
        Toolbar {
            aria_label: "Text formatting",
            ToolbarToggleGroup {
                type_: ToggleGroupType::Multiple,
                ToggleGroupItem { value: "bold", class: btn_class, "Bold" }
                ToggleGroupItem { value: "italic", class: btn_class, "Italic" }
                ToggleGroupItem { value: "underline", class: btn_class, "Underline" }
            }
            ToolbarSeparator {}
            ToolbarToggleGroup {
                type_: ToggleGroupType::Single,
                default_value: vec!["left".to_string()],
                ToggleGroupItem { value: "left", class: btn_class, "Align Left" }
                ToggleGroupItem { value: "center", class: btn_class, "Align Center" }
                ToggleGroupItem { value: "right", class: btn_class, "Align Right" }
            }
        }
    }
}
