//! SSR snapshot tests for the styled toggle (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::toggle::*;

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
fn toggle_base_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Toggle { "Bold" } }
    }

    let html = render(TestApp);
    assert!(
        html.contains("inline-flex cursor-pointer items-center justify-center"),
        "toggle should have base classes: {html}"
    );
    assert!(
        html.contains(r#"data-slot="toggle""#),
        "should have data-slot: {html}"
    );
    assert!(
        html.contains("aria-pressed=false"),
        "should have aria-pressed: {html}"
    );
}

// ---------------------------------------------------------------------------
// Default variant (bg-transparent)
// ---------------------------------------------------------------------------

#[test]
fn toggle_default_variant() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Toggle { "B" } }
    }

    let html = render(TestApp);
    assert!(
        html.contains("bg-transparent"),
        "default variant should have bg-transparent: {html}"
    );
}

// ---------------------------------------------------------------------------
// Outline variant
// ---------------------------------------------------------------------------

#[test]
fn toggle_outline_variant() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Toggle { variant: ToggleVariant::Outline, "B" } }
    }

    let html = render(TestApp);
    assert!(
        html.contains("border-input"),
        "outline variant should have border-input: {html}"
    );
    assert!(
        html.contains("shadow-xs"),
        "outline variant should have shadow-xs: {html}"
    );
}

// ---------------------------------------------------------------------------
// Sizes
// ---------------------------------------------------------------------------

#[test]
fn toggle_size_default() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Toggle { "B" } }
    }

    let html = render(TestApp);
    assert!(html.contains("h-9"), "default size should have h-9: {html}");
}

#[test]
fn toggle_size_sm() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Toggle { size: ToggleSize::Sm, "B" } }
    }

    let html = render(TestApp);
    assert!(html.contains("h-8"), "sm size should have h-8: {html}");
}

#[test]
fn toggle_size_lg() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Toggle { size: ToggleSize::Lg, "B" } }
    }

    let html = render(TestApp);
    assert!(html.contains("h-10"), "lg size should have h-10: {html}");
}

// ---------------------------------------------------------------------------
// Disabled state
// ---------------------------------------------------------------------------

#[test]
fn toggle_disabled() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Toggle { disabled: true, "B" } }
    }

    let html = render(TestApp);
    assert!(
        html.contains("disabled=true"),
        "disabled toggle should have disabled attr: {html}"
    );
}

// ---------------------------------------------------------------------------
// data-state
// ---------------------------------------------------------------------------

#[test]
fn toggle_data_state_off() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Toggle { "B" } }
    }

    let html = render(TestApp);
    assert!(
        html.contains(r#"data-state="off""#),
        "unpressed toggle should have data-state=off: {html}"
    );
}

#[test]
fn toggle_data_state_on() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Toggle { default_pressed: true, "B" } }
    }

    let html = render(TestApp);
    assert!(
        html.contains(r#"data-state="on""#),
        "pressed toggle should have data-state=on: {html}"
    );
}

// ---------------------------------------------------------------------------
// Class merge
// ---------------------------------------------------------------------------

#[test]
fn toggle_class_merge() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Toggle { class: "my-custom", "B" } }
    }

    let html = render(TestApp);
    assert!(
        html.contains("my-custom"),
        "consumer class should be applied: {html}"
    );
}
