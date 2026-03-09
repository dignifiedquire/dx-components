//! SSR snapshot tests for the styled toolbar.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::toolbar::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

// ---------------------------------------------------------------------------
// Base classes
// ---------------------------------------------------------------------------

#[test]
fn toolbar_base_classes() {
    fn App() -> Element {
        rsx! {
            Toolbar {
                ToolbarButton { "Click" }
            }
        }
    }

    let html = render(App);
    eprintln!("=== toolbar_base_classes ===\n{html}\n");

    assert!(
        html.contains("flex items-center gap-1 rounded-md border bg-background p-1"),
        "toolbar should have base classes: {html}"
    );
}

// ---------------------------------------------------------------------------
// Separator classes
// ---------------------------------------------------------------------------

#[test]
fn toolbar_separator_classes() {
    fn App() -> Element {
        rsx! {
            Toolbar {
                ToolbarSeparator {}
            }
        }
    }

    let html = render(App);
    eprintln!("=== toolbar_separator_classes ===\n{html}\n");

    assert!(
        html.contains("shrink-0 bg-border"),
        "separator should have shrink-0 bg-border classes: {html}"
    );
}

// ---------------------------------------------------------------------------
// Class merge
// ---------------------------------------------------------------------------

#[test]
fn toolbar_class_merge() {
    fn App() -> Element {
        rsx! {
            Toolbar {
                class: "my-toolbar",
                ToolbarButton { "Click" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains("my-toolbar"),
        "consumer class should be applied: {html}"
    );
}
