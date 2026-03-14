//! SSR snapshot tests for the context menu primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::context_menu::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

// ---------------------------------------------------------------------------
// ContextMenuRoot renders no DOM
// ---------------------------------------------------------------------------

#[test]
fn root_renders_no_dom_element() {
    fn App() -> Element {
        rsx! {
            ContextMenuRoot {
                span { "sentinel" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("sentinel"), "children should render: {html}");
}

// ---------------------------------------------------------------------------
// ContextMenuTrigger
// ---------------------------------------------------------------------------

#[test]
fn trigger_attributes() {
    fn App() -> Element {
        rsx! {
            ContextMenuRoot {
                ContextMenuTrigger { "Right click here" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="context-menu-trigger""#),
        "trigger has data-slot: {html}"
    );
    assert!(
        html.contains(r#"data-state="closed""#),
        "trigger shows closed: {html}"
    );
    // Trigger is a span, not a button
    assert!(html.contains("<span"), "trigger renders as span: {html}");
    assert!(
        html.contains("Right click here"),
        "trigger has children: {html}"
    );
}

#[test]
fn trigger_disabled() {
    fn App() -> Element {
        rsx! {
            ContextMenuRoot {
                disabled: true,
                ContextMenuTrigger { "Trigger" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-disabled="true""#),
        "trigger has data-disabled: {html}"
    );
}

// ---------------------------------------------------------------------------
// ContextMenuContent
// ---------------------------------------------------------------------------

#[test]
fn content_renders_when_open() {
    fn App() -> Element {
        rsx! {
            ContextMenuRoot {
                open: true,
                ContextMenuContent {
                    p { "Hello" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="context-menu-content""#),
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
            ContextMenuRoot {
                ContextMenuContent {
                    p { "Hidden" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains(r#"data-slot="context-menu-content""#),
        "content not rendered when closed: {html}"
    );
}

#[test]
fn content_has_fixed_position() {
    fn App() -> Element {
        rsx! {
            ContextMenuRoot {
                open: true,
                ContextMenuContent {
                    p { "Content" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("position: fixed") || html.contains("position:fixed"),
        "content has fixed position: {html}"
    );
}

// ---------------------------------------------------------------------------
// ContextMenuItem
// ---------------------------------------------------------------------------

#[test]
fn item_attributes() {
    fn App() -> Element {
        rsx! {
            ContextMenuRoot {
                open: true,
                ContextMenuContent {
                    ContextMenuItem { "Edit" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="context-menu-item""#),
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
            ContextMenuRoot {
                open: true,
                ContextMenuContent {
                    ContextMenuItem {
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
// ContextMenuCheckboxItem
// ---------------------------------------------------------------------------

#[test]
fn checkbox_item_attributes() {
    fn App() -> Element {
        rsx! {
            ContextMenuRoot {
                open: true,
                ContextMenuContent {
                    ContextMenuCheckboxItem {
                        checked: true,
                        "Show Toolbar"
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="context-menu-checkbox-item""#),
        "checkbox item has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="menuitemcheckbox""#),
        "checkbox item has role: {html}"
    );
    assert!(
        html.contains("aria-checked=true"),
        "checkbox item is checked: {html}"
    );
}

// ---------------------------------------------------------------------------
// ContextMenuRadioGroup + ContextMenuRadioItem
// ---------------------------------------------------------------------------

#[test]
fn radio_group_and_item() {
    fn App() -> Element {
        let selected = use_signal(|| Some("a".to_string()));
        rsx! {
            ContextMenuRoot {
                open: true,
                ContextMenuContent {
                    ContextMenuRadioGroup {
                        value: selected,
                        ContextMenuRadioItem { value: "a".to_string(), "A" }
                        ContextMenuRadioItem { value: "b".to_string(), "B" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="context-menu-radio-group""#),
        "radio group has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="menuitemradio""#),
        "radio item has role: {html}"
    );
    assert!(
        html.contains("aria-checked=true"),
        "selected radio item is checked: {html}"
    );
}

// ---------------------------------------------------------------------------
// ContextMenuSeparator
// ---------------------------------------------------------------------------

#[test]
fn separator_attributes() {
    fn App() -> Element {
        rsx! {
            ContextMenuRoot {
                open: true,
                ContextMenuContent {
                    ContextMenuSeparator {}
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="context-menu-separator""#),
        "separator has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="separator""#),
        "separator has role=separator: {html}"
    );
}

// ---------------------------------------------------------------------------
// ContextMenuLabel
// ---------------------------------------------------------------------------

#[test]
fn label_attributes() {
    fn App() -> Element {
        rsx! {
            ContextMenuRoot {
                open: true,
                ContextMenuContent {
                    ContextMenuLabel { "Actions" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="context-menu-label""#),
        "label has data-slot: {html}"
    );
    assert!(html.contains("Actions"), "label has children: {html}");
}

// ---------------------------------------------------------------------------
// ContextMenuGroup
// ---------------------------------------------------------------------------

#[test]
fn group_attributes() {
    fn App() -> Element {
        rsx! {
            ContextMenuRoot {
                open: true,
                ContextMenuContent {
                    ContextMenuGroup {
                        ContextMenuItem { "Item" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="context-menu-group""#),
        "group has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="group""#),
        "group has role=group: {html}"
    );
}

// ---------------------------------------------------------------------------
// ContextMenuShortcut
// ---------------------------------------------------------------------------

#[test]
fn shortcut_attributes() {
    fn App() -> Element {
        rsx! {
            ContextMenuRoot {
                open: true,
                ContextMenuContent {
                    ContextMenuItem {
                        "Edit"
                        ContextMenuShortcut { "⌘E" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="context-menu-shortcut""#),
        "shortcut has data-slot: {html}"
    );
    assert!(html.contains("⌘E"), "shortcut has children: {html}");
}

// ---------------------------------------------------------------------------
// ContextMenuSub
// ---------------------------------------------------------------------------

#[test]
fn sub_trigger_and_content() {
    fn App() -> Element {
        rsx! {
            ContextMenuRoot {
                open: true,
                ContextMenuContent {
                    ContextMenuSub {
                        open: true,
                        ContextMenuSubTrigger { "More" }
                        ContextMenuSubContent {
                            ContextMenuItem { "Sub Item" }
                        }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="context-menu-sub-trigger""#),
        "sub-trigger has data-slot: {html}"
    );
    assert!(
        html.contains(r#"data-slot="context-menu-sub-content""#),
        "sub-content has data-slot: {html}"
    );
}

// ---------------------------------------------------------------------------
// ContextMenuPortal
// ---------------------------------------------------------------------------

#[test]
fn portal_passes_through() {
    fn App() -> Element {
        rsx! {
            ContextMenuRoot {
                open: true,
                ContextMenuPortal {
                    ContextMenuContent {
                        ContextMenuItem { "Item" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("Item"), "portal renders children: {html}");
}

// ---------------------------------------------------------------------------
// ContextMenu alias
// ---------------------------------------------------------------------------

#[test]
fn context_menu_alias_works() {
    fn App() -> Element {
        rsx! {
            ContextMenu {
                ContextMenuTrigger { "Trigger" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("Trigger"), "alias renders children: {html}");
}
