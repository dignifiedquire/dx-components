use crate::components::context_menu::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        ContextMenu {
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
