use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        ResizablePanelGroup {
            orientation: dioxus_primitives::direction::Orientation::Vertical,
            class: "max-w-md rounded-lg border md:min-w-[450px]",
            ResizablePanel { default_size: 25.0,
                div { class: "flex h-[100px] items-center justify-center p-6",
                    span { class: "font-semibold", "Header" }
                }
            }
            ResizableHandle {}
            ResizablePanel { default_size: 75.0,
                div { class: "flex h-[200px] items-center justify-center p-6",
                    span { class: "font-semibold", "Content" }
                }
            }
        }
    }
}
