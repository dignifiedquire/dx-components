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
                        index: 0usize,
                        "Back"
                        ContextMenuShortcut { "⌘[" }
                    }
                    ContextMenuItem {
                        index: 1usize,
                        disabled: true,
                        "Forward"
                        ContextMenuShortcut { "⌘]" }
                    }
                    ContextMenuItem {
                        index: 2usize,
                        "Reload"
                        ContextMenuShortcut { "⌘R" }
                    }
                }
                ContextMenuSeparator {}
                ContextMenuItem { index: 3usize, "More Tools" }
                ContextMenuSeparator {}
                ContextMenuItem { index: 4usize, "Show Bookmarks Bar" }
                ContextMenuItem { index: 5usize, "Show Full URLs" }
            }
        }
    }
}
