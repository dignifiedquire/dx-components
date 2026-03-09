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
    fn App() -> Element {
        rsx! { Skeleton { class: "h-4 w-[250px]" } }
    }

    let html = render(App);
    eprintln!("=== data_slot_and_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="skeleton""#));
    assert!(html.contains("<div"));
    assert!(html.contains("animate-pulse"));
    assert!(html.contains("rounded-md"));
    assert!(html.contains("bg-accent"));
}

#[test]
fn consumer_class_merges() {
    fn App() -> Element {
        rsx! { Skeleton { class: "h-12 w-12 rounded-full" } }
    }

    let html = render(App);
    assert!(html.contains("h-12 w-12"));
    // rounded-full should override rounded-md via tw_merge
    assert!(html.contains("rounded-full"));
}
