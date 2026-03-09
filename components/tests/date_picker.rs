//! SSR snapshot tests for the styled DatePicker component.
//!
//! DatePicker relies on popover/portal internals that make full SSR rendering
//! difficult. These tests verify that the module exports compile and that the
//! outermost DatePicker wrapper renders its data-slot correctly.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::date_picker::*;

/// Render a component to an HTML string via SSR, with normalized IDs.
fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

// ---------------------------------------------------------------------------
// Verify module exports compile — DatePickerProps type exists
// ---------------------------------------------------------------------------

#[test]
fn date_picker_exports_compile() {
    // This test verifies that the styled module re-exports compile correctly.
    // DatePickerProps is re-exported from the primitive via the styled layer.
    fn _assert_type_exists(_: DatePickerProps) {}

    // DatePicker renders its root div with data-slot="date-picker"
    fn App() -> Element {
        rsx! {
            DatePicker {
                "placeholder child"
            }
        }
    }

    let html = render(App);
    eprintln!("=== date_picker_exports_compile ===\n{html}\n");

    assert!(
        html.contains(r#"data-slot="date-picker""#),
        "DatePicker root should have data-slot=\"date-picker\""
    );
    assert!(
        html.contains(r#"role="group""#),
        "DatePicker root should have role=\"group\""
    );
    assert!(
        html.contains(r#"aria-label="Date""#),
        "DatePicker root should have aria-label=\"Date\""
    );
}
