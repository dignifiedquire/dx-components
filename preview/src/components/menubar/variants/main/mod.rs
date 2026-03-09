use crate::components::menubar::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Menubar {
            MenubarMenu {
                MenubarTrigger { "File" }
                MenubarContent {
                    MenubarGroup {
                        MenubarItem {
                            "New Tab"
                            MenubarShortcut { "⌘T" }
                        }
                        MenubarItem {
                            "New Window"
                            MenubarShortcut { "⌘N" }
                        }
                        MenubarItem {
                            disabled: true,
                            "New Incognito Window"
                        }
                    }
                    MenubarSeparator {}
                    MenubarItem {
                        "Print..."
                        MenubarShortcut { "⌘P" }
                    }
                }
            }
            MenubarMenu {
                MenubarTrigger { "Edit" }
                MenubarContent {
                    MenubarItem {
                        "Undo"
                        MenubarShortcut { "⌘Z" }
                    }
                    MenubarItem {
                        "Redo"
                        MenubarShortcut { "⇧⌘Z" }
                    }
                    MenubarSeparator {}
                    MenubarGroup {
                        MenubarLabel { "Clipboard" }
                        MenubarItem { "Cut" }
                        MenubarItem { "Copy" }
                        MenubarItem { "Paste" }
                    }
                }
            }
            MenubarMenu {
                MenubarTrigger { "View" }
                MenubarContent {
                    MenubarItem { "Always Show Bookmarks Bar" }
                    MenubarItem { "Always Show Full URLs" }
                    MenubarSeparator {}
                    MenubarItem {
                        "Reload"
                        MenubarShortcut { "⌘R" }
                    }
                    MenubarItem {
                        disabled: true,
                        "Force Reload"
                        MenubarShortcut { "⇧⌘R" }
                    }
                    MenubarSeparator {}
                    MenubarItem { "Toggle Fullscreen" }
                }
            }
        }
    }
}
