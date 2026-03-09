use dioxus::prelude::*;
use dioxus_components::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[component]
pub fn Demo() -> Element {
    rsx! {
        Collapsible {
            CollapsibleTrigger {
                b { "Recent Activity" }
            }
            div { class: "flex flex-col gap-2 max-w-80 text-muted-foreground",
                div { class: "rounded-lg border p-4",
                    "Added a new feature to the collapsible component"
                }
                CollapsibleContent {
                    div { class: "flex flex-col gap-2",
                        div { class: "rounded-lg border p-4",
                            "Fixed a bug in the collapsible component"
                        }
                        div { class: "rounded-lg border p-4",
                            "Updated the documentation for the collapsible component"
                        }
                    }
                }
            }
        }
    }
}
