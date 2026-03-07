use dioxus::prelude::*;
use dioxus_primitives::dropdown_menu::{
    DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel,
    DropdownMenuRoot, DropdownMenuSeparator, DropdownMenuShortcut, DropdownMenuTrigger,
};

#[component]
pub fn Demo() -> Element {
    rsx! {
        DropdownMenuRoot {
            DropdownMenuTrigger { "Open" }
            DropdownMenuContent {
                DropdownMenuLabel { "My Account" }
                DropdownMenuSeparator {}
                DropdownMenuGroup {
                    DropdownMenuItem {
                        index: 0usize,
                        "Profile"
                        DropdownMenuShortcut { "⇧⌘P" }
                    }
                    DropdownMenuItem {
                        index: 1usize,
                        "Billing"
                        DropdownMenuShortcut { "⌘B" }
                    }
                    DropdownMenuItem {
                        index: 2usize,
                        "Settings"
                        DropdownMenuShortcut { "⌘S" }
                    }
                    DropdownMenuItem {
                        index: 3usize,
                        "Keyboard shortcuts"
                        DropdownMenuShortcut { "⌘K" }
                    }
                }
                DropdownMenuSeparator {}
                DropdownMenuGroup {
                    DropdownMenuItem { index: 4usize, "Team" }
                    DropdownMenuItem {
                        index: 5usize,
                        disabled: true,
                        "New Team"
                        DropdownMenuShortcut { "⌘+T" }
                    }
                }
                DropdownMenuSeparator {}
                DropdownMenuItem {
                    index: 6usize,
                    "Log out"
                    DropdownMenuShortcut { "⇧⌘Q" }
                }
            }
        }
    }
}
