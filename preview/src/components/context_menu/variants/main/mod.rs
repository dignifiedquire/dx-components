use dioxus::prelude::*;
use dioxus_primitives::context_menu::{
    ContextMenuContent, ContextMenuGroup, ContextMenuItem, ContextMenuLabel, ContextMenuRoot,
    ContextMenuSeparator, ContextMenuShortcut, ContextMenuTrigger,
};

#[component]
pub fn Demo() -> Element {
    rsx! {
        ContextMenuRoot {
            ContextMenuTrigger { "Right click here" }
            ContextMenuContent {
                ContextMenuGroup {
                    ContextMenuLabel { "Actions" }
                    ContextMenuItem {
                        "Back"
                        ContextMenuShortcut { "⌘[" }
                    }
                    ContextMenuItem {
                        disabled: true,
                        "Forward"
                        ContextMenuShortcut { "⌘]" }
                    }
                    ContextMenuItem {
                        "Reload"
                        ContextMenuShortcut { "⌘R" }
                    }
                }
                ContextMenuSeparator {}
                ContextMenuItem { "More Tools" }
                ContextMenuSeparator {}
                ContextMenuItem { "Show Bookmarks Bar" }
                ContextMenuItem { "Show Full URLs" }
            }
        }
    }
}
