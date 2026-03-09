use crate::components::hover_card::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        HoverCard {
            HoverCardTrigger {
                class: "text-sm font-medium underline underline-offset-4 cursor-pointer",
                href: "#",
                "@dioxuslabs"
            }
            HoverCardContent {
                side: ContentSide::Bottom,
                div { class: "flex justify-between gap-4",
                    div { class: "flex size-10 items-center justify-center rounded-full bg-muted text-sm font-semibold",
                        "DX"
                    }
                    div { class: "space-y-1",
                        h4 { class: "text-sm font-semibold", "@dioxuslabs" }
                        p { class: "text-sm text-muted-foreground",
                            "The Rust framework for building fullstack web, desktop, and mobile apps."
                        }
                        div { class: "flex items-center pt-2",
                            span { class: "text-xs text-muted-foreground", "Joined December 2021" }
                        }
                    }
                }
            }
        }
    }
}
