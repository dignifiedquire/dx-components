use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("../../style.css") }
        Command { class: "rounded-lg border shadow-md md:min-w-[450px]",
            CommandInput { placeholder: "Type a command or search..." }
            CommandList {
                CommandEmpty { "No results found." }
                CommandGroup { heading: "Suggestions",
                    CommandItem { value: "calendar", "Calendar" }
                    CommandItem { value: "search-emoji", "Search Emoji" }
                    CommandItem { value: "calculator", "Calculator" }
                }
                CommandSeparator {}
                CommandGroup { heading: "Settings",
                    CommandItem { value: "profile",
                        "Profile"
                        CommandShortcut { "⌘P" }
                    }
                    CommandItem { value: "billing",
                        "Billing"
                        CommandShortcut { "⌘B" }
                    }
                    CommandItem { value: "settings",
                        "Settings"
                        CommandShortcut { "⌘S" }
                    }
                }
            }
        }
    }
}
