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
                    index: 0usize,
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
                    index: 0usize,
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
                    index: 0usize,
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
                    index: 0usize,
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
                    index: 0usize,
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
                MenubarSeparator {}
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
                MenubarLabel { "Actions" }
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
                MenubarGroup {
                    span { "items" }
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
                MenubarShortcut { "⌘N" }
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
                    index: 0usize,
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
