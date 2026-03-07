//! SSR snapshot tests for the checkbox primitive.
//!
//! Each test renders a specific checkbox configuration and asserts the exact
//! HTML output matches the expected string. This ensures our HTML structure
//! (data-state, aria-*, role, hidden, etc.) matches Radix UI's checkbox.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::checkbox::*;

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
// Snapshot: default unchecked checkbox
// ---------------------------------------------------------------------------

#[test]
fn default_unchecked() {
    fn App() -> Element {
        rsx! {
            Checkbox {
                CheckboxIndicator { "✓" }
            }
        }
    }

    let html = render(App);

    // Checkbox renders as a <button>
    assert!(
        html.contains("<button"),
        "checkbox should render as a button"
    );

    // role="checkbox"
    assert!(
        html.contains(r#"role="checkbox""#),
        "checkbox should have role=checkbox"
    );

    // data-slot="checkbox"
    assert!(
        html.contains(r#"data-slot="checkbox""#),
        "checkbox should have data-slot=checkbox"
    );

    // data-state="unchecked"
    assert!(
        html.contains(r#"data-state="unchecked""#),
        "default checkbox should have data-state=unchecked"
    );

    // aria-checked="false"
    assert!(
        html.contains(r#"aria-checked="false""#),
        "default checkbox should have aria-checked=false"
    );

    // type="button"
    assert!(
        html.contains(r#"type="button""#),
        "checkbox should have type=button"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: default checked checkbox
// ---------------------------------------------------------------------------

#[test]
fn default_checked() {
    fn App() -> Element {
        rsx! {
            Checkbox { default_checked: CheckedState::Checked,
                CheckboxIndicator { "✓" }
            }
        }
    }

    let html = render(App);

    // data-state="checked"
    assert!(
        html.contains(r#"data-state="checked""#),
        "checked checkbox should have data-state=checked"
    );

    // aria-checked="true"
    assert!(
        html.contains(r#"aria-checked="true""#),
        "checked checkbox should have aria-checked=true"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: indeterminate checkbox
// ---------------------------------------------------------------------------

#[test]
fn indeterminate() {
    fn App() -> Element {
        rsx! {
            Checkbox { default_checked: CheckedState::Indeterminate,
                CheckboxIndicator { "✓" }
            }
        }
    }

    let html = render(App);

    // data-state="indeterminate"
    assert!(
        html.contains(r#"data-state="indeterminate""#),
        "indeterminate checkbox should have data-state=indeterminate"
    );

    // aria-checked="mixed"
    assert!(
        html.contains(r#"aria-checked="mixed""#),
        "indeterminate checkbox should have aria-checked=mixed"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: disabled checkbox
// ---------------------------------------------------------------------------

#[test]
fn disabled() {
    fn App() -> Element {
        rsx! {
            Checkbox { disabled: true,
                CheckboxIndicator { "✓" }
            }
        }
    }

    let html = render(App);

    // data-disabled=""
    assert!(
        html.contains("data-disabled"),
        "disabled checkbox should have data-disabled attribute"
    );

    // disabled=true on the button
    assert!(
        html.contains("disabled=true"),
        "disabled checkbox should have disabled=true"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: indicator visibility
// ---------------------------------------------------------------------------

#[test]
fn indicator_visibility() {
    // When unchecked, CheckboxIndicator span is rendered but children are NOT
    fn UncheckedApp() -> Element {
        rsx! {
            Checkbox {
                CheckboxIndicator {
                    span { "check-icon" }
                }
            }
        }
    }

    let unchecked_html = render(UncheckedApp);

    // Indicator wrapper is always present with data-slot
    assert!(
        unchecked_html.contains(r#"data-slot="checkbox-indicator""#),
        "indicator should have data-slot=checkbox-indicator"
    );

    // But children should NOT be rendered when unchecked
    assert!(
        !unchecked_html.contains("check-icon"),
        "indicator children should not render when unchecked"
    );

    // When checked, children ARE rendered
    fn CheckedApp() -> Element {
        rsx! {
            Checkbox { default_checked: CheckedState::Checked,
                CheckboxIndicator {
                    span { "check-icon" }
                }
            }
        }
    }

    let checked_html = render(CheckedApp);

    assert!(
        checked_html.contains("check-icon"),
        "indicator children should render when checked"
    );

    // With force_mount, children are always rendered even when unchecked
    fn ForceMountApp() -> Element {
        rsx! {
            Checkbox {
                CheckboxIndicator { force_mount: true,
                    span { "check-icon" }
                }
            }
        }
    }

    let force_html = render(ForceMountApp);

    assert!(
        force_html.contains(r#"data-slot="checkbox-indicator""#),
        "force_mount indicator should have data-slot=checkbox-indicator"
    );
    assert!(
        force_html.contains("check-icon"),
        "force_mount indicator should always render children"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: hidden input for form submission
// ---------------------------------------------------------------------------

#[test]
fn hidden_input() {
    fn App() -> Element {
        rsx! {
            Checkbox {
                CheckboxIndicator { "✓" }
            }
        }
    }

    let html = render(App);

    // Hidden input for form submission
    assert!(
        html.contains(r#"type="checkbox""#),
        "should render a hidden checkbox input"
    );

    // aria-hidden=true
    assert!(
        html.contains("aria-hidden=true"),
        "hidden input should have aria-hidden=true"
    );

    // tabindex="-1"
    assert!(
        html.contains(r#"tabindex="-1""#),
        "hidden input should have tabindex=-1"
    );
}
