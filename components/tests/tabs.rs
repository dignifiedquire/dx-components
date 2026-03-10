//! SSR snapshot tests for the styled tabs (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::tabs::*;

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
fn tabs_base_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Tabs {
                default_value: "a".to_string(),
                TabsList {
                    TabsTrigger { value: "a".to_string(), "Tab A" }
                }
                TabsContent { value: "a".to_string(), "Content A" }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== tabs_base_classes ===\n{html}\n");

    assert!(
        html.contains("group/tabs"),
        "tabs should have group/tabs class: {html}"
    );
    assert!(
        html.contains("data-orientation"),
        "should have data-orientation: {html}"
    );
}

// ---------------------------------------------------------------------------
// TabsList classes
// ---------------------------------------------------------------------------

#[test]
fn tabs_list_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Tabs {
                default_value: "a".to_string(),
                TabsList {
                    TabsTrigger { value: "a".to_string(), "Tab A" }
                }
                TabsContent { value: "a".to_string(), "Content A" }
            }
        }
    }

    let html = render(TestApp);

    assert!(
        html.contains(r#"data-slot="tabs-list""#),
        "should have data-slot=tabs-list: {html}"
    );
    assert!(
        html.contains("bg-muted"),
        "tabs list should have bg-muted: {html}"
    );
}

// ---------------------------------------------------------------------------
// TabsTrigger classes
// ---------------------------------------------------------------------------

#[test]
fn tabs_trigger_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Tabs {
                default_value: "a".to_string(),
                TabsList {
                    TabsTrigger { value: "a".to_string(), "Tab A" }
                }
                TabsContent { value: "a".to_string(), "Content A" }
            }
        }
    }

    let html = render(TestApp);

    assert!(
        html.contains(r#"data-slot="tabs-trigger""#),
        "should have data-slot=tabs-trigger: {html}"
    );
    assert!(
        html.contains("text-sm font-medium"),
        "tabs trigger should have text-sm font-medium: {html}"
    );
}

// ---------------------------------------------------------------------------
// TabsContent classes
// ---------------------------------------------------------------------------

#[test]
fn tabs_content_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Tabs {
                default_value: "a".to_string(),
                TabsList {
                    TabsTrigger { value: "a".to_string(), "Tab A" }
                }
                TabsContent { value: "a".to_string(), "Content A" }
            }
        }
    }

    let html = render(TestApp);

    assert!(
        html.contains(r#"data-slot="tabs-content""#),
        "should have data-slot=tabs-content: {html}"
    );
    assert!(
        html.contains("flex-1"),
        "tabs content should have flex-1: {html}"
    );
}

// ---------------------------------------------------------------------------
// Class merge
// ---------------------------------------------------------------------------

#[test]
fn tabs_class_merge() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Tabs {
                default_value: "a".to_string(),
                class: "my-tabs",
                TabsList {
                    TabsTrigger { value: "a".to_string(), "Tab A" }
                }
                TabsContent { value: "a".to_string(), "Content A" }
            }
        }
    }

    let html = render(TestApp);

    assert!(
        html.contains("my-tabs"),
        "consumer class should be applied: {html}"
    );
}
