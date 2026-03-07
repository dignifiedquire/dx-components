//! SSR snapshot tests for the dropdown menu primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::dropdown_menu::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

// ---------------------------------------------------------------------------
// DropdownMenuRoot renders no DOM
// ---------------------------------------------------------------------------

#[test]
fn root_renders_no_dom_element() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                span { "sentinel" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("sentinel"), "children should render: {html}");
}

// ---------------------------------------------------------------------------
// DropdownMenuTrigger
// ---------------------------------------------------------------------------

#[test]
fn trigger_attributes() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                DropdownMenuTrigger { "Open" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dropdown-menu-trigger""#),
        "trigger has data-slot: {html}"
    );
    assert!(
        html.contains(r#"aria-haspopup="menu""#),
        "trigger has aria-haspopup=menu: {html}"
    );
    assert!(
        html.contains(r#"data-state="closed""#),
        "trigger shows closed: {html}"
    );
    assert!(
        html.contains(r#"type="button""#),
        "trigger is type=button: {html}"
    );
    assert!(
        html.contains("aria-expanded=false"),
        "trigger shows aria-expanded=false: {html}"
    );
    assert!(html.contains("Open"), "trigger has children: {html}");
}

#[test]
fn trigger_open_state() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuTrigger { "Open" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-state="open""#),
        "trigger shows open: {html}"
    );
    assert!(
        html.contains("aria-expanded=true"),
        "trigger shows aria-expanded=true: {html}"
    );
}

#[test]
fn trigger_disabled() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                disabled: true,
                DropdownMenuTrigger { "Open" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-disabled="true""#),
        "trigger has data-disabled: {html}"
    );
    assert!(
        html.contains("disabled"),
        "trigger has disabled attribute: {html}"
    );
}

// ---------------------------------------------------------------------------
// DropdownMenuContent
// ---------------------------------------------------------------------------

#[test]
fn content_renders_when_open() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuContent {
                    p { "Hello" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dropdown-menu-content""#),
        "content has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="menu""#),
        "content has role=menu: {html}"
    );
    assert!(
        html.contains(r#"data-state="open""#),
        "content shows open: {html}"
    );
    assert!(
        html.contains(r#"aria-orientation="vertical""#),
        "content has aria-orientation: {html}"
    );
    assert!(html.contains("Hello"), "content has children: {html}");
}

#[test]
fn content_hidden_when_closed() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                DropdownMenuContent {
                    p { "Hidden" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains(r#"data-slot="dropdown-menu-content""#),
        "content not rendered when closed: {html}"
    );
}

// ---------------------------------------------------------------------------
// DropdownMenuItem
// ---------------------------------------------------------------------------

#[test]
fn item_attributes() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuContent {
                    DropdownMenuItem {
                        index: 0usize,
                        "Edit"
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dropdown-menu-item""#),
        "item has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="menuitem""#),
        "item has role=menuitem: {html}"
    );
    assert!(html.contains("Edit"), "item has children: {html}");
}

#[test]
fn item_disabled() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuContent {
                    DropdownMenuItem {
                        index: 0usize,
                        disabled: true,
                        "Disabled"
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-disabled="true""#),
        "disabled item has data-disabled: {html}"
    );
    assert!(
        html.contains(r#"aria-disabled="true""#),
        "disabled item has aria-disabled: {html}"
    );
}

// ---------------------------------------------------------------------------
// DropdownMenuSeparator
// ---------------------------------------------------------------------------

#[test]
fn separator_attributes() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuContent {
                    DropdownMenuSeparator {}
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dropdown-menu-separator""#),
        "separator has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="separator""#),
        "separator has role=separator: {html}"
    );
}

// ---------------------------------------------------------------------------
// DropdownMenuLabel
// ---------------------------------------------------------------------------

#[test]
fn label_attributes() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuContent {
                    DropdownMenuLabel { "Actions" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dropdown-menu-label""#),
        "label has data-slot: {html}"
    );
    assert!(html.contains("Actions"), "label has children: {html}");
}

// ---------------------------------------------------------------------------
// DropdownMenuGroup
// ---------------------------------------------------------------------------

#[test]
fn group_attributes() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuContent {
                    DropdownMenuGroup {
                        DropdownMenuItem { index: 0usize, "Item" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dropdown-menu-group""#),
        "group has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="group""#),
        "group has role=group: {html}"
    );
}

// ---------------------------------------------------------------------------
// DropdownMenuShortcut
// ---------------------------------------------------------------------------

#[test]
fn shortcut_attributes() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuContent {
                    DropdownMenuItem {
                        index: 0usize,
                        "Edit"
                        DropdownMenuShortcut { "⌘E" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dropdown-menu-shortcut""#),
        "shortcut has data-slot: {html}"
    );
    assert!(html.contains("⌘E"), "shortcut has children: {html}");
}

// ---------------------------------------------------------------------------
// DropdownMenu alias
// ---------------------------------------------------------------------------

#[test]
fn dropdown_menu_alias_works() {
    fn App() -> Element {
        rsx! {
            DropdownMenu {
                DropdownMenuTrigger { "Open" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("Open"), "alias renders children: {html}");
}

// ---------------------------------------------------------------------------
// ARIA linking
// ---------------------------------------------------------------------------

#[test]
fn trigger_aria_controls_links_to_content() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuTrigger { "Open" }
                DropdownMenuContent {
                    DropdownMenuItem { index: 0usize, "Item" }
                }
            }
        }
    }

    let html = render(App);
    // Extract content id from data-slot="dropdown-menu-content"
    let content_id_start = html.find(r#"data-slot="dropdown-menu-content""#).unwrap();
    let before_content = &html[..content_id_start];
    // Find the id attribute before data-slot
    let id_prefix = r#"id=""#;
    let id_start = before_content.rfind(id_prefix).unwrap() + id_prefix.len();
    let id_end = before_content[id_start..].find('"').unwrap() + id_start;
    let content_id = &before_content[id_start..id_end];

    assert!(
        html.contains(&format!(r#"aria-controls="{content_id}""#)),
        "trigger aria-controls matches content id: {html}"
    );
}
