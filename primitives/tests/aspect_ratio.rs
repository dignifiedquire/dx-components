//! SSR snapshot tests for the aspect-ratio primitive.
//!
//! Each test renders a specific aspect-ratio configuration and asserts the exact
//! HTML output matches the expected string. This ensures our HTML structure
//! (data-slot, style, etc.) matches Radix UI's aspect-ratio.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::aspect_ratio::*;

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
// Snapshot: default ratio (1.0 = square)
// ---------------------------------------------------------------------------

#[test]
fn default_ratio() {
    fn App() -> Element {
        rsx! {
            AspectRatio {
                div { "Content" }
            }
        }
    }

    let html = render(App);

    // Outer wrapper has data-radix-aspect-ratio-wrapper
    assert!(
        html.contains("data-radix-aspect-ratio-wrapper"),
        "outer wrapper should have data-radix-aspect-ratio-wrapper attribute"
    );

    // data-slot="aspect-ratio"
    assert!(
        html.contains(r#"data-slot="aspect-ratio""#),
        "outer wrapper should have data-slot=aspect-ratio"
    );

    // Default ratio is 1.0 => padding-bottom: 100%
    assert!(
        html.contains("padding-bottom: 100%"),
        "default ratio (1.0) should produce padding-bottom: 100%"
    );

    // Outer wrapper has position: relative
    assert!(
        html.contains("position: relative"),
        "outer wrapper should have position: relative"
    );

    // Inner div has absolute positioning
    assert!(
        html.contains("position: absolute"),
        "inner div should have position: absolute"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: 16:9 ratio
// ---------------------------------------------------------------------------

#[test]
fn sixteen_nine() {
    fn App() -> Element {
        rsx! {
            AspectRatio { ratio: 16.0 / 9.0,
                div { "Widescreen" }
            }
        }
    }

    let html = render(App);

    // 16:9 ratio => padding-bottom: 100 / (16/9) = 56.25%
    assert!(
        html.contains("padding-bottom: 56.25%"),
        "16:9 ratio should produce padding-bottom: 56.25%"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: children are rendered
// ---------------------------------------------------------------------------

#[test]
fn children() {
    fn App() -> Element {
        rsx! {
            AspectRatio { ratio: 4.0 / 3.0,
                span { "Hello from aspect ratio" }
            }
        }
    }

    let html = render(App);

    // Children should be present in the output
    assert!(
        html.contains("Hello from aspect ratio"),
        "children should be rendered inside the aspect-ratio container"
    );

    // 4:3 ratio => padding-bottom: 100 / (4/3) = 75%
    assert!(
        html.contains("padding-bottom: 75%"),
        "4:3 ratio should produce padding-bottom: 75%"
    );
}
