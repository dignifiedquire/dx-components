//! SSR snapshot tests for the styled textarea (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::textarea::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn data_slot_and_base_classes() {
    fn App() -> Element {
        rsx! { Textarea { placeholder: "Type your message" } }
    }

    let html = render(App);
    eprintln!("=== data_slot_and_base_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="textarea""#));
    assert!(html.contains("<textarea"));
    assert!(html.contains("rounded-md border border-input"));
    assert!(html.contains("bg-transparent"));
    assert!(html.contains("min-h-16"));
}

#[test]
fn placeholder_passes_through() {
    fn App() -> Element {
        rsx! { Textarea { placeholder: "Your message" } }
    }

    let html = render(App);
    assert!(html.contains(r#"placeholder="Your message""#));
}

#[test]
fn focus_visible_classes() {
    fn App() -> Element {
        rsx! { Textarea {} }
    }

    let html = render(App);
    assert!(html.contains("focus-visible:border-ring"));
}

#[test]
fn aria_invalid_classes() {
    fn App() -> Element {
        rsx! { Textarea {} }
    }

    let html = render(App);
    assert!(html.contains("aria-invalid:border-destructive"));
}

#[test]
fn consumer_class_merges() {
    fn App() -> Element {
        rsx! { Textarea { class: "min-h-32" } }
    }

    let html = render(App);
    assert!(html.contains("min-h-32"));
}
