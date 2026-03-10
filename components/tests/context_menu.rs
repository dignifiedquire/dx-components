#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::context_menu::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

#[test]
fn context_menu_trigger_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ContextMenu {
                ContextMenuTrigger { "Right click me" }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== context_menu_trigger_slot ===\n{html}\n");

    assert!(html.contains(r#"data-slot="context-menu-trigger""#));
    assert!(html.contains(r#"data-state="closed""#));
}

#[test]
fn context_menu_content_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ContextMenu {
                default_open: true,
                ContextMenuContent {
                    ContextMenuItem { "Item 1" }
                }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== context_menu_content_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="context-menu-content""#));
    assert!(html.contains(r#"role="menu""#));
    assert!(html.contains("z-50"));
    assert!(html.contains("min-w-[8rem]"));
    assert!(html.contains("rounded-md border bg-popover p-1 text-popover-foreground shadow-md"));
}

#[test]
fn context_menu_item_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ContextMenu {
                default_open: true,
                ContextMenuContent {
                    ContextMenuItem { "Item" }
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="context-menu-item""#));
    assert!(html.contains(r#"role="menuitem""#));
    assert!(html.contains("cursor-default"));
    assert!(html.contains("focus:bg-accent focus:text-accent-foreground"));
}

#[test]
fn context_menu_separator_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ContextMenu {
                default_open: true,
                ContextMenuContent {
                    ContextMenuSeparator {}
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="context-menu-separator""#));
    assert!(html.contains("h-px bg-border"));
}

#[test]
fn context_menu_label_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ContextMenu {
                default_open: true,
                ContextMenuContent {
                    ContextMenuLabel { "Label" }
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="context-menu-label""#));
    assert!(html.contains("text-sm font-medium"));
}
