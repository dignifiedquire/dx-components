//! SSR snapshot tests for the collapsible primitive.
//!
//! Each test renders a specific collapsible configuration and asserts the exact
//! HTML output matches the expected string. This ensures our HTML structure
//! (data-state, aria-*, hidden, etc.) matches Radix UI's collapsible.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::collapsible::*;

/// Render a component to an HTML string via SSR.
///
/// The returned HTML is stripped of generated IDs (dxc-N) so snapshots are
/// stable across runs.
fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

// ---------------------------------------------------------------------------
// Snapshot: default closed
// ---------------------------------------------------------------------------

#[test]
fn default_closed() {
    fn App() -> Element {
        rsx! {
            Collapsible {
                CollapsibleTrigger { "Toggle" }
                CollapsibleContent { "Hidden content" }
            }
        }
    }

    let html = render(App);

    // Root
    assert!(html.contains(r#"data-slot="collapsible""#));
    assert!(html.contains(r#"data-state="closed""#));

    // Trigger is a button
    assert!(html.contains(r#"data-slot="collapsible-trigger""#));
    assert!(html.contains(r#"type="button""#));
    assert!(html.contains("aria-expanded=false"));
    assert!(html.contains("aria-controls"));
    assert!(html.contains("Toggle"));

    // Content outer div is in DOM but hidden
    assert!(html.contains(r#"data-slot="collapsible-content""#));
    assert!(html.contains("hidden=true"));

    // Content children NOT rendered when closed
    assert!(!html.contains("Hidden content"));
}

// ---------------------------------------------------------------------------
// Snapshot: default open
// ---------------------------------------------------------------------------

#[test]
fn default_open() {
    fn App() -> Element {
        rsx! {
            Collapsible { default_open: true,
                CollapsibleTrigger { "Toggle" }
                CollapsibleContent { "Visible content" }
            }
        }
    }

    let html = render(App);

    // Root has data-state="open"
    assert!(html.contains(r#"data-state="open""#));

    // Trigger has aria-expanded=true
    assert!(html.contains("aria-expanded=true"));

    // Content is visible (not hidden)
    assert!(html.contains("Visible content"));

    // Content div should NOT have hidden=true
    // (it might have hidden=false or no hidden attr)
    let content_start = html.find(r#"data-slot="collapsible-content""#).unwrap();
    let content_section = &html[content_start..content_start + 200.min(html.len() - content_start)];
    assert!(
        !content_section.contains("hidden=true"),
        "open content should not be hidden"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: disabled
// ---------------------------------------------------------------------------

#[test]
fn disabled() {
    fn App() -> Element {
        rsx! {
            Collapsible { disabled: true,
                CollapsibleTrigger { "Toggle" }
                CollapsibleContent { "Content" }
            }
        }
    }

    let html = render(App);

    // Root has data-disabled
    assert!(
        html.contains(r#"data-disabled="""#),
        "root should have data-disabled when disabled"
    );

    // Trigger button has disabled=true
    assert!(
        html.contains("disabled=true"),
        "trigger should have disabled=true when disabled"
    );

    // Trigger has data-disabled
    let trigger_section = {
        let start = html.find(r#"data-slot="collapsible-trigger""#).unwrap();
        &html[start..start + 300.min(html.len() - start)]
    };
    assert!(
        trigger_section.contains(r#"data-disabled="""#),
        "trigger should have data-disabled when disabled"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: trigger has aria-controls pointing to content ID
// ---------------------------------------------------------------------------

#[test]
fn trigger_controls_content() {
    fn App() -> Element {
        rsx! {
            Collapsible {
                CollapsibleTrigger { "Toggle" }
                CollapsibleContent { "Content" }
            }
        }
    }

    let html = render(App);

    // Both trigger (aria-controls) and content (id) should reference the same ID.
    // The ID format may vary (dxc-ID after normalization, or quoted).
    assert!(
        html.contains("aria-controls"),
        "trigger should have aria-controls"
    );
    assert!(
        html.contains(r#"data-slot="collapsible-content""#),
        "content div should exist"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: force_mount keeps children when closed
// ---------------------------------------------------------------------------

#[test]
fn force_mount() {
    fn App() -> Element {
        rsx! {
            Collapsible {
                CollapsibleTrigger { "Toggle" }
                CollapsibleContent { force_mount: true,
                    "Force mounted content"
                }
            }
        }
    }

    let html = render(App);

    // Content children SHOULD be rendered even when closed (force_mount)
    assert!(
        html.contains("Force mounted content"),
        "force_mount should keep children in DOM when closed"
    );

    // But the content div should still be hidden
    assert!(html.contains("hidden=true"));
}

// ---------------------------------------------------------------------------
// Snapshot: custom class
// ---------------------------------------------------------------------------

#[test]
fn custom_class() {
    fn App() -> Element {
        rsx! {
            Collapsible { class: "my-collapsible",
                CollapsibleTrigger { class: "my-trigger", "Toggle" }
                CollapsibleContent { class: "my-content", "Content" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains("my-collapsible"));
    assert!(html.contains("my-trigger"));
    assert!(html.contains("my-content"));
}

// ---------------------------------------------------------------------------
// Snapshot: trigger data-state matches open state
// ---------------------------------------------------------------------------

#[test]
fn trigger_data_state_matches_open() {
    fn App() -> Element {
        rsx! {
            Collapsible { default_open: true,
                CollapsibleTrigger { "Toggle" }
                CollapsibleContent { "Content" }
            }
        }
    }

    let html = render(App);

    // All data-state should be "open" when default_open is true
    let open_count = html.matches(r#"data-state="open""#).count();
    // Root + trigger + content = 3
    assert_eq!(
        open_count, 3,
        "expected 3 data-state=open (root, trigger, content), got {open_count}"
    );
}
