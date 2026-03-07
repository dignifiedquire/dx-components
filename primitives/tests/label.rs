//! SSR snapshot tests for the label primitive.
//!
//! Each test renders a specific label configuration and asserts the exact
//! HTML output matches the expected attributes. This ensures our HTML structure
//! (data-slot, for) matches Radix UI's label.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::label::*;

/// Render a component to an HTML string via SSR.
///
/// The returned HTML is stripped of generated IDs (dxc-N) so snapshots are
/// stable across runs.
fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    // Normalize auto-generated IDs so snapshots are deterministic.
    // Replace `dxc-N` with `dxc-ID` everywhere.
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

// ---------------------------------------------------------------------------
// Snapshot: label renders with data-slot and children
// ---------------------------------------------------------------------------

#[test]
fn renders_label() {
    fn App() -> Element {
        rsx! {
            Label { "Username" }
        }
    }

    let html = render(App);

    // Should render a <label> element
    assert!(html.contains("<label"), "should render a <label> element");

    // data-slot
    assert!(
        html.contains(r#"data-slot="label""#),
        "should have data-slot=label"
    );

    // Children content
    assert!(html.contains("Username"), "should render children content");
}

// ---------------------------------------------------------------------------
// Snapshot: label with html_for
// ---------------------------------------------------------------------------

#[test]
fn html_for() {
    fn App() -> Element {
        rsx! {
            Label { html_for: "input-id", "Name" }
        }
    }

    let html = render(App);

    // for attribute
    assert!(
        html.contains(r#"for="input-id""#),
        "should have for=input-id attribute"
    );

    // data-slot still present
    assert!(
        html.contains(r#"data-slot="label""#),
        "should still have data-slot=label"
    );

    // Children content
    assert!(html.contains("Name"), "should render children content");
}

// ---------------------------------------------------------------------------
// Snapshot: label with custom class
// ---------------------------------------------------------------------------

#[test]
fn with_class() {
    fn App() -> Element {
        rsx! {
            Label { class: "custom", "Field" }
        }
    }

    let html = render(App);

    // Custom class
    assert!(
        html.contains("custom"),
        "label should have the custom class"
    );

    // data-slot still present
    assert!(
        html.contains(r#"data-slot="label""#),
        "should still have data-slot=label"
    );
}
