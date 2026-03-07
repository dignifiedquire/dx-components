//! SSR snapshot tests for the toggle-group primitive.
//!
//! Each test renders a specific configuration and asserts the HTML output
//! contains the expected structure matching Radix UI's toggle-group.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::toggle_group::*;

/// Render a component to an HTML string via SSR.
fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

// ---------------------------------------------------------------------------
// Root structure — single mode
// ---------------------------------------------------------------------------

#[test]
fn single_mode_root() {
    fn App() -> Element {
        rsx! {
            ToggleGroup { type_: ToggleGroupType::Single,
                ToggleGroupItem { value: "a", "A" }
                ToggleGroupItem { value: "b", "B" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"role="group""#), "HTML: {html}");
    assert!(html.contains(r#"data-slot="toggle-group""#), "HTML: {html}");
    assert!(
        html.contains(r#"data-orientation="horizontal""#),
        "default horizontal: {html}"
    );
}

// ---------------------------------------------------------------------------
// Single mode — items have radio semantics
// ---------------------------------------------------------------------------

#[test]
fn single_mode_items_radio_semantics() {
    fn App() -> Element {
        rsx! {
            ToggleGroup { type_: ToggleGroupType::Single, default_value: vec!["a".to_string()],
                ToggleGroupItem { value: "a", "A" }
                ToggleGroupItem { value: "b", "B" }
            }
        }
    }

    let html = render(App);

    // Items in single mode have role="radio" + aria-checked
    assert!(
        html.contains(r#"role="radio""#),
        "single mode items have role=radio: {html}"
    );
    assert!(
        html.contains("aria-checked=true"),
        "pressed item has aria-checked=true: {html}"
    );
    assert!(
        html.contains("aria-checked=false"),
        "unpressed item has aria-checked=false: {html}"
    );
}

// ---------------------------------------------------------------------------
// Multiple mode — items have toggle semantics
// ---------------------------------------------------------------------------

#[test]
fn multiple_mode_items_toggle_semantics() {
    fn App() -> Element {
        rsx! {
            ToggleGroup {
                type_: ToggleGroupType::Multiple,
                default_value: vec!["a".to_string(), "c".to_string()],
                ToggleGroupItem { value: "a", "A" }
                ToggleGroupItem { value: "b", "B" }
                ToggleGroupItem { value: "c", "C" }
            }
        }
    }

    let html = render(App);

    // Items in multiple mode have aria-pressed (no role="radio")
    assert!(
        !html.contains(r#"role="radio""#),
        "multiple mode items should NOT have role=radio: {html}"
    );
    assert!(
        html.contains("aria-pressed=true"),
        "pressed items have aria-pressed=true: {html}"
    );
    assert!(
        html.contains("aria-pressed=false"),
        "unpressed items have aria-pressed=false: {html}"
    );
}

// ---------------------------------------------------------------------------
// Item data attributes
// ---------------------------------------------------------------------------

#[test]
fn item_data_attributes() {
    fn App() -> Element {
        rsx! {
            ToggleGroup {
                type_: ToggleGroupType::Single,
                default_value: vec!["a".to_string()],
                ToggleGroupItem { value: "a", "A" }
                ToggleGroupItem { value: "b", "B" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains(r#"data-slot="toggle-group-item""#),
        "items have data-slot: {html}"
    );
    assert!(
        html.contains(r#"data-state="on""#),
        "pressed item has data-state=on: {html}"
    );
    assert!(
        html.contains(r#"data-state="off""#),
        "unpressed item has data-state=off: {html}"
    );
    assert!(
        html.contains(r#"type="button""#),
        "items are buttons: {html}"
    );
}

// ---------------------------------------------------------------------------
// Disabled group
// ---------------------------------------------------------------------------

#[test]
fn disabled_group() {
    fn App() -> Element {
        rsx! {
            ToggleGroup { type_: ToggleGroupType::Single, disabled: true,
                ToggleGroupItem { value: "a", "A" }
                ToggleGroupItem { value: "b", "B" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains(r#"data-disabled="""#),
        "group has data-disabled: {html}"
    );
    assert_eq!(
        html.matches("disabled=true").count(),
        2,
        "both items disabled: {html}"
    );
}

// ---------------------------------------------------------------------------
// Disabled individual item
// ---------------------------------------------------------------------------

#[test]
fn disabled_item() {
    fn App() -> Element {
        rsx! {
            ToggleGroup { type_: ToggleGroupType::Single,
                ToggleGroupItem { value: "a", "A" }
                ToggleGroupItem { value: "b", disabled: true, "B" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains("disabled=true"),
        "disabled item is disabled: {html}"
    );
    assert!(
        html.contains(r#"data-disabled="""#),
        "disabled item has data-disabled: {html}"
    );
}

// ---------------------------------------------------------------------------
// Vertical orientation
// ---------------------------------------------------------------------------

#[test]
fn vertical_orientation() {
    fn App() -> Element {
        rsx! {
            ToggleGroup {
                type_: ToggleGroupType::Single,
                orientation: dioxus_primitives::direction::Orientation::Vertical,
                ToggleGroupItem { value: "a", "A" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains(r#"data-orientation="vertical""#),
        "HTML: {html}"
    );
}

// ---------------------------------------------------------------------------
// No default value — all items off
// ---------------------------------------------------------------------------

#[test]
fn no_default_all_off() {
    fn App() -> Element {
        rsx! {
            ToggleGroup { type_: ToggleGroupType::Multiple,
                ToggleGroupItem { value: "a", "A" }
                ToggleGroupItem { value: "b", "B" }
            }
        }
    }

    let html = render(App);

    assert_eq!(
        html.matches(r#"data-state="off""#).count(),
        2,
        "all items off: {html}"
    );
    assert!(!html.contains(r#"data-state="on""#), "no items on: {html}");
}
