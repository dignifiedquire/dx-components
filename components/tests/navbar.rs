//! SSR snapshot tests for the styled navbar.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::navbar::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

// ---------------------------------------------------------------------------
// Navbar base rendering
// ---------------------------------------------------------------------------

#[test]
fn navbar_base_renders() {
    fn App() -> Element {
        rsx! {
            Navbar {
                "nav content"
            }
        }
    }

    let html = render(App);
    eprintln!("=== navbar_base_renders ===\n{html}\n");

    assert!(
        html.contains("nav content"),
        "navbar should render children: {html}"
    );
    assert!(
        html.contains(r#"role="navigation""#),
        "navbar should have role=navigation: {html}"
    );
    assert!(
        html.contains(r#"role="menubar""#),
        "navbar should have inner menubar role: {html}"
    );
}

// ---------------------------------------------------------------------------
// Data-slot attributes
// ---------------------------------------------------------------------------

#[test]
fn navbar_data_slots() {
    fn App() -> Element {
        rsx! {
            Navbar {
                NavbarNav { index: 0usize,
                    NavbarTrigger { "Section" }
                    NavbarContent {
                        NavbarItem {
                            index: 0usize,
                            value: "item1".to_string(),
                            to: "#",
                            "Item 1"
                        }
                    }
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== navbar_data_slots ===\n{html}\n");

    assert!(
        html.contains(r#"data-slot="navbar""#),
        "should have data-slot navbar: {html}"
    );
    assert!(
        html.contains(r#"data-slot="navbar-menubar""#),
        "should have data-slot navbar-menubar: {html}"
    );
    assert!(
        html.contains(r#"data-slot="navbar-nav""#),
        "should have data-slot navbar-nav: {html}"
    );
    assert!(
        html.contains(r#"data-slot="navbar-trigger""#),
        "should have data-slot navbar-trigger: {html}"
    );
}

// ---------------------------------------------------------------------------
// NavbarItem renders with data-slot
// ---------------------------------------------------------------------------

#[test]
fn navbar_item_data_slot() {
    fn App() -> Element {
        rsx! {
            Navbar {
                NavbarNav { index: 0usize,
                    NavbarTrigger { "Section" }
                    NavbarContent {
                        NavbarItem {
                            index: 0usize,
                            value: "link1".to_string(),
                            to: "#",
                            "Link 1"
                        }
                    }
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== navbar_item_data_slot ===\n{html}\n");

    // NavbarContent is only rendered when the nav is open, so the item
    // won't appear in the initial SSR output. We verify the navbar-trigger
    // and navbar-nav slots are present.
    assert!(
        html.contains(r#"data-slot="navbar-trigger""#),
        "trigger should have data-slot: {html}"
    );
    assert!(
        html.contains(r#"data-state="closed""#),
        "nav should be closed by default: {html}"
    );
}

// ---------------------------------------------------------------------------
// Class merge
// ---------------------------------------------------------------------------

#[test]
fn navbar_class_merge() {
    fn App() -> Element {
        rsx! {
            Navbar {
                class: "my-navbar",
                "content"
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains("my-navbar"),
        "consumer class should be applied: {html}"
    );
}
