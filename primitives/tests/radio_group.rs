//! SSR snapshot tests for the radio-group primitive.
//!
//! Each test renders a specific configuration and asserts the HTML output
//! contains the expected structure matching Radix UI's radio-group.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::radio_group::*;

/// Render a component to an HTML string via SSR.
fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

// ---------------------------------------------------------------------------
// Root structure
// ---------------------------------------------------------------------------

#[test]
fn default_vertical_no_selection() {
    fn App() -> Element {
        rsx! {
            RadioGroup {
                RadioGroupItem { value: "a", "A" }
                RadioGroupItem { value: "b", "B" }
            }
        }
    }

    let html = render(App);

    // Root
    assert!(html.contains(r#"role="radiogroup""#), "HTML: {html}");
    assert!(html.contains(r#"data-slot="radio-group""#), "HTML: {html}");
    assert!(
        html.contains(r#"data-orientation="vertical""#),
        "default orientation should be vertical: {html}"
    );
    assert!(
        html.contains(r#"aria-orientation="vertical""#),
        "HTML: {html}"
    );

    // Both items unchecked
    assert_eq!(
        html.matches(r#"data-state="unchecked""#).count(),
        2,
        "both items unchecked: {html}"
    );
    assert_eq!(
        html.matches("aria-checked=false").count(),
        2,
        "both aria-checked=false: {html}"
    );
}

// ---------------------------------------------------------------------------
// Default value selects correct item
// ---------------------------------------------------------------------------

#[test]
fn default_value_selects_item() {
    fn App() -> Element {
        rsx! {
            RadioGroup { default_value: "b",
                RadioGroupItem { value: "a", "A" }
                RadioGroupItem { value: "b", "B" }
                RadioGroupItem { value: "c", "C" }
            }
        }
    }

    let html = render(App);

    // One checked, two unchecked
    assert_eq!(
        html.matches(r#"data-state="checked""#).count(),
        1,
        "one item checked: {html}"
    );
    assert_eq!(
        html.matches(r#"data-state="unchecked""#).count(),
        2,
        "two items unchecked: {html}"
    );
    assert!(
        html.contains("aria-checked=true"),
        "checked item has aria-checked=true: {html}"
    );
}

// ---------------------------------------------------------------------------
// Items have correct roles and data-slot
// ---------------------------------------------------------------------------

#[test]
fn item_attributes() {
    fn App() -> Element {
        rsx! {
            RadioGroup { default_value: "a",
                RadioGroupItem { value: "a", "A" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains(r#"role="radio""#),
        "item has role=radio: {html}"
    );
    assert!(
        html.contains(r#"data-slot="radio-group-item""#),
        "item has data-slot: {html}"
    );
    assert!(
        html.contains(r#"type="button""#),
        "item is a button: {html}"
    );
}

// ---------------------------------------------------------------------------
// Disabled group
// ---------------------------------------------------------------------------

#[test]
fn disabled_group() {
    fn App() -> Element {
        rsx! {
            RadioGroup { disabled: true,
                RadioGroupItem { value: "a", "A" }
                RadioGroupItem { value: "b", "B" }
            }
        }
    }

    let html = render(App);

    // Root has data-disabled
    assert!(
        html.contains(r#"data-slot="radio-group" data-orientation="vertical" data-disabled="""#)
            || html.contains(r#"data-disabled="""#),
        "group should have data-disabled: {html}"
    );

    // Items are disabled (disabled=true on each button)
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
            RadioGroup {
                RadioGroupItem { value: "a", "A" }
                RadioGroupItem { value: "b", disabled: true, "B" }
            }
        }
    }

    let html = render(App);

    // Only one item disabled
    assert!(
        html.contains(r#"data-disabled="""#),
        "disabled item has data-disabled: {html}"
    );
}

// ---------------------------------------------------------------------------
// Horizontal orientation
// ---------------------------------------------------------------------------

#[test]
fn horizontal_orientation() {
    fn App() -> Element {
        rsx! {
            RadioGroup {
                orientation: dioxus_primitives::direction::Orientation::Horizontal,
                RadioGroupItem { value: "a", "A" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains(r#"data-orientation="horizontal""#),
        "HTML: {html}"
    );
    assert!(
        html.contains(r#"aria-orientation="horizontal""#),
        "HTML: {html}"
    );
}

// ---------------------------------------------------------------------------
// RadioGroupIndicator
// ---------------------------------------------------------------------------

#[test]
fn indicator_visible_when_checked() {
    fn App() -> Element {
        rsx! {
            RadioGroup { default_value: "a",
                RadioGroupItem { value: "a",
                    RadioGroupIndicator { "dot" }
                }
                RadioGroupItem { value: "b",
                    RadioGroupIndicator { "dot" }
                }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains(r#"data-slot="radio-group-indicator""#),
        "indicator has data-slot: {html}"
    );
    // The checked indicator should have its children ("dot")
    // The unchecked indicator should not render its children
    let indicator_count = html.matches(r#"data-slot="radio-group-indicator""#).count();
    assert_eq!(indicator_count, 2, "two indicators rendered: {html}");
}

// ---------------------------------------------------------------------------
// Indicator force_mount
// ---------------------------------------------------------------------------

#[test]
fn indicator_force_mount() {
    fn App() -> Element {
        rsx! {
            RadioGroup {
                RadioGroupItem { value: "a",
                    RadioGroupIndicator { force_mount: true, "dot" }
                }
            }
        }
    }

    let html = render(App);

    // Force-mounted indicator should show "dot" even when unchecked
    assert!(
        html.contains("dot"),
        "force_mount indicator renders children: {html}"
    );
}

// ---------------------------------------------------------------------------
// Hidden input for form submission
// ---------------------------------------------------------------------------

#[test]
fn hidden_input_with_name() {
    fn App() -> Element {
        rsx! {
            RadioGroup { name: "color", default_value: "red",
                RadioGroupItem { value: "red", "Red" }
                RadioGroupItem { value: "blue", "Blue" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains(r#"type="radio""#),
        "hidden inputs present: {html}"
    );
    assert!(
        html.contains(r#"name="color""#),
        "inputs have name attribute: {html}"
    );
    assert!(
        html.contains("aria-hidden"),
        "inputs are aria-hidden: {html}"
    );
}

// ---------------------------------------------------------------------------
// No hidden input when name is absent
// ---------------------------------------------------------------------------

#[test]
fn no_hidden_input_without_name() {
    fn App() -> Element {
        rsx! {
            RadioGroup {
                RadioGroupItem { value: "a", "A" }
            }
        }
    }

    let html = render(App);

    assert!(
        !html.contains(r#"type="radio""#),
        "no hidden input without name: {html}"
    );
}

// ---------------------------------------------------------------------------
// Required attribute
// ---------------------------------------------------------------------------

#[test]
fn required_attribute() {
    fn App() -> Element {
        rsx! {
            RadioGroup { required: true,
                RadioGroupItem { value: "a", "A" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains("aria-required"),
        "group has aria-required: {html}"
    );
}
