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
    #[component]
    fn TestApp() -> Element {
        rsx! { Spinner {} }
    }

    let html = render(TestApp);
    eprintln!("=== data_slot_and_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="spinner""#));
    assert!(html.contains("<svg"));
    assert!(html.contains("animate-spin"));
    assert!(html.contains("size-4"));
}

#[test]
fn accessibility_attributes() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Spinner {} }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"role="status""#));
    assert!(html.contains(r#"aria-label="Loading""#));
}

#[test]
fn lucide_loader_circle_path() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Spinner {} }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"d="M21 12a9 9 0 1 1-6.219-8.56""#));
}

#[test]
fn consumer_class_merges() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Spinner { class: "size-8" } }
    }

    let html = render(TestApp);
    assert!(html.contains("size-8"));
}
