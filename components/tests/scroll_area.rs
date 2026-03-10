#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::scroll_area::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

#[test]
fn scroll_area_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ScrollArea {
                "Scrollable content"
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== scroll_area_slot ===\n{html}\n");

    assert!(html.contains(r#"data-slot="scroll-area""#));
    assert!(html.contains("relative"));
}

#[test]
fn scroll_area_direction_attribute() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ScrollArea {
                direction: ScrollDirection::Horizontal,
                "Content"
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-scroll-direction="horizontal""#));
}

#[test]
fn scroll_area_consumer_class_merge() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ScrollArea {
                class: "my-scroll",
                "Content"
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains("my-scroll"));
}
