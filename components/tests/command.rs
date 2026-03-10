//! SSR snapshot tests for the styled Command component.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::command::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn command_has_shadcn_classes() {
    fn App() -> Element {
        rsx! {
            Command {
                CommandInput {}
            }
        }
    }

    let html = render(App);
    assert!(html.contains("bg-popover"), "root has bg-popover: {html}");
    assert!(html.contains("rounded-md"), "root has rounded-md: {html}");
}

#[test]
fn command_input_has_shadcn_classes() {
    fn App() -> Element {
        rsx! {
            Command {
                CommandInput { placeholder: "Search..." }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("bg-transparent"),
        "input has bg-transparent: {html}"
    );
}

#[test]
fn command_list_has_shadcn_classes() {
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
    assert!(
        html.contains("overflow-y-auto"),
        "list has overflow: {html}"
    );
    assert!(html.contains("max-h-[300px]"), "list has max-h: {html}");
}

#[test]
fn command_item_has_shadcn_classes() {
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
    assert!(html.contains("cursor-default"), "item class: {html}");
    assert!(html.contains("rounded-sm"), "item rounded: {html}");
}

#[test]
fn command_empty_has_shadcn_classes() {
    fn App() -> Element {
        rsx! {
            Command {
                CommandList {
                    CommandEmpty { "No results." }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("text-center"), "empty center: {html}");
    assert!(html.contains("py-6"), "empty padding: {html}");
}

#[test]
fn command_group_has_shadcn_classes() {
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
        html.contains("text-foreground"),
        "group has text-foreground: {html}"
    );
}

#[test]
fn command_separator_has_shadcn_classes() {
    fn App() -> Element {
        rsx! {
            Command {
                CommandList {
                    CommandSeparator {}
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("bg-border"), "separator: {html}");
    assert!(html.contains("h-px"), "separator height: {html}");
}

#[test]
fn command_shortcut_has_shadcn_classes() {
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
        html.contains("tracking-widest"),
        "shortcut tracking: {html}"
    );
    assert!(
        html.contains("text-muted-foreground"),
        "shortcut muted: {html}"
    );
}

#[test]
fn full_styled_command_composition() {
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
                    }
                    CommandSeparator {}
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
