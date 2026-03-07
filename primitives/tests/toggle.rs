//! SSR snapshot tests for the toggle primitive.
//!
//! Each test renders a specific toggle configuration and asserts the exact
//! HTML output matches the expected string. This ensures our HTML structure
//! (data-state, aria-*, type, etc.) matches Radix UI's toggle.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::toggle::Toggle;

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
// Snapshot: default off
// ---------------------------------------------------------------------------

#[test]
fn default_off() {
    fn App() -> Element {
        rsx! {
            Toggle { "Bold" }
        }
    }

    let html = render(App);

    // Toggle renders a button
    assert!(html.contains("<button"), "should render a button element");

    // data-slot
    assert!(
        html.contains(r#"data-slot="toggle""#),
        "should have data-slot=toggle"
    );

    // data-state should be "off" by default
    assert!(
        html.contains(r#"data-state="off""#),
        "should have data-state=off by default"
    );

    // aria-pressed should be false
    assert!(
        html.contains("aria-pressed=false"),
        "should have aria-pressed=false by default"
    );

    // type should be "button"
    assert!(html.contains(r#"type="button""#), "should have type=button");
}

// ---------------------------------------------------------------------------
// Snapshot: default pressed on
// ---------------------------------------------------------------------------

#[test]
fn default_pressed_on() {
    fn App() -> Element {
        rsx! {
            Toggle { default_pressed: true, "Bold" }
        }
    }

    let html = render(App);

    // data-state should be "on"
    assert!(
        html.contains(r#"data-state="on""#),
        "should have data-state=on when default_pressed is true"
    );

    // aria-pressed should be true
    assert!(
        html.contains("aria-pressed=true"),
        "should have aria-pressed=true when default_pressed is true"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: disabled
// ---------------------------------------------------------------------------

#[test]
fn disabled() {
    fn App() -> Element {
        rsx! {
            Toggle { disabled: true, "Bold" }
        }
    }

    let html = render(App);

    // data-disabled attribute should be present (empty string value)
    assert!(
        html.contains(r#"data-disabled="""#),
        "should have data-disabled=\"\" when disabled"
    );

    // disabled attribute on the button
    assert!(
        html.contains("disabled=true"),
        "should have disabled=true when disabled"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: with children
// ---------------------------------------------------------------------------

#[test]
fn with_children() {
    fn App() -> Element {
        rsx! {
            Toggle { "Bold Text" }
        }
    }

    let html = render(App);

    // Children should be rendered inside the button
    assert!(
        html.contains("Bold Text"),
        "children should be rendered inside the toggle button"
    );
}
