use dioxus::prelude::*;
use dioxus_primitives::scroll_area::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        ScrollArea {
            class: "h-40 w-64 rounded-md border p-4",
            direction: ScrollDirection::Vertical,
            tabindex: "0",
            div {
                for i in 1..=20 {
                    p { class: "text-sm", "Scrollable content item {i}" }
                }
            }
        }
    }
}
