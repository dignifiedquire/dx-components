//! SSR snapshot tests for the styled aspect ratio.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::aspect_ratio::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

#[test]
fn data_slot_present() {
    fn App() -> Element {
        rsx! {
            AspectRatio { ratio: 16.0 / 9.0,
                img { src: "photo.jpg" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="aspect-ratio""#),
        "should have data-slot: {html}"
    );
}

#[test]
fn consumer_class_passed_through() {
    fn App() -> Element {
        rsx! {
            AspectRatio { class: "rounded-lg overflow-hidden",
                img { src: "photo.jpg" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("rounded-lg overflow-hidden"),
        "consumer class should pass through: {html}"
    );
}

#[test]
fn children_rendered() {
    fn App() -> Element {
        rsx! {
            AspectRatio { ratio: 1.0,
                span { "child-sentinel" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("child-sentinel"),
        "children should render: {html}"
    );
}
