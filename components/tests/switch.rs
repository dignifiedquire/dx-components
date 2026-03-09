//! SSR snapshot tests for the styled switch (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::switch::*;

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
fn switch_base_classes() {
    fn App() -> Element {
        rsx! { Switch {} }
    }

    let html = render(App);
    assert!(
        html.contains("peer group/switch inline-flex shrink-0 items-center rounded-full border border-transparent shadow-xs"),
        "switch should have base classes: {html}"
    );
    assert!(
        html.contains(r#"data-slot="switch""#),
        "should have data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="switch""#),
        "should have role=switch: {html}"
    );
}

// ---------------------------------------------------------------------------
// Size variants
// ---------------------------------------------------------------------------

#[test]
fn switch_default_size() {
    fn App() -> Element {
        rsx! { Switch {} }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-size="default""#),
        "should have data-size=default: {html}"
    );
}

#[test]
fn switch_sm_size() {
    fn App() -> Element {
        rsx! { Switch { size: SwitchSize::Sm } }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-size="sm""#),
        "should have data-size=sm: {html}"
    );
}

// ---------------------------------------------------------------------------
// Thumb classes
// ---------------------------------------------------------------------------

#[test]
fn switch_thumb_classes() {
    fn App() -> Element {
        rsx! { Switch {} }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="switch-thumb""#),
        "thumb should have data-slot: {html}"
    );
    assert!(
        html.contains(
            "pointer-events-none block rounded-full bg-background ring-0 transition-transform"
        ),
        "thumb should have base classes: {html}"
    );
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

#[test]
fn switch_unchecked_state() {
    fn App() -> Element {
        rsx! { Switch {} }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-state="unchecked""#),
        "default state should be unchecked: {html}"
    );
}

#[test]
fn switch_checked_state() {
    fn App() -> Element {
        rsx! { Switch { default_checked: true } }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-state="checked""#),
        "should be checked: {html}"
    );
}

// ---------------------------------------------------------------------------
// State classes present
// ---------------------------------------------------------------------------

#[test]
fn switch_has_state_classes() {
    fn App() -> Element {
        rsx! { Switch {} }
    }

    let html = render(App);
    assert!(
        html.contains("data-[state=checked]:bg-primary data-[state=unchecked]:bg-input"),
        "should have state variant classes: {html}"
    );
}

// ---------------------------------------------------------------------------
// Consumer class merge
// ---------------------------------------------------------------------------

#[test]
fn switch_consumer_class() {
    fn App() -> Element {
        rsx! { Switch { class: "my-switch" } }
    }

    let html = render(App);
    assert!(
        html.contains("my-switch"),
        "consumer class should merge: {html}"
    );
}
