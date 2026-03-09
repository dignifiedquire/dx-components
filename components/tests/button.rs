//! SSR snapshot tests for the styled button (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::button::*;

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
fn button_base_classes() {
    fn App() -> Element {
        rsx! { Button { "Click" } }
    }

    let html = render(App);
    eprintln!("=== button_base_classes ===\n{html}\n");

    assert!(
        html.contains("inline-flex"),
        "button should have inline-flex base class: {html}"
    );
    assert!(
        html.contains(r#"data-slot="button""#),
        "should have data-slot: {html}"
    );
}

// ---------------------------------------------------------------------------
// Variants
// ---------------------------------------------------------------------------

#[test]
fn button_default_variant() {
    fn App() -> Element {
        rsx! { Button { "Click" } }
    }

    let html = render(App);

    assert!(
        html.contains("bg-primary"),
        "default variant should have bg-primary: {html}"
    );
}

#[test]
fn button_destructive_variant() {
    fn App() -> Element {
        rsx! { Button { variant: ButtonVariant::Destructive, "Click" } }
    }

    let html = render(App);

    assert!(
        html.contains("bg-destructive"),
        "destructive variant should have bg-destructive: {html}"
    );
}

#[test]
fn button_outline_variant() {
    fn App() -> Element {
        rsx! { Button { variant: ButtonVariant::Outline, "Click" } }
    }

    let html = render(App);

    assert!(
        html.contains("border"),
        "outline variant should have border: {html}"
    );
}

#[test]
fn button_secondary_variant() {
    fn App() -> Element {
        rsx! { Button { variant: ButtonVariant::Secondary, "Click" } }
    }

    let html = render(App);

    assert!(
        html.contains("bg-secondary"),
        "secondary variant should have bg-secondary: {html}"
    );
}

#[test]
fn button_ghost_variant() {
    fn App() -> Element {
        rsx! { Button { variant: ButtonVariant::Ghost, "Click" } }
    }

    let html = render(App);

    assert!(
        html.contains("hover:bg-accent"),
        "ghost variant should have hover:bg-accent: {html}"
    );
}

#[test]
fn button_link_variant() {
    fn App() -> Element {
        rsx! { Button { variant: ButtonVariant::Link, "Click" } }
    }

    let html = render(App);

    assert!(
        html.contains("underline-offset-4"),
        "link variant should have underline-offset-4: {html}"
    );
}

// ---------------------------------------------------------------------------
// Sizes
// ---------------------------------------------------------------------------

#[test]
fn button_size_sm() {
    fn App() -> Element {
        rsx! { Button { size: ButtonSize::Sm, "Click" } }
    }

    let html = render(App);

    assert!(html.contains("h-8"), "sm size should have h-8: {html}");
}

#[test]
fn button_size_lg() {
    fn App() -> Element {
        rsx! { Button { size: ButtonSize::Lg, "Click" } }
    }

    let html = render(App);

    assert!(html.contains("h-10"), "lg size should have h-10: {html}");
}

#[test]
fn button_size_icon() {
    fn App() -> Element {
        rsx! { Button { size: ButtonSize::Icon, "X" } }
    }

    let html = render(App);

    assert!(
        html.contains("size-9"),
        "icon size should have size-9: {html}"
    );
}

// ---------------------------------------------------------------------------
// Disabled
// ---------------------------------------------------------------------------

#[test]
fn button_disabled() {
    fn App() -> Element {
        rsx! { Button { disabled: true, "Click" } }
    }

    let html = render(App);

    assert!(
        html.contains("disabled=true"),
        "disabled button should have disabled=true: {html}"
    );
}

// ---------------------------------------------------------------------------
// Class merge
// ---------------------------------------------------------------------------

#[test]
fn button_class_merge() {
    fn App() -> Element {
        rsx! { Button { class: "my-class", "Click" } }
    }

    let html = render(App);

    assert!(
        html.contains("my-class"),
        "consumer class should be applied: {html}"
    );
}
