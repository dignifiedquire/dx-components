//! SSR snapshot tests for the styled progress (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::progress::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

// ---------------------------------------------------------------------------
// Root classes
// ---------------------------------------------------------------------------

#[test]
fn progress_root_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Progress { value: 50.0 } }
    }

    let html = render(TestApp);
    assert!(
        html.contains("relative h-2 w-full overflow-hidden rounded-full bg-primary/20"),
        "root should have shadcn classes: {html}"
    );
    assert!(
        html.contains(r#"data-slot="progress""#),
        "should have data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="progressbar""#),
        "should have role=progressbar: {html}"
    );
}

// ---------------------------------------------------------------------------
// Indicator classes
// ---------------------------------------------------------------------------

#[test]
fn progress_indicator_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Progress { value: 50.0 } }
    }

    let html = render(TestApp);
    assert!(
        html.contains("h-full w-full flex-1 bg-primary transition-all"),
        "indicator should have shadcn classes: {html}"
    );
    assert!(
        html.contains(r#"data-slot="progress-indicator""#),
        "indicator should have data-slot: {html}"
    );
}

// ---------------------------------------------------------------------------
// Transform style
// ---------------------------------------------------------------------------

#[test]
fn progress_transform_at_50() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Progress { value: 50.0 } }
    }

    let html = render(TestApp);
    assert!(
        html.contains("translateX(-50%"),
        "50% progress should translate -50%: {html}"
    );
}

#[test]
fn progress_transform_at_0() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Progress { value: 0.0 } }
    }

    let html = render(TestApp);
    assert!(
        html.contains("translateX(-100%"),
        "0% progress should translate -100%: {html}"
    );
}

#[test]
fn progress_transform_at_100() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Progress { value: 100.0 } }
    }

    let html = render(TestApp);
    assert!(
        html.contains("translateX(-0%") || html.contains("translateX(0%"),
        "100% progress should translate 0%: {html}"
    );
}

// ---------------------------------------------------------------------------
// Consumer class merge
// ---------------------------------------------------------------------------

#[test]
fn progress_consumer_class() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Progress { class: "my-progress", value: 50.0 } }
    }

    let html = render(TestApp);
    assert!(
        html.contains("my-progress"),
        "consumer class should merge: {html}"
    );
}

// ---------------------------------------------------------------------------
// Data attributes
// ---------------------------------------------------------------------------

#[test]
fn progress_data_state() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Progress { value: 50.0 } }
    }

    let html = render(TestApp);
    assert!(
        html.contains(r#"data-state="loading""#),
        "50% progress should be loading state: {html}"
    );
}

#[test]
fn progress_complete_state() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Progress { value: 100.0 } }
    }

    let html = render(TestApp);
    assert!(
        html.contains(r#"data-state="complete""#),
        "100% progress should be complete state: {html}"
    );
}
