//! SSR snapshot tests for the styled spinner (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::spinner::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn data_slot_and_classes() {
    fn App() -> Element {
        rsx! { Spinner {} }
    }

    let html = render(App);
    eprintln!("=== data_slot_and_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="spinner""#));
    assert!(html.contains("<svg"));
    assert!(html.contains("animate-spin"));
    assert!(html.contains("size-4"));
}

#[test]
fn accessibility_attributes() {
    fn App() -> Element {
        rsx! { Spinner {} }
    }

    let html = render(App);
    assert!(html.contains(r#"role="status""#));
    assert!(html.contains(r#"aria-label="Loading""#));
}

#[test]
fn lucide_loader_circle_path() {
    fn App() -> Element {
        rsx! { Spinner {} }
    }

    let html = render(App);
    assert!(html.contains(r#"d="M21 12a9 9 0 1 1-6.219-8.56""#));
}

#[test]
fn consumer_class_merges() {
    fn App() -> Element {
        rsx! { Spinner { class: "size-8" } }
    }

    let html = render(App);
    assert!(html.contains("size-8"));
}
