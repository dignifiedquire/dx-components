//! SSR snapshot tests for the Command primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::command::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn root_renders_container() {
    fn App() -> Element {
        rsx! {
            Command {
                CommandInput {}
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"command\""),
        "root has data-slot: {html}"
    );
}

#[test]
fn input_renders() {
    fn App() -> Element {
        rsx! {
            Command {
                CommandInput { placeholder: "Type a command..." }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"command-input\""),
        "input has data-slot: {html}"
    );
    assert!(
        html.contains("data-slot=\"command-input-wrapper\""),
        "wrapper has data-slot: {html}"
    );
    assert!(
        html.contains("placeholder=\"Type a command...\""),
        "has placeholder: {html}"
    );
}

#[test]
fn list_renders_with_role_listbox() {
    fn App() -> Element {
        rsx! {
            Command {
                CommandList {
                    CommandItem { value: "test", "Test" }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("role=\"listbox\""), "list has role: {html}");
    assert!(
        html.contains("data-slot=\"command-list\""),
        "list has data-slot: {html}"
    );
}

#[test]
fn item_renders_with_role_option() {
    fn App() -> Element {
        rsx! {
            Command {
                CommandList {
                    CommandItem { value: "copy", "Copy" }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("role=\"option\""), "item has role: {html}");
    assert!(
        html.contains("data-slot=\"command-item\""),
        "item has data-slot: {html}"
    );
    assert!(
        html.contains("data-value=\"copy\""),
        "item has data-value: {html}"
    );
}

#[test]
fn empty_renders() {
    fn App() -> Element {
        rsx! {
            Command {
                CommandList {
                    CommandEmpty { "No results found." }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"command-empty\""),
        "empty has data-slot: {html}"
    );
    assert!(html.contains("No results found."), "empty text: {html}");
}

#[test]
fn group_renders_with_heading() {
    fn App() -> Element {
        rsx! {
            Command {
                CommandList {
                    CommandGroup { heading: "Actions",
                        CommandItem { value: "copy", "Copy" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"command-group\""),
        "group has data-slot: {html}"
    );
    assert!(html.contains("role=\"group\""), "group has role: {html}");
    assert!(
        html.contains("data-slot=\"command-group-heading\""),
        "heading has data-slot: {html}"
    );
    assert!(html.contains("Actions"), "heading text: {html}");
}

#[test]
fn separator_renders() {
    fn App() -> Element {
        rsx! {
            Command {
                CommandList {
                    CommandItem { value: "a", "A" }
                    CommandSeparator {}
                    CommandItem { value: "b", "B" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"command-separator\""),
        "separator has data-slot: {html}"
    );
    assert!(
        html.contains("role=\"separator\""),
        "separator has role: {html}"
    );
}

#[test]
fn shortcut_renders() {
    fn App() -> Element {
        rsx! {
            Command {
                CommandList {
                    CommandItem { value: "copy",
                        "Copy"
                        CommandShortcut { "⌘C" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"command-shortcut\""),
        "shortcut has data-slot: {html}"
    );
}

#[test]
fn disabled_item() {
    fn App() -> Element {
        rsx! {
            Command {
                CommandList {
                    CommandItem { value: "disabled", disabled: true, "Disabled" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-disabled=\"true\""),
        "disabled item: {html}"
    );
}

#[test]
fn full_composition() {
    fn App() -> Element {
        rsx! {
            Command {
                CommandInput { placeholder: "Search..." }
                CommandList {
                    CommandEmpty { "No results." }
                    CommandGroup { heading: "Actions",
                        CommandItem { value: "copy",
                            "Copy"
                            CommandShortcut { "⌘C" }
                        }
                        CommandItem { value: "paste", "Paste" }
                    }
                    CommandSeparator {}
                    CommandGroup { heading: "Navigation",
                        CommandItem { value: "home", "Home" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("data-slot=\"command\""), "root: {html}");
    assert!(
        html.contains("data-slot=\"command-input\""),
        "input: {html}"
    );
    assert!(html.contains("data-slot=\"command-list\""), "list: {html}");
    assert!(html.contains("data-slot=\"command-item\""), "item: {html}");
    assert!(
        html.contains("data-slot=\"command-group\""),
        "group: {html}"
    );
    assert!(
        html.contains("data-slot=\"command-separator\""),
        "separator: {html}"
    );
    assert!(
        html.contains("data-slot=\"command-empty\""),
        "empty: {html}"
    );
    assert!(
        html.contains("data-slot=\"command-shortcut\""),
        "shortcut: {html}"
    );
}
