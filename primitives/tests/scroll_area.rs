//! SSR snapshot tests for the scroll_area primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::scroll_area::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn renders_div_with_data_slot() {
    fn App() -> Element {
        rsx! {
            ScrollArea {
                p { "content" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="scroll-area""#),
        "has data-slot: {html}"
    );
    assert!(html.contains("content"), "renders children: {html}");
}

#[test]
fn default_direction_is_both() {
    fn App() -> Element {
        rsx! {
            ScrollArea {
                p { "content" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-scroll-direction="both""#),
        "default direction is both: {html}"
    );
}

#[test]
fn vertical_direction() {
    fn App() -> Element {
        rsx! {
            ScrollArea {
                direction: ScrollDirection::Vertical,
                p { "content" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-scroll-direction="vertical""#),
        "vertical direction: {html}"
    );
}

#[test]
fn horizontal_direction() {
    fn App() -> Element {
        rsx! {
            ScrollArea {
                direction: ScrollDirection::Horizontal,
                p { "content" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-scroll-direction="horizontal""#),
        "horizontal direction: {html}"
    );
}

#[test]
fn hidden_scroll_type_has_scrollbar_width_none() {
    fn App() -> Element {
        rsx! {
            ScrollArea {
                scroll_type: ScrollType::Hidden,
                p { "content" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("scrollbar-width") && html.contains("none"),
        "hidden scroll type has scrollbar-width:none: {html}"
    );
}
