use crate::components::button::component::{Button, ButtonVariant};
use crate::components::dropdown_menu::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        DropdownMenu {
            DropdownMenuTrigger {
                Button { variant: ButtonVariant::Outline, "Open" }
            }
            DropdownMenuContent {
                class: "w-40",
                align: Align::Start,
                DropdownMenuGroup {
                    DropdownMenuLabel { "My Account" }
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
                }
                DropdownMenuSeparator {}
                DropdownMenuGroup {
                    DropdownMenuItem { "Team" }
                    DropdownMenuSub {
                        DropdownMenuSubTrigger { "Invite users" }
                        DropdownMenuPortal {
                            DropdownMenuSubContent {
                                DropdownMenuItem { "Email" }
                                DropdownMenuItem { "Message" }
                                DropdownMenuSeparator {}
                                DropdownMenuItem { "More..." }
                            }
                        }
                    }
                    DropdownMenuItem {
                        "New Team"
                        DropdownMenuShortcut { "⌘+T" }
                    }
                }
                DropdownMenuSeparator {}
                DropdownMenuGroup {
                    DropdownMenuItem { "GitHub" }
                    DropdownMenuItem { "Support" }
                    DropdownMenuItem { disabled: true, "API" }
                }
                DropdownMenuSeparator {}
                DropdownMenuGroup {
                    DropdownMenuItem {
                        "Log out"
                        DropdownMenuShortcut { "⇧⌘Q" }
                    }
                }
            }
        }
    }
}
