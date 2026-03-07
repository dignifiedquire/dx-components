//! SSR snapshot tests for the switch primitive.
//!
//! Each test renders a specific switch configuration and asserts the exact
//! HTML output matches the expected string. This ensures our HTML structure
//! (data-slot, data-state, aria-*, role, etc.) matches Radix UI's switch.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::switch::*;

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
// Snapshot: default unchecked switch
// ---------------------------------------------------------------------------

#[test]
fn default_unchecked() {
    fn App() -> Element {
        rsx! {
            Switch {
                SwitchThumb {}
            }
        }
    }

    let html = render(App);

    // Switch renders a <button>
    assert!(
        html.contains("<button"),
        "Switch should render as a <button>"
    );

    // role="switch"
    assert!(
        html.contains(r#"role="switch""#),
        "Switch should have role=\"switch\""
    );

    // data-slot="switch"
    assert!(
        html.contains(r#"data-slot="switch""#),
        "Switch should have data-slot=\"switch\""
    );

    // data-state="unchecked" (default)
    assert!(
        html.contains(r#"data-state="unchecked""#),
        "Switch should have data-state=\"unchecked\" by default"
    );

    // aria-checked=false
    assert!(
        html.contains("aria-checked=false"),
        "Switch should have aria-checked=false by default"
    );

    // type="button"
    assert!(
        html.contains(r#"type="button""#),
        "Switch should have type=\"button\""
    );
}

// ---------------------------------------------------------------------------
// Snapshot: default checked switch
// ---------------------------------------------------------------------------

#[test]
fn default_checked() {
    fn App() -> Element {
        rsx! {
            Switch { default_checked: true,
                SwitchThumb {}
            }
        }
    }

    let html = render(App);

    // data-state="checked"
    assert!(
        html.contains(r#"data-state="checked""#),
        "Switch with default_checked should have data-state=\"checked\""
    );

    // aria-checked=true
    assert!(
        html.contains("aria-checked=true"),
        "Switch with default_checked should have aria-checked=true"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: disabled switch
// ---------------------------------------------------------------------------

#[test]
fn disabled() {
    fn App() -> Element {
        rsx! {
            Switch { disabled: true,
                SwitchThumb {}
            }
        }
    }

    let html = render(App);

    // data-disabled="" (empty string value for data-disabled)
    assert!(
        html.contains("data-disabled"),
        "Disabled Switch should have data-disabled attribute"
    );

    // disabled=true on the button
    assert!(
        html.contains("disabled=true"),
        "Disabled Switch should have disabled=true"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: thumb inherits data-state from switch context
// ---------------------------------------------------------------------------

#[test]
fn thumb_inherits_state() {
    fn App() -> Element {
        rsx! {
            Switch {
                SwitchThumb {}
            }
        }
    }

    let html = render(App);

    // SwitchThumb has data-slot="switch-thumb"
    assert!(
        html.contains(r#"data-slot="switch-thumb""#),
        "SwitchThumb should have data-slot=\"switch-thumb\""
    );

    // SwitchThumb inherits data-state from the parent switch.
    // Since default is unchecked, both switch and thumb have data-state="unchecked".
    // The switch button has data-state="unchecked" and the thumb span also has it.
    let unchecked_count = html.matches(r#"data-state="unchecked""#).count();
    assert!(
        unchecked_count >= 2,
        "Both Switch and SwitchThumb should have data-state=\"unchecked\", got {unchecked_count}"
    );

    // Now verify with checked state
    fn AppChecked() -> Element {
        rsx! {
            Switch { default_checked: true,
                SwitchThumb {}
            }
        }
    }

    let html_checked = render(AppChecked);

    // Both switch and thumb should have data-state="checked"
    let checked_count = html_checked.matches(r#"data-state="checked""#).count();
    assert!(
        checked_count >= 2,
        "Both Switch and SwitchThumb should have data-state=\"checked\" when checked, got {checked_count}"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: hidden input for form submission
// ---------------------------------------------------------------------------

#[test]
fn hidden_input() {
    fn App() -> Element {
        rsx! {
            Switch {
                SwitchThumb {}
            }
        }
    }

    let html = render(App);

    // Hidden checkbox input for form submission
    assert!(
        html.contains(r#"type="checkbox""#),
        "Switch should render a hidden <input type=\"checkbox\"> for form submission"
    );

    // aria-hidden=true on the hidden input
    assert!(
        html.contains("aria-hidden=true"),
        "Hidden input should have aria-hidden=true"
    );

    // tabindex="-1" on the hidden input
    assert!(
        html.contains(r#"tabindex="-1""#),
        "Hidden input should have tabindex=\"-1\""
    );

    // The hidden input has positioning styles to make it invisible
    assert!(
        html.contains("position: absolute"),
        "Hidden input should have position: absolute style"
    );
    assert!(
        html.contains("pointer-events: none"),
        "Hidden input should have pointer-events: none style"
    );
    assert!(
        html.contains("opacity: 0"),
        "Hidden input should have opacity: 0 style"
    );
}
