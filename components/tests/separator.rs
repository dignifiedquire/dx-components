//! SSR snapshot tests for the styled separator (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::separator::*;

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
fn separator_base_classes() {
    fn App() -> Element {
        rsx! { Separator {} }
    }

    let html = render(App);
    assert!(
        html.contains("shrink-0 bg-border"),
        "separator should have base classes: {html}"
    );
    assert!(
        html.contains(r#"data-slot="separator""#),
        "should have data-slot: {html}"
    );
}

// ---------------------------------------------------------------------------
// Orientation classes
// ---------------------------------------------------------------------------

#[test]
fn separator_horizontal_classes() {
    fn App() -> Element {
        rsx! { Separator {} }
    }

    let html = render(App);
    assert!(
        html.contains("data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full"),
        "should have horizontal classes: {html}"
    );
    assert!(
        html.contains(r#"data-orientation="horizontal""#),
        "default orientation should be horizontal: {html}"
    );
}

#[test]
fn separator_vertical() {
    fn App() -> Element {
        rsx! { Separator { orientation: Orientation::Vertical } }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-orientation="vertical""#),
        "should be vertical: {html}"
    );
    assert!(
        html.contains("data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px"),
        "should have vertical classes: {html}"
    );
}

// ---------------------------------------------------------------------------
// Decorative vs semantic
// ---------------------------------------------------------------------------

#[test]
fn separator_decorative_role() {
    fn App() -> Element {
        rsx! { Separator { decorative: true } }
    }

    let html = render(App);
    assert!(
        html.contains(r#"role="none""#),
        "decorative separator should have role=none: {html}"
    );
}

#[test]
fn separator_semantic_role() {
    fn App() -> Element {
        rsx! { Separator { decorative: false } }
    }

    let html = render(App);
    assert!(
        html.contains(r#"role="separator""#),
        "non-decorative separator should have role=separator: {html}"
    );
}

// ---------------------------------------------------------------------------
// Consumer class merge
// ---------------------------------------------------------------------------

#[test]
fn separator_consumer_class() {
    fn App() -> Element {
        rsx! { Separator { class: "my-4" } }
    }

    let html = render(App);
    assert!(html.contains("my-4"), "consumer class should merge: {html}");
}
