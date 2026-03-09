//! SSR snapshot tests for the styled collapsible (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::collapsible::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

// ---------------------------------------------------------------------------
// Base
// ---------------------------------------------------------------------------

#[test]
fn collapsible_base() {
    fn App() -> Element {
        rsx! {
            Collapsible {
                CollapsibleTrigger { "Toggle" }
                CollapsibleContent { "Content" }
            }
        }
    }

    let html = render(App);
    eprintln!("=== collapsible_base ===\n{html}\n");

    assert!(
        html.contains(r#"data-slot="collapsible""#),
        "should have data-slot=collapsible: {html}"
    );
}

// ---------------------------------------------------------------------------
// Trigger slot
// ---------------------------------------------------------------------------

#[test]
fn collapsible_trigger_slot() {
    fn App() -> Element {
        rsx! {
            Collapsible {
                CollapsibleTrigger { "Toggle" }
                CollapsibleContent { "Content" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains(r#"data-slot="collapsible-trigger""#),
        "should have data-slot=collapsible-trigger: {html}"
    );
}

// ---------------------------------------------------------------------------
// Content slot (only renders when open)
// ---------------------------------------------------------------------------

#[test]
fn collapsible_content_slot() {
    fn App() -> Element {
        rsx! {
            Collapsible {
                default_open: true,
                CollapsibleTrigger { "Toggle" }
                CollapsibleContent { "Content" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains(r#"data-slot="collapsible-content""#),
        "should have data-slot=collapsible-content when open: {html}"
    );
}

// ---------------------------------------------------------------------------
// Disabled
// ---------------------------------------------------------------------------

#[test]
fn collapsible_disabled() {
    fn App() -> Element {
        rsx! {
            Collapsible {
                disabled: true,
                CollapsibleTrigger { "Toggle" }
                CollapsibleContent { "Content" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains("data-disabled"),
        "disabled collapsible should have data-disabled: {html}"
    );
}
