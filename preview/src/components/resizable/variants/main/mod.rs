use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        ResizablePanelGroup { class: "max-w-md rounded-lg border md:min-w-[450px]",
            ResizablePanel { default_size: 50.0,
                div { class: "flex h-[200px] items-center justify-center p-6",
                    span { class: "font-semibold", "One" }
                }
            }
            ResizableHandle {}
            ResizablePanel { default_size: 50.0,
                ResizablePanelGroup { orientation: dioxus_primitives::direction::Orientation::Vertical,
                    ResizablePanel { default_size: 50.0,
                        div { class: "flex h-full items-center justify-center p-6",
                            span { class: "font-semibold", "Two" }
                        }
                    }
                    ResizableHandle {}
                    ResizablePanel { default_size: 50.0,
                        div { class: "flex h-full items-center justify-center p-6",
                            span { class: "font-semibold", "Three" }
                        }
                    }
                }
            }
        }
    }
}
