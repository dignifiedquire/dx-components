use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger { class: "inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm hover:bg-primary/90",
                "Open Drawer"
            }
            DrawerOverlay {}
            DrawerContent {
                DrawerHeader {
                    DrawerTitle { "Move Goal" }
                    DrawerDescription { "Set your daily activity goal." }
                }
                div { class: "p-4 pb-0",
                    div { class: "flex items-center justify-center space-x-2",
                        div { class: "flex-1 text-center",
                            div { class: "text-7xl font-bold tracking-tighter", "350" }
                            div { class: "text-[0.70rem] uppercase text-muted-foreground", "Calories/day" }
                        }
                    }
                }
                DrawerFooter {
                    button { class: "inline-flex w-full items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm hover:bg-primary/90",
                        "Submit"
                    }
                    DrawerClose { class: "inline-flex w-full items-center justify-center rounded-md border px-4 py-2 text-sm font-medium shadow-sm hover:bg-accent",
                        "Cancel"
                    }
                }
            }
        }
    }
}
