//! SSR snapshot tests for the separator primitive.
//!
//! Each test renders a specific separator configuration and asserts the exact
//! HTML output matches the expected attributes. This ensures our HTML structure
//! (data-slot, data-orientation, role, aria-orientation) matches Radix UI's
//! separator.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::direction::Orientation;
use dioxus_primitives::separator::*;

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
// Snapshot: default horizontal separator
// ---------------------------------------------------------------------------

#[test]
fn default_horizontal() {
    fn App() -> Element {
        rsx! {
            Separator { orientation: Orientation::Horizontal }
        }
    }

    let html = render(App);

    // data-slot
    assert!(
        html.contains(r#"data-slot="separator""#),
        "should have data-slot=separator"
    );

    // data-orientation
    assert!(
        html.contains(r#"data-orientation="horizontal""#),
        "should have data-orientation=horizontal"
    );

    // role
    assert!(
        html.contains(r#"role="separator""#),
        "should have role=separator"
    );

    // aria-orientation defaults to horizontal per WAI-ARIA, so it should NOT
    // be explicitly set.
    assert!(
        !html.contains("aria-orientation"),
        "horizontal separator should not set aria-orientation (it is the WAI-ARIA default)"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: vertical separator
// ---------------------------------------------------------------------------

#[test]
fn vertical() {
    fn App() -> Element {
        rsx! {
            Separator { orientation: Orientation::Vertical }
        }
    }

    let html = render(App);

    // data-orientation
    assert!(
        html.contains(r#"data-orientation="vertical""#),
        "should have data-orientation=vertical"
    );

    // aria-orientation should be explicitly set for vertical
    assert!(
        html.contains(r#"aria-orientation="vertical""#),
        "vertical separator should have aria-orientation=vertical"
    );

    // role
    assert!(
        html.contains(r#"role="separator""#),
        "should have role=separator"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: decorative separator
// ---------------------------------------------------------------------------

#[test]
fn decorative() {
    fn App() -> Element {
        rsx! {
            Separator { decorative: true }
        }
    }

    let html = render(App);

    // role should be "none"
    assert!(
        html.contains(r#"role="none""#),
        "decorative separator should have role=none"
    );

    // No aria-orientation on decorative separators
    assert!(
        !html.contains("aria-orientation"),
        "decorative separator should not have aria-orientation"
    );

    // data-slot still present
    assert!(
        html.contains(r#"data-slot="separator""#),
        "should still have data-slot=separator"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: separator with custom class
// ---------------------------------------------------------------------------

#[test]
fn with_class() {
    fn App() -> Element {
        rsx! {
            Separator { class: "my-class" }
        }
    }

    let html = render(App);

    // Custom class
    assert!(
        html.contains("my-class"),
        "separator should have the custom class"
    );

    // data-slot still present
    assert!(
        html.contains(r#"data-slot="separator""#),
        "should still have data-slot=separator"
    );
}
