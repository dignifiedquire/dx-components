use crate::components::label::component::Label;
use crate::components::popover::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Popover {
            PopoverTrigger { "Open popover" }
            PopoverContent {
                div { class: "grid gap-4",
                    div { class: "grid gap-2",
                        h4 { class: "font-medium leading-none", "Dimensions" }
                        p { class: "text-sm text-muted-foreground", "Set the dimensions for the layer." }
                    }
                    div { class: "grid gap-2",
                        div { class: "grid grid-cols-3 items-center gap-4",
                            Label { html_for: "width", "Width" }
                            input { id: "width", class: "input col-span-2", value: "100%" }
                        }
                        div { class: "grid grid-cols-3 items-center gap-4",
                            Label { html_for: "max-width", "Max. width" }
                            input { id: "max-width", class: "input col-span-2", value: "300px" }
                        }
                        div { class: "grid grid-cols-3 items-center gap-4",
                            Label { html_for: "height", "Height" }
                            input { id: "height", class: "input col-span-2", value: "25px" }
                        }
                        div { class: "grid grid-cols-3 items-center gap-4",
                            Label { html_for: "max-height", "Max. height" }
                            input { id: "max-height", class: "input col-span-2", value: "none" }
                        }
                    }
                }
            }
        }
    }
}
