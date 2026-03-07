//! SSR snapshot tests for the toolbar primitive.
//!
//! Each test renders a specific configuration and asserts the HTML output
//! contains the expected structure matching Radix UI's toolbar.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::toolbar::*;

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
fn root_attributes() {
    fn App() -> Element {
        rsx! {
            Toolbar {
                ToolbarButton { "Bold" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains(r#"role="toolbar""#),
        "root has role=toolbar: {html}"
    );
    assert!(
        html.contains(r#"data-slot="toolbar""#),
        "root has data-slot: {html}"
    );
    assert!(
        html.contains(r#"data-orientation="horizontal""#),
        "default horizontal: {html}"
    );
    assert!(
        html.contains(r#"aria-orientation="horizontal""#),
        "HTML: {html}"
    );
}

// ---------------------------------------------------------------------------
// ToolbarButton
// ---------------------------------------------------------------------------

#[test]
fn button_attributes() {
    fn App() -> Element {
        rsx! {
            Toolbar {
                ToolbarButton { "Click me" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains(r#"data-slot="toolbar-button""#),
        "button has data-slot: {html}"
    );
    assert!(
        html.contains(r#"type="button""#),
        "button has type=button: {html}"
    );
    assert!(html.contains("Click me"), "button has children: {html}");
}

// ---------------------------------------------------------------------------
// ToolbarButton disabled
// ---------------------------------------------------------------------------

#[test]
fn button_disabled() {
    fn App() -> Element {
        rsx! {
            Toolbar {
                ToolbarButton { disabled: true, "Disabled" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains("disabled=true"), "disabled button: {html}");
    assert!(
        html.contains(r#"data-disabled="""#),
        "button has data-disabled: {html}"
    );
}

// ---------------------------------------------------------------------------
// ToolbarSeparator
// ---------------------------------------------------------------------------

#[test]
fn separator_inverts_orientation() {
    fn App() -> Element {
        rsx! {
            Toolbar {
                ToolbarButton { "A" }
                ToolbarSeparator {}
                ToolbarButton { "B" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains(r#"data-slot="toolbar-separator""#),
        "separator has data-slot: {html}"
    );
    // Horizontal toolbar → vertical separator
    assert!(
        html.contains(r#"data-slot="toolbar-separator" data-orientation="vertical""#)
            || (html.contains(r#"data-slot="toolbar-separator""#)
                && html.contains(r#"aria-orientation="vertical""#)),
        "separator should be vertical in horizontal toolbar: {html}"
    );
    assert!(
        html.contains(r#"role="separator""#),
        "separator has role: {html}"
    );
}

// ---------------------------------------------------------------------------
// Vertical toolbar → horizontal separator
// ---------------------------------------------------------------------------

#[test]
fn vertical_toolbar_horizontal_separator() {
    fn App() -> Element {
        rsx! {
            Toolbar {
                orientation: dioxus_primitives::direction::Orientation::Vertical,
                ToolbarButton { "A" }
                ToolbarSeparator {}
                ToolbarButton { "B" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains(r#"data-orientation="vertical""#),
        "toolbar is vertical: {html}"
    );
    // Check separator is horizontal
    // Find the separator's data-orientation
    let sep_idx = html.find(r#"data-slot="toolbar-separator""#).unwrap();
    let sep_html = &html[sep_idx..];
    assert!(
        sep_html.contains(r#"data-orientation="horizontal""#),
        "separator should be horizontal in vertical toolbar: {sep_html}"
    );
}

// ---------------------------------------------------------------------------
// ToolbarLink
// ---------------------------------------------------------------------------

#[test]
fn link_attributes() {
    fn App() -> Element {
        rsx! {
            Toolbar {
                ToolbarLink { "Docs" }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains(r#"data-slot="toolbar-link""#),
        "link has data-slot: {html}"
    );
    assert!(html.contains("Docs"), "link has children: {html}");
}

// ---------------------------------------------------------------------------
// ToolbarToggleGroup
// ---------------------------------------------------------------------------

#[test]
fn toggle_group_embedded() {
    fn App() -> Element {
        rsx! {
            Toolbar {
                ToolbarToggleGroup {
                    type_: dioxus_primitives::toggle_group::ToggleGroupType::Single,
                    default_value: vec!["a".to_string()],
                    dioxus_primitives::toggle_group::ToggleGroupItem { value: "a", "A" }
                    dioxus_primitives::toggle_group::ToggleGroupItem { value: "b", "B" }
                }
            }
        }
    }

    let html = render(App);

    assert!(
        html.contains(r#"role="toolbar""#),
        "root is toolbar: {html}"
    );
    assert!(
        html.contains(r#"data-slot="toggle-group""#),
        "toggle group embedded: {html}"
    );
    assert!(
        html.contains(r#"data-slot="toggle-group-item""#),
        "toggle items present: {html}"
    );
    assert!(
        html.contains(r#"data-state="on""#),
        "selected item is on: {html}"
    );
}
