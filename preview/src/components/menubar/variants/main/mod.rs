use dioxus::prelude::*;
use dioxus_primitives::menubar::{
    MenubarContent, MenubarGroup, MenubarItem, MenubarLabel, MenubarMenu, MenubarRoot,
    MenubarSeparator, MenubarShortcut, MenubarTrigger,
};

#[component]
pub fn Demo() -> Element {
    rsx! {
        MenubarRoot {
            MenubarMenu { index: 0usize,
                MenubarTrigger { "File" }
                MenubarContent {
                    MenubarGroup {
                        MenubarItem {
                            index: 0usize,
                            "New Tab"
                            MenubarShortcut { "⌘T" }
                        }
                        MenubarItem {
                            index: 1usize,
                            "New Window"
                            MenubarShortcut { "⌘N" }
                        }
                        MenubarItem {
                            index: 2usize,
                            disabled: true,
                            "New Incognito Window"
                        }
                    }
                    MenubarSeparator {}
                    MenubarItem {
                        index: 3usize,
                        "Print..."
                        MenubarShortcut { "⌘P" }
                    }
                }
            }
            MenubarMenu { index: 1usize,
                MenubarTrigger { "Edit" }
                MenubarContent {
                    MenubarItem {
                        index: 0usize,
                        "Undo"
                        MenubarShortcut { "⌘Z" }
                    }
                    MenubarItem {
                        index: 1usize,
                        "Redo"
                        MenubarShortcut { "⇧⌘Z" }
                    }
                    MenubarSeparator {}
                    MenubarGroup {
                        MenubarLabel { "Clipboard" }
                        MenubarItem { index: 2usize, "Cut" }
                        MenubarItem { index: 3usize, "Copy" }
                        MenubarItem { index: 4usize, "Paste" }
                    }
                }
            }
            MenubarMenu { index: 2usize,
                MenubarTrigger { "View" }
                MenubarContent {
                    MenubarItem { index: 0usize, "Always Show Bookmarks Bar" }
                    MenubarItem { index: 1usize, "Always Show Full URLs" }
                    MenubarSeparator {}
                    MenubarItem {
                        index: 2usize,
                        "Reload"
                        MenubarShortcut { "⌘R" }
                    }
                    MenubarItem {
                        index: 3usize,
                        disabled: true,
                        "Force Reload"
                        MenubarShortcut { "⇧⌘R" }
                    }
                    MenubarSeparator {}
                    MenubarItem { index: 4usize, "Toggle Fullscreen" }
                }
            }
        }
    }
}
