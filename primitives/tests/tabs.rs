//! SSR snapshot tests for the tabs primitive.
//!
//! Each test renders a specific tabs configuration and asserts the HTML output
//! contains the expected structure (data-slot, role, aria-*, data-state, etc.)
//! matching Radix UI's tabs.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::tabs::*;

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
// Snapshot: default tab selected
// ---------------------------------------------------------------------------

#[test]
fn default_tab_selected() {
    fn App() -> Element {
        rsx! {
            Tabs { default_value: "tab1",
                TabsList {
                    TabsTrigger { value: "tab1", "Tab 1" }
                    TabsTrigger { value: "tab2", "Tab 2" }
                }
                TabsContent { value: "tab1", "Content 1" }
                TabsContent { value: "tab2", "Content 2" }
            }
        }
    }

    let html = render(App);

    // Root has data-slot and horizontal orientation (default)
    assert!(html.contains(r#"data-slot="tabs""#), "HTML: {html}");
    assert!(
        html.contains(r#"data-orientation="horizontal""#),
        "tabs should default to horizontal orientation: {html}"
    );

    // TabsList has tablist role and data-slot
    assert!(html.contains(r#"role="tablist""#));
    assert!(html.contains(r#"data-slot="tabs-list""#));

    // Two triggers with role="tab"
    assert_eq!(html.matches(r#"role="tab""#).count(), 2);
    assert_eq!(
        html.matches(r#"data-slot="tabs-trigger""#).count(),
        2,
        "should have 2 tab triggers"
    );

    // First trigger is active, second is inactive
    assert!(html.contains(r#"data-state="active""#));
    assert!(html.contains(r#"data-state="inactive""#));

    // Selected trigger has aria-selected=true
    assert!(html.contains("aria-selected=true"));
    assert!(html.contains("aria-selected=false"));

    // Content panels
    assert_eq!(
        html.matches(r#"role="tabpanel""#).count(),
        2,
        "should have 2 tab panels"
    );
    assert_eq!(
        html.matches(r#"data-slot="tabs-content""#).count(),
        2,
        "should have 2 tab content slots"
    );

    // Active content is visible
    assert!(
        html.contains("Content 1"),
        "active content should be present"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: three tabs
// ---------------------------------------------------------------------------

#[test]
fn three_tabs_second_selected() {
    fn App() -> Element {
        rsx! {
            Tabs { default_value: "b",
                TabsList {
                    TabsTrigger { value: "a", "Alpha" }
                    TabsTrigger { value: "b", "Beta" }
                    TabsTrigger { value: "c", "Gamma" }
                }
                TabsContent { value: "a", "Alpha content" }
                TabsContent { value: "b", "Beta content" }
                TabsContent { value: "c", "Gamma content" }
            }
        }
    }

    let html = render(App);

    // Three triggers
    assert_eq!(html.matches(r#"role="tab""#).count(), 3);

    // Three content panels
    assert_eq!(html.matches(r#"role="tabpanel""#).count(), 3);
}

// ---------------------------------------------------------------------------
// Snapshot: disabled trigger
// ---------------------------------------------------------------------------

#[test]
fn disabled_trigger() {
    fn App() -> Element {
        rsx! {
            Tabs { default_value: "tab1",
                TabsList {
                    TabsTrigger { value: "tab1", "Tab 1" }
                    TabsTrigger { value: "tab2", disabled: true, "Tab 2" }
                }
                TabsContent { value: "tab1", "Content 1" }
                TabsContent { value: "tab2", "Content 2" }
            }
        }
    }

    let html = render(App);

    // Disabled trigger has data-disabled and disabled attribute
    assert!(
        html.contains("data-disabled"),
        "disabled trigger should have data-disabled"
    );
    assert!(
        html.contains("disabled=true"),
        "disabled trigger should have disabled attribute"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: vertical orientation
// ---------------------------------------------------------------------------

#[test]
fn vertical_orientation() {
    fn App() -> Element {
        rsx! {
            Tabs {
                default_value: "tab1",
                orientation: dioxus_primitives::direction::Orientation::Vertical,
                TabsList {
                    TabsTrigger { value: "tab1", "Tab 1" }
                    TabsTrigger { value: "tab2", "Tab 2" }
                }
                TabsContent { value: "tab1", "Content 1" }
                TabsContent { value: "tab2", "Content 2" }
            }
        }
    }

    let html = render(App);

    // Root and components should have vertical orientation
    assert!(
        html.contains(r#"data-orientation="vertical""#),
        "should have vertical orientation"
    );
    assert!(
        html.contains(r#"aria-orientation="vertical""#),
        "tablist should have aria-orientation=vertical"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: force_mount content
// ---------------------------------------------------------------------------

#[test]
fn force_mount_content() {
    fn App() -> Element {
        rsx! {
            Tabs { default_value: "tab1",
                TabsList {
                    TabsTrigger { value: "tab1", "Tab 1" }
                    TabsTrigger { value: "tab2", "Tab 2" }
                }
                TabsContent { value: "tab1", "Content 1" }
                TabsContent { value: "tab2", force_mount: true, "Content 2" }
            }
        }
    }

    let html = render(App);

    // Both contents should be present in DOM
    assert!(html.contains("Content 1"));
    assert!(
        html.contains("Content 2"),
        "force_mount content should be in DOM even when inactive"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: ARIA linkage between trigger and content
// ---------------------------------------------------------------------------

#[test]
fn aria_linkage() {
    fn App() -> Element {
        rsx! {
            Tabs { default_value: "tab1",
                TabsList {
                    TabsTrigger { value: "tab1", "Tab 1" }
                }
                TabsContent { value: "tab1", "Content 1" }
            }
        }
    }

    let html = render(App);

    // Trigger has aria-controls pointing to content ID
    assert!(html.contains("aria-controls=\"dxc-ID-content-tab1\""));
    // Content has aria-labelledby pointing to trigger ID
    assert!(html.contains("aria-labelledby=\"dxc-ID-trigger-tab1\""));
}
