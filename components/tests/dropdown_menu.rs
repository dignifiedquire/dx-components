#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::dropdown_menu::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

#[test]
fn dropdown_menu_trigger_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            DropdownMenu {
                DropdownMenuTrigger { "Open" }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== dropdown_menu_trigger_slot ===\n{html}\n");

    assert!(html.contains(r#"data-slot="dropdown-menu-trigger""#));
    assert!(html.contains(r#"data-state="closed""#));
    assert!(html.contains(r#"aria-haspopup="menu""#));
}

#[test]
fn dropdown_menu_content_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            DropdownMenu {
                default_open: true,
                DropdownMenuContent {
                    DropdownMenuItem { "Item 1" }
                }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== dropdown_menu_content_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="dropdown-menu-content""#));
    assert!(html.contains(r#"role="menu""#));
    assert!(html.contains("z-50"));
    assert!(html.contains("min-w-[8rem]"));
    assert!(html.contains("rounded-md border bg-popover p-1 text-popover-foreground shadow-md"));
}

#[test]
fn dropdown_menu_item_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            DropdownMenu {
                default_open: true,
                DropdownMenuContent {
                    DropdownMenuItem { "Item" }
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="dropdown-menu-item""#));
    assert!(html.contains(r#"role="menuitem""#));
    assert!(html.contains("cursor-default"));
    assert!(html.contains("focus:bg-accent focus:text-accent-foreground"));
}

#[test]
fn dropdown_menu_separator_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            DropdownMenu {
                default_open: true,
                DropdownMenuContent {
                    DropdownMenuSeparator {}
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="dropdown-menu-separator""#));
    assert!(html.contains("h-px bg-border"));
}

#[test]
fn dropdown_menu_label_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            DropdownMenu {
                default_open: true,
                DropdownMenuContent {
                    DropdownMenuLabel { "Label" }
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="dropdown-menu-label""#));
    assert!(html.contains("text-sm font-medium"));
}

#[test]
fn dropdown_menu_consumer_class_merge() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            DropdownMenu {
                default_open: true,
                DropdownMenuContent {
                    class: "my-custom",
                    DropdownMenuItem { "Item" }
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains("my-custom"));
}
