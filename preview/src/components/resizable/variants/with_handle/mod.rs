use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        ResizablePanelGroup { class: "max-w-md rounded-lg border md:min-w-[450px]",
            ResizablePanel { default_size: 25.0,
                div { class: "flex h-[200px] items-center justify-center p-6",
                    span { class: "font-semibold", "Sidebar" }
                }
            }
            ResizableHandle { with_handle: true }
            ResizablePanel { default_size: 75.0,
                div { class: "flex h-[200px] items-center justify-center p-6",
                    span { class: "font-semibold", "Content" }
                }
            }
        }
    }
}
