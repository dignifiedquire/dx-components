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
    #[component]
    fn TestApp() -> Element {
        rsx! { Input { placeholder: "Email" } }
    }

    let html = render(TestApp);
    eprintln!("=== data_slot_and_base_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="input""#));
    assert!(html.contains("<input"));
    assert!(html.contains("rounded-md border border-input"));
    assert!(html.contains("bg-transparent"));
    assert!(html.contains("h-9"));
}

#[test]
fn placeholder_passes_through() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Input { placeholder: "Enter email" } }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"placeholder="Enter email""#));
}

#[test]
fn focus_visible_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Input {} }
    }

    let html = render(TestApp);
    assert!(html.contains("focus-visible:border-ring"));
}

#[test]
fn aria_invalid_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Input {} }
    }

    let html = render(TestApp);
    assert!(html.contains("aria-invalid:border-destructive"));
}

#[test]
fn consumer_class_merges() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Input { class: "max-w-sm" } }
    }

    let html = render(TestApp);
    assert!(html.contains("max-w-sm"));
}
