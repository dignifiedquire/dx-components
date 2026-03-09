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
                    DropdownMenuItem { "Edit" }
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
// DropdownMenuCheckboxItem
// ---------------------------------------------------------------------------

#[test]
fn checkbox_item_checked() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuContent {
                    DropdownMenuCheckboxItem {
                        checked: true,
                        "Show Toolbar"
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dropdown-menu-checkbox-item""#),
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
    assert!(
        html.contains("Show Toolbar"),
        "checkbox item has children: {html}"
    );
}

#[test]
fn checkbox_item_unchecked() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuContent {
                    DropdownMenuCheckboxItem {
                        checked: false,
                        "Show Toolbar"
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("aria-checked=false"),
        "checkbox item is unchecked: {html}"
    );
}

// ---------------------------------------------------------------------------
// DropdownMenuRadioGroup + DropdownMenuRadioItem
// ---------------------------------------------------------------------------

#[test]
fn radio_group_attributes() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuContent {
                    DropdownMenuRadioGroup {
                        DropdownMenuRadioItem { value: "a".to_string(), "Option A" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dropdown-menu-radio-group""#),
        "radio group has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="group""#),
        "radio group has role=group: {html}"
    );
}

#[test]
fn radio_item_attributes() {
    fn App() -> Element {
        let selected = use_signal(|| Some("a".to_string()));
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuContent {
                    DropdownMenuRadioGroup {
                        value: selected,
                        DropdownMenuRadioItem { value: "a".to_string(), "Option A" }
                        DropdownMenuRadioItem { value: "b".to_string(), "Option B" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dropdown-menu-radio-item""#),
        "radio item has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="menuitemradio""#),
        "radio item has role: {html}"
    );
    // First item should be checked
    assert!(
        html.contains("aria-checked=true"),
        "selected radio item is checked: {html}"
    );
    assert!(
        html.contains("aria-checked=false"),
        "unselected radio item is unchecked: {html}"
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
                        DropdownMenuItem { "Item" }
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
// DropdownMenuSub
// ---------------------------------------------------------------------------

#[test]
fn sub_renders_no_dom() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuContent {
                    DropdownMenuSub {
                        span { "sentinel" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("sentinel"), "sub renders children: {html}");
}

#[test]
fn sub_trigger_attributes() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuContent {
                    DropdownMenuSub {
                        DropdownMenuSubTrigger { "More" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dropdown-menu-sub-trigger""#),
        "sub-trigger has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="menuitem""#),
        "sub-trigger has role=menuitem: {html}"
    );
    assert!(
        html.contains(r#"aria-haspopup="menu""#),
        "sub-trigger has aria-haspopup: {html}"
    );
    assert!(
        html.contains(r#"data-state="closed""#),
        "sub-trigger shows closed: {html}"
    );
}

#[test]
fn sub_content_renders_when_open() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuContent {
                    DropdownMenuSub {
                        open: true,
                        DropdownMenuSubTrigger { "More" }
                        DropdownMenuSubContent {
                            DropdownMenuItem { "Sub Item" }
                        }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dropdown-menu-sub-content""#),
        "sub-content has data-slot: {html}"
    );
    assert!(
        html.contains("Sub Item"),
        "sub-content has children: {html}"
    );
}

#[test]
fn sub_content_hidden_when_closed() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuContent {
                    DropdownMenuSub {
                        DropdownMenuSubContent {
                            DropdownMenuItem { "Sub Item" }
                        }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains(r#"data-slot="dropdown-menu-sub-content""#),
        "sub-content not rendered when closed: {html}"
    );
}

// ---------------------------------------------------------------------------
// DropdownMenuPortal
// ---------------------------------------------------------------------------

#[test]
fn portal_passes_children_through() {
    fn App() -> Element {
        rsx! {
            DropdownMenuRoot {
                open: true,
                DropdownMenuPortal {
                    DropdownMenuContent {
                        DropdownMenuItem { "Item" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("Item"), "portal renders children: {html}");
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
                    DropdownMenuItem { "Item" }
                }
            }
        }
    }

    let html = render(App);
    // Extract aria-controls value from trigger
    let controls_prefix = r#"aria-controls=""#;
    let controls_start = html.find(controls_prefix).unwrap() + controls_prefix.len();
    let controls_end = html[controls_start..].find('"').unwrap() + controls_start;
    let controls_id = &html[controls_start..controls_end];

    // The content element should have this as its id
    let expected_id_attr = format!(r#"id="{controls_id}""#);
    let expected_slot = r#"data-slot="dropdown-menu-content""#;
    assert!(
        html.contains(&expected_id_attr) && html.contains(expected_slot),
        "trigger aria-controls matches content id: {html}"
    );
}
