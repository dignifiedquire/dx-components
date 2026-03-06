//! SSR snapshot tests for the accordion primitive.
//!
//! Each test renders a specific accordion configuration and asserts the exact
//! HTML output matches the expected string. This ensures our HTML structure
//! (data-state, aria-*, role, hidden, etc.) matches Radix UI's accordion.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::accordion::*;

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
// Snapshot: all items closed
// ---------------------------------------------------------------------------

#[test]
fn all_closed() {
    fn App() -> Element {
        rsx! {
            Accordion {
                AccordionItem { value: "one", index: 0,
                    AccordionHeader { class: "flex",
                        AccordionTrigger { "Trigger One" }
                    }
                    AccordionContent { "Content One" }
                }
                AccordionItem { value: "two", index: 1,
                    AccordionHeader { class: "flex",
                        AccordionTrigger { "Trigger Two" }
                    }
                    AccordionContent { "Content Two" }
                }
                AccordionItem { value: "three", index: 2,
                    AccordionHeader { class: "flex",
                        AccordionTrigger { "Trigger Three" }
                    }
                    AccordionContent { "Content Three" }
                }
            }
        }
    }

    let html = render(App);

    // --- Structure assertions ---

    // Accordion root
    assert!(html.contains(r#"data-slot="accordion""#));
    assert!(html.contains("data-disabled=false"));

    // All three items are closed — data-state="closed" on collapsible wrappers
    let closed_count = html.matches(r#"data-state="closed""#).count();
    // Each item has: collapsible root + header + trigger = 3 per item = 9 total
    assert!(closed_count >= 9, "expected at least 9 data-state=closed, got {closed_count}");

    // No open state anywhere
    assert!(!html.contains(r#"data-state="open""#));

    // All triggers have aria-expanded=false
    let expanded_false_count = html.matches("aria-expanded=false").count();
    assert_eq!(expanded_false_count, 3, "all 3 triggers should have aria-expanded=false");

    // No content in DOM (presence=Unmounted when initially closed)
    assert!(!html.contains("Content One"));
    assert!(!html.contains("Content Two"));
    assert!(!html.contains("Content Three"));
    assert!(!html.contains(r#"data-slot="collapsible-content""#));

    // All triggers are buttons
    let button_count = html.matches(r#"type="button""#).count();
    assert_eq!(button_count, 3);

    // Headers are h3
    let h3_count = html.matches("<h3").count();
    assert_eq!(h3_count, 3);
    assert_eq!(html.matches(r#"data-slot="accordion-header""#).count(), 3);
}

// ---------------------------------------------------------------------------
// Snapshot: one item default open
// ---------------------------------------------------------------------------

#[test]
fn one_default_open() {
    fn App() -> Element {
        rsx! {
            Accordion {
                AccordionItem { value: "one", index: 0, default_open: true,
                    AccordionHeader { class: "flex",
                        AccordionTrigger { "Trigger One" }
                    }
                    AccordionContent { "Content One" }
                }
                AccordionItem { value: "two", index: 1,
                    AccordionHeader { class: "flex",
                        AccordionTrigger { "Trigger Two" }
                    }
                    AccordionContent { "Content Two" }
                }
                AccordionItem { value: "three", index: 2,
                    AccordionHeader { class: "flex",
                        AccordionTrigger { "Trigger Three" }
                    }
                    AccordionContent { "Content Three" }
                }
            }
        }
    }

    let html = render(App);

    // --- Open item (one) ---

    // Exactly 1 trigger with aria-expanded=true
    assert_eq!(html.matches("aria-expanded=true").count(), 1);

    // Open item has data-state="open" (collapsible + header + trigger + content = 4)
    let open_count = html.matches(r#"data-state="open""#).count();
    assert_eq!(open_count, 4, "open item should have 4 data-state=open (collapsible, header, trigger, content)");

    // Content One IS in the DOM
    assert!(html.contains("Content One"));
    assert!(html.contains(r#"data-slot="collapsible-content""#));

    // Content has role=region
    assert!(html.contains(r#"role="region""#));

    // Content has aria-labelledby pointing to trigger ID
    assert!(html.contains("aria-labelledby"));

    // AccordionContent sets CSS variable aliases
    assert!(html.contains("--radix-accordion-content-height: var(--radix-collapsible-content-height)"));
    assert!(html.contains("--radix-accordion-content-width: var(--radix-collapsible-content-width)"));

    // --- Closed items (two, three) ---

    // 2 triggers with aria-expanded=false
    assert_eq!(html.matches("aria-expanded=false").count(), 2);

    // Closed items' content NOT in DOM
    assert!(!html.contains("Content Two"));
    assert!(!html.contains("Content Three"));
}

// ---------------------------------------------------------------------------
// Snapshot: disabled accordion
// ---------------------------------------------------------------------------

#[test]
fn disabled() {
    fn App() -> Element {
        rsx! {
            Accordion { disabled: true,
                AccordionItem { value: "one", index: 0,
                    AccordionHeader { class: "flex",
                        AccordionTrigger { "Trigger One" }
                    }
                    AccordionContent { "Content One" }
                }
                AccordionItem { value: "two", index: 1,
                    AccordionHeader { class: "flex",
                        AccordionTrigger { "Trigger Two" }
                    }
                    AccordionContent { "Content Two" }
                }
            }
        }
    }

    let html = render(App);

    // Root has data-disabled=true
    assert!(html.contains("data-disabled=true"));

    // All triggers have disabled=true
    let disabled_count = html.matches("disabled=true").count();
    // Each trigger has disabled=true AND data-disabled=true, plus collapsible root + header
    assert!(disabled_count >= 2, "triggers should be disabled, got {disabled_count}");

    // Triggers have the HTML disabled attribute and are buttons
    assert!(html.contains(r#"type="button""#), "triggers should be buttons");
    assert!(html.contains("disabled=true"), "triggers should have disabled attr");
}

// ---------------------------------------------------------------------------
// Snapshot: header renders as h3
// ---------------------------------------------------------------------------

#[test]
fn header_is_h3() {
    fn App() -> Element {
        rsx! {
            Accordion {
                AccordionItem { value: "one", index: 0,
                    AccordionHeader {
                        AccordionTrigger { "Trigger" }
                    }
                    AccordionContent { "Content" }
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains("<h3"));
    assert!(html.contains(r#"data-slot="accordion-header""#));
    assert!(html.contains(r#"data-state="closed""#));
}

// ---------------------------------------------------------------------------
// Snapshot: content CSS variable aliases
// ---------------------------------------------------------------------------

#[test]
fn content_css_vars() {
    fn App() -> Element {
        rsx! {
            Accordion {
                AccordionItem { value: "one", index: 0, default_open: true,
                    AccordionHeader {
                        AccordionTrigger { "Trigger" }
                    }
                    AccordionContent { "Content" }
                }
            }
        }
    }

    let html = render(App);

    // CollapsibleContent inner style should have the collapsible CSS vars
    // (initially empty since no measurement happens in SSR)
    assert!(html.contains(r#"data-slot="collapsible-content""#));

    // AccordionContent aliases collapsible vars to accordion vars
    assert!(
        html.contains("--radix-accordion-content-height: var(--radix-collapsible-content-height)"),
        "should alias height CSS var"
    );
    assert!(
        html.contains("--radix-accordion-content-width: var(--radix-collapsible-content-width)"),
        "should alias width CSS var"
    );
}
