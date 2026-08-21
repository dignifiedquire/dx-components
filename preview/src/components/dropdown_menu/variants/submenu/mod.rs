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
                DropdownMenuGroup {
                    DropdownMenuItem { "Team" }
                    DropdownMenuSub {
                        DropdownMenuSubTrigger { "Invite users" }
                        DropdownMenuPortal {
                            DropdownMenuSubContent {
                                DropdownMenuItem { "Email" }
                                DropdownMenuItem { "Message" }
                                DropdownMenuSub {
                                    DropdownMenuSubTrigger { "More options" }
                                    DropdownMenuPortal {
                                        DropdownMenuSubContent {
                                            DropdownMenuItem { "Calendly" }
                                            DropdownMenuItem { "Slack" }
                                            DropdownMenuSeparator {}
                                            DropdownMenuItem { "Webhook" }
                                        }
                                    }
                                }
                                DropdownMenuSeparator {}
                                DropdownMenuItem { "Advanced..." }
                            }
                        }
                    }
                    DropdownMenuItem {
                        "New Team"
                        DropdownMenuShortcut { "⌘+T" }
                    }
                }
            }
        }
    }
}
