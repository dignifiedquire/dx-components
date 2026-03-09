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
                        "Profile"
                        DropdownMenuShortcut { "⇧⌘P" }
                    }
                    DropdownMenuItem {
                        "Billing"
                        DropdownMenuShortcut { "⌘B" }
                    }
                    DropdownMenuItem {
                        "Settings"
                        DropdownMenuShortcut { "⌘S" }
                    }
                    DropdownMenuItem {
                        "Keyboard shortcuts"
                        DropdownMenuShortcut { "⌘K" }
                    }
                }
                DropdownMenuSeparator {}
                DropdownMenuGroup {
                    DropdownMenuItem { "Team" }
                    DropdownMenuItem {
                        disabled: true,
                        "New Team"
                        DropdownMenuShortcut { "⌘+T" }
                    }
                }
                DropdownMenuSeparator {}
                DropdownMenuItem {
                    "Log out"
                    DropdownMenuShortcut { "⇧⌘Q" }
                }
            }
        }
    }
}
