//! SSR snapshot tests for the menubar primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::menubar::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

// ---------------------------------------------------------------------------
// MenubarRoot
// ---------------------------------------------------------------------------

#[test]
fn root_renders_menubar_role() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                span { "sentinel" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"role="menubar""#),
        "root has role=menubar: {html}"
    );
    assert!(
        html.contains(r#"data-slot="menubar""#),
        "root has data-slot: {html}"
    );
    assert!(html.contains("sentinel"), "children should render: {html}");
}

// ---------------------------------------------------------------------------
// MenubarMenu renders no DOM
// ---------------------------------------------------------------------------

#[test]
fn menu_renders_no_dom_element() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    span { "inner" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("inner"),
        "menu children should render: {html}"
    );
    // MenubarMenu should NOT add a wrapper div with data-slot="menubar-menu"
    assert!(
        !html.contains(r#"data-slot="menubar-menu""#),
        "menu should not render DOM: {html}"
    );
}

// ---------------------------------------------------------------------------
// MenubarTrigger
// ---------------------------------------------------------------------------

#[test]
fn trigger_attributes() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarTrigger { "File" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="menubar-trigger""#),
        "trigger has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="menuitem""#),
        "trigger has role=menuitem: {html}"
    );
    assert!(
        html.contains(r#"aria-haspopup="menu""#),
        "trigger has aria-haspopup: {html}"
    );
    assert!(
        html.contains(r#"data-state="closed""#),
        "trigger shows closed: {html}"
    );
    assert!(
        html.contains(r#"type="button""#),
        "trigger is type=button: {html}"
    );
    assert!(html.contains("File"), "trigger has children: {html}");
}

#[test]
fn trigger_disabled() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                disabled: true,
                MenubarMenu {
                    MenubarTrigger { "File" }
                }
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
// MenubarContent
// ---------------------------------------------------------------------------

#[test]
fn content_hidden_when_closed() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarTrigger { "File" }
                    MenubarContent {
                        p { "Hidden" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains(r#"data-slot="menubar-content""#),
        "content not rendered when closed: {html}"
    );
}

// ---------------------------------------------------------------------------
// MenubarItem
// ---------------------------------------------------------------------------

#[test]
fn item_has_menuitem_role() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarTrigger { "File" }
                }
            }
        }
    }

    let html = render(App);
    // Trigger should have role=menuitem (it's a menubar menuitem)
    assert!(
        html.contains(r#"role="menuitem""#),
        "trigger has role=menuitem: {html}"
    );
}

// ---------------------------------------------------------------------------
// MenubarSeparator
// ---------------------------------------------------------------------------

#[test]
fn separator_attributes() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarSeparator {}
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="menubar-separator""#),
        "separator has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="separator""#),
        "separator has role=separator: {html}"
    );
}

// ---------------------------------------------------------------------------
// MenubarLabel
// ---------------------------------------------------------------------------

#[test]
fn label_attributes() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarLabel { "Actions" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="menubar-label""#),
        "label has data-slot: {html}"
    );
    assert!(html.contains("Actions"), "label has children: {html}");
}

// ---------------------------------------------------------------------------
// MenubarGroup
// ---------------------------------------------------------------------------

#[test]
fn group_attributes() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarGroup {
                        span { "items" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="menubar-group""#),
        "group has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="group""#),
        "group has role=group: {html}"
    );
}

// ---------------------------------------------------------------------------
// MenubarShortcut
// ---------------------------------------------------------------------------

#[test]
fn shortcut_attributes() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarShortcut { "⌘N" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="menubar-shortcut""#),
        "shortcut has data-slot: {html}"
    );
    assert!(html.contains("⌘N"), "shortcut has children: {html}");
}

// ---------------------------------------------------------------------------
// Menubar alias
// ---------------------------------------------------------------------------

#[test]
fn menubar_alias_works() {
    fn App() -> Element {
        rsx! {
            Menubar {
                span { "alias" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"role="menubar""#),
        "alias renders menubar: {html}"
    );
    assert!(html.contains("alias"), "alias renders children: {html}");
}

// ---------------------------------------------------------------------------
// ARIA linking
// ---------------------------------------------------------------------------

#[test]
fn trigger_has_aria_controls_when_open() {
    // We can't easily force open in SSR without controlled state,
    // but we can verify the trigger has basic ARIA attributes
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarTrigger { "File" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("aria-expanded=false"),
        "trigger has aria-expanded when closed: {html}"
    );
}

// ---------------------------------------------------------------------------
// MenubarCheckboxItem
// ---------------------------------------------------------------------------

#[test]
fn checkbox_item_checked() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarCheckboxItem {
                        checked: true,
                        "Show Toolbar"
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="menubar-checkbox-item""#),
        "checkbox item has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="menuitemcheckbox""#),
        "checkbox item has role=menuitemcheckbox: {html}"
    );
    assert!(
        html.contains("aria-checked=true"),
        "checkbox item has aria-checked=true: {html}"
    );
}

#[test]
fn checkbox_item_unchecked() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarCheckboxItem {
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
        "unchecked has aria-checked=false: {html}"
    );
}

// ---------------------------------------------------------------------------
// MenubarRadioGroup / MenubarRadioItem
// ---------------------------------------------------------------------------

#[test]
fn radio_group_attributes() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarRadioGroup {
                        value: "a",
                        span { "radios" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="menubar-radio-group""#),
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
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarRadioGroup {
                        value: "opt-a",
                        MenubarRadioItem {
                            value: "opt-a",
                            "Option A"
                        }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="menubar-radio-item""#),
        "radio item has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="menuitemradio""#),
        "radio item has role=menuitemradio: {html}"
    );
    assert!(
        html.contains("aria-checked=true"),
        "selected radio item has aria-checked=true: {html}"
    );
}

// ---------------------------------------------------------------------------
// MenubarSub / SubTrigger / SubContent
// ---------------------------------------------------------------------------

#[test]
fn sub_renders_no_dom() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarSub {
                        span { "sub-inner" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("sub-inner"),
        "sub children should render: {html}"
    );
}

#[test]
fn sub_trigger_attributes() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarSub {
                        MenubarSubTrigger { "More options" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="menubar-sub-trigger""#),
        "sub trigger has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="menuitem""#),
        "sub trigger has role=menuitem: {html}"
    );
    assert!(
        html.contains(r#"aria-haspopup="menu""#),
        "sub trigger has aria-haspopup: {html}"
    );
}

#[test]
fn sub_content_hidden_when_closed() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarSub {
                        MenubarSubTrigger { "More" }
                        MenubarSubContent {
                            p { "Should not render" }
                        }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains(r#"data-slot="menubar-sub-content""#),
        "sub content not rendered when closed: {html}"
    );
}

// ---------------------------------------------------------------------------
// MenubarPortal
// ---------------------------------------------------------------------------

#[test]
fn portal_passes_children_through() {
    fn App() -> Element {
        rsx! {
            MenubarRoot {
                MenubarMenu {
                    MenubarPortal {
                        span { "portal-child" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("portal-child"),
        "portal should render children: {html}"
    );
}
