//! SSR snapshot tests for the styled label (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::label::*;

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
fn label_base_classes() {
    fn App() -> Element {
        rsx! { Label { "Email" } }
    }

    let html = render(App);
    assert!(
        html.contains("flex items-center gap-2 text-sm leading-none font-medium select-none"),
        "label should have base classes: {html}"
    );
    assert!(
        html.contains(r#"data-slot="label""#),
        "should have data-slot: {html}"
    );
    assert!(html.contains("Email"), "should render children: {html}");
}

// ---------------------------------------------------------------------------
// Disabled group classes present
// ---------------------------------------------------------------------------

#[test]
fn label_has_disabled_classes() {
    fn App() -> Element {
        rsx! { Label { "Email" } }
    }

    let html = render(App);
    assert!(
        html.contains(
            "group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50"
        ),
        "should have disabled group classes: {html}"
    );
    assert!(
        html.contains("peer-disabled:cursor-not-allowed peer-disabled:opacity-50"),
        "should have peer-disabled classes: {html}"
    );
}

// ---------------------------------------------------------------------------
// Consumer class merge
// ---------------------------------------------------------------------------

#[test]
fn label_consumer_class() {
    fn App() -> Element {
        rsx! { Label { class: "text-red-500", "Error" } }
    }

    let html = render(App);
    assert!(
        html.contains("text-red-500"),
        "consumer class should merge: {html}"
    );
}

// ---------------------------------------------------------------------------
// html_for attribute
// ---------------------------------------------------------------------------

#[test]
fn label_html_for() {
    fn App() -> Element {
        rsx! { Label { html_for: "email-input", "Email" } }
    }

    let html = render(App);
    assert!(
        html.contains(r#"for="email-input""#),
        "should have for attribute: {html}"
    );
}

// ---------------------------------------------------------------------------
// Renders as <label>
// ---------------------------------------------------------------------------

#[test]
fn label_renders_label_element() {
    fn App() -> Element {
        rsx! { Label { "Email" } }
    }

    let html = render(App);
    assert!(html.contains("<label"), "should render as <label>: {html}");
}
