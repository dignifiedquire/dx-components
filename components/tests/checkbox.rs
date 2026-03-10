//! SSR snapshot tests for the styled checkbox (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::checkbox::*;

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
fn checkbox_base_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Checkbox {} }
    }

    let html = render(TestApp);
    assert!(
        html.contains("peer size-4 shrink-0 rounded-[4px] border border-input shadow-xs"),
        "checkbox should have base classes: {html}"
    );
    assert!(
        html.contains(r#"data-slot="checkbox""#),
        "should have data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="checkbox""#),
        "should have role=checkbox: {html}"
    );
}

// ---------------------------------------------------------------------------
// Checked state classes
// ---------------------------------------------------------------------------

#[test]
fn checkbox_has_state_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Checkbox {} }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-[state=checked]:border-primary data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground"),
        "should have checked state classes: {html}"
    );
}

// ---------------------------------------------------------------------------
// Indicator with check icon
// ---------------------------------------------------------------------------

#[test]
fn checkbox_indicator_present() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Checkbox {} }
    }

    let html = render(TestApp);
    assert!(
        html.contains(r#"data-slot="checkbox-indicator""#),
        "should have indicator: {html}"
    );
    assert!(
        html.contains("grid place-content-center text-current transition-none"),
        "indicator should have shadcn classes: {html}"
    );
}

#[test]
fn checkbox_has_check_icon_when_checked() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Checkbox { default_checked: CheckedState::Checked } }
    }

    let html = render(TestApp);
    assert!(
        html.contains("<svg"),
        "should contain check icon SVG: {html}"
    );
    assert!(
        html.contains("size-3.5"),
        "check icon should have size-3.5: {html}"
    );
}

// ---------------------------------------------------------------------------
// Consumer class merge
// ---------------------------------------------------------------------------

#[test]
fn checkbox_consumer_class() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Checkbox { class: "my-custom" } }
    }

    let html = render(TestApp);
    assert!(
        html.contains("my-custom"),
        "consumer class should merge: {html}"
    );
}

// ---------------------------------------------------------------------------
// Unchecked state
// ---------------------------------------------------------------------------

#[test]
fn checkbox_unchecked_state() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Checkbox {} }
    }

    let html = render(TestApp);
    assert!(
        html.contains(r#"data-state="unchecked""#),
        "default state should be unchecked: {html}"
    );
    assert!(
        html.contains(r#"aria-checked="false""#),
        "aria-checked should be false: {html}"
    );
}

// ---------------------------------------------------------------------------
// Checked state
// ---------------------------------------------------------------------------

#[test]
fn checkbox_checked_state() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Checkbox { default_checked: CheckedState::Checked } }
    }

    let html = render(TestApp);
    assert!(
        html.contains(r#"data-state="checked""#),
        "should be checked: {html}"
    );
    assert!(
        html.contains(r#"aria-checked="true""#),
        "aria-checked should be true: {html}"
    );
}
