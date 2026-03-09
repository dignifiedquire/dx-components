//! SSR snapshot tests for the styled toggle group (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::toggle_group::*;

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
fn toggle_group_base_classes() {
    fn App() -> Element {
        rsx! {
            ToggleGroup {
                ToggleGroupItem { value: "a", "A" }
            }
        }
    }

    let html = render(App);
    eprintln!("=== toggle_group_base_classes ===\n{html}\n");

    assert!(
        html.contains("group/toggle-group"),
        "toggle group should have group/toggle-group class: {html}"
    );
    assert!(
        html.contains(r#"data-slot="toggle-group""#),
        "should have data-slot toggle-group: {html}"
    );
}

// ---------------------------------------------------------------------------
// Item classes
// ---------------------------------------------------------------------------

#[test]
fn toggle_group_item_classes() {
    fn App() -> Element {
        rsx! {
            ToggleGroup {
                ToggleGroupItem { value: "a", "A" }
            }
        }
    }

    let html = render(App);
    eprintln!("=== toggle_group_item_classes ===\n{html}\n");

    assert!(
        html.contains(r#"data-slot="toggle-group-item""#),
        "item should have data-slot toggle-group-item: {html}"
    );
    assert!(
        html.contains("inline-flex items-center justify-center"),
        "item should have inline-flex items-center justify-center: {html}"
    );
}

// ---------------------------------------------------------------------------
// Class merge
// ---------------------------------------------------------------------------

#[test]
fn toggle_group_class_merge() {
    fn App() -> Element {
        rsx! {
            ToggleGroup {
                class: "my-group",
                ToggleGroupItem { value: "a", "A" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains("my-group"),
        "consumer class should be applied: {html}"
    );
}
