use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2",
            for (direction, label) in [
                (DrawerDirection::Top, "Top"),
                (DrawerDirection::Right, "Right"),
                (DrawerDirection::Bottom, "Bottom"),
                (DrawerDirection::Left, "Left"),
            ] {
                Drawer {
                    DrawerTrigger { class: "inline-flex items-center justify-center rounded-md border px-4 py-2 text-sm font-medium shadow-sm hover:bg-accent",
                        "{label}"
                    }
                    DrawerOverlay {}
                    DrawerContent { direction,
                        DrawerHeader {
                            DrawerTitle { "{label} Drawer" }
                            DrawerDescription { "This drawer slides in from the {label}." }
                        }
                        div { class: "p-4",
                            p { "Drawer content goes here." }
                        }
                        DrawerFooter {
                            DrawerClose { class: "inline-flex w-full items-center justify-center rounded-md border px-4 py-2 text-sm font-medium shadow-sm hover:bg-accent",
                                "Close"
                            }
                        }
                    }
                }
            }
        }
    }
}
