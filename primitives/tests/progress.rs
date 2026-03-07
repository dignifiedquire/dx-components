//! SSR snapshot tests for the progress primitive.
//!
//! Each test renders a specific progress configuration and asserts the exact
//! HTML output matches the expected string. This ensures our HTML structure
//! (data-state, aria-*, role, etc.) matches Radix UI's progress.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::progress::*;

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
// Snapshot: indeterminate (no value)
// ---------------------------------------------------------------------------

#[test]
fn indeterminate() {
    fn App() -> Element {
        let value = use_signal(|| None::<f64>);
        rsx! {
            Progress { value: value,
                ProgressIndicator {}
            }
        }
    }

    let html = render(App);

    // Progress root attributes
    assert!(
        html.contains(r#"data-slot="progress""#),
        "should have data-slot=progress"
    );
    assert!(
        html.contains(r#"role="progressbar""#),
        "should have role=progressbar"
    );
    assert!(
        html.contains(r#"data-state="indeterminate""#),
        "should have data-state=indeterminate"
    );
    assert!(
        html.contains(r#"aria-valuemin="0""#),
        "should have aria-valuemin=0"
    );
    assert!(
        html.contains(r#"aria-valuemax="100""#),
        "should have aria-valuemax=100"
    );

    // No aria-valuenow when indeterminate
    assert!(
        !html.contains("aria-valuenow"),
        "indeterminate progress should not have aria-valuenow"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: with value (loading state)
// ---------------------------------------------------------------------------

#[test]
fn with_value() {
    fn App() -> Element {
        let value = use_signal(|| Some(50.0f64));
        rsx! {
            Progress { value: value,
                ProgressIndicator {}
            }
        }
    }

    let html = render(App);

    // data-state should be "loading" when value < max
    assert!(
        html.contains(r#"data-state="loading""#),
        "should have data-state=loading when value < max"
    );

    // aria-valuenow should be present with value
    assert!(
        html.contains(r#"aria-valuenow="50""#),
        "should have aria-valuenow=50"
    );

    // aria-valuetext should show percentage
    assert!(
        html.contains(r#"aria-valuetext="50%""#),
        "should have aria-valuetext=50%"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: complete (value == max)
// ---------------------------------------------------------------------------

#[test]
fn complete() {
    fn App() -> Element {
        let value = use_signal(|| Some(100.0f64));
        rsx! {
            Progress { value: value, max: 100.0,
                ProgressIndicator {}
            }
        }
    }

    let html = render(App);

    // data-state should be "complete" when value >= max
    assert!(
        html.contains(r#"data-state="complete""#),
        "should have data-state=complete when value >= max"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: indicator inherits context
// ---------------------------------------------------------------------------

#[test]
fn indicator_inherits() {
    fn App() -> Element {
        let value = use_signal(|| Some(75.0f64));
        rsx! {
            Progress { value: value,
                ProgressIndicator {}
            }
        }
    }

    let html = render(App);

    // ProgressIndicator should have its own data-slot
    assert!(
        html.contains(r#"data-slot="progress-indicator""#),
        "indicator should have data-slot=progress-indicator"
    );

    // ProgressIndicator inherits data-state from context
    // Both root and indicator should have data-state="loading"
    let loading_count = html.matches(r#"data-state="loading""#).count();
    assert_eq!(
        loading_count, 2,
        "both root and indicator should have data-state=loading"
    );

    // ProgressIndicator inherits data-value from context
    let data_value_count = html.matches(r#"data-value="75""#).count();
    assert_eq!(
        data_value_count, 2,
        "both root and indicator should have data-value=75"
    );

    // ProgressIndicator inherits data-max from context
    let data_max_count = html.matches(r#"data-max="100""#).count();
    assert_eq!(
        data_max_count, 2,
        "both root and indicator should have data-max=100"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: custom max
// ---------------------------------------------------------------------------

#[test]
fn custom_max() {
    fn App() -> Element {
        let value = use_signal(|| Some(100.0f64));
        rsx! {
            Progress { value: value, max: 200.0,
                ProgressIndicator {}
            }
        }
    }

    let html = render(App);

    // aria-valuemax should reflect the custom max
    assert!(
        html.contains(r#"aria-valuemax="200""#),
        "should have aria-valuemax=200"
    );

    // data-max should reflect the custom max
    assert!(
        html.contains(r#"data-max="200""#),
        "should have data-max=200"
    );

    // data-state should be "loading" since 100 < 200
    assert!(
        html.contains(r#"data-state="loading""#),
        "should have data-state=loading when value < custom max"
    );
}
