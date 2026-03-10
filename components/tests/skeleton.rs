//! SSR snapshot tests for the styled skeleton (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::skeleton::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn data_slot_and_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Skeleton { class: "h-4 w-[250px]" } }
    }

    let html = render(TestApp);
    eprintln!("=== data_slot_and_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="skeleton""#));
    assert!(html.contains("<div"));
    assert!(html.contains("animate-pulse"));
    assert!(html.contains("rounded-md"));
    assert!(html.contains("bg-accent"));
}

#[test]
fn consumer_class_merges() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Skeleton { class: "h-12 w-12 rounded-full" } }
    }

    let html = render(TestApp);
    assert!(html.contains("h-12 w-12"));
    // rounded-full should override rounded-md via tw_merge
    assert!(html.contains("rounded-full"));
}

#[test]
fn skeleton_renders_as_div() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Skeleton {} }
    }

    let html = render(TestApp);
    assert!(html.contains("<div"), "renders as div: {html}");
    assert!(
        html.contains("data-slot=\"skeleton\""),
        "has data-slot: {html}"
    );
}

#[test]
fn skeleton_custom_dimensions() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Skeleton { class: "h-8 w-32" }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("h-8"), "has custom height: {html}");
    assert!(html.contains("w-32"), "has custom width: {html}");
    assert!(html.contains("animate-pulse"), "still pulses: {html}");
}

#[test]
fn multiple_skeletons() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            div {
                Skeleton { class: "h-4 w-[200px]" }
                Skeleton { class: "h-4 w-[150px]" }
            }
        }
    }

    let html = render(TestApp);
    assert_eq!(
        html.matches("data-slot=\"skeleton\"").count(),
        2,
        "two skeletons: {html}"
    );
}
