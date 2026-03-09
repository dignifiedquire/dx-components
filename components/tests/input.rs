//! SSR snapshot tests for the styled input (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::input::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn data_slot_and_base_classes() {
    fn App() -> Element {
        rsx! { Input { placeholder: "Email" } }
    }

    let html = render(App);
    eprintln!("=== data_slot_and_base_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="input""#));
    assert!(html.contains("<input"));
    assert!(html.contains("rounded-md border border-input"));
    assert!(html.contains("bg-transparent"));
    assert!(html.contains("h-9"));
}

#[test]
fn placeholder_passes_through() {
    fn App() -> Element {
        rsx! { Input { placeholder: "Enter email" } }
    }

    let html = render(App);
    assert!(html.contains(r#"placeholder="Enter email""#));
}

#[test]
fn focus_visible_classes() {
    fn App() -> Element {
        rsx! { Input {} }
    }

    let html = render(App);
    assert!(html.contains("focus-visible:border-ring"));
}

#[test]
fn aria_invalid_classes() {
    fn App() -> Element {
        rsx! { Input {} }
    }

    let html = render(App);
    assert!(html.contains("aria-invalid:border-destructive"));
}

#[test]
fn consumer_class_merges() {
    fn App() -> Element {
        rsx! { Input { class: "max-w-sm" } }
    }

    let html = render(App);
    assert!(html.contains("max-w-sm"));
}
