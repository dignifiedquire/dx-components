#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::select::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

#[test]
fn select_trigger_classes() {
    fn App() -> Element {
        rsx! {
            Select::<String> {
                SelectTrigger {
                    SelectValue {}
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== select_trigger_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="select-trigger""#));
    assert!(html.contains(r#"role="combobox""#));
    assert!(html.contains("rounded-md border border-input"));
    assert!(html.contains("shadow-xs"));
}

#[test]
fn select_trigger_consumer_class_merge() {
    fn App() -> Element {
        rsx! {
            Select::<String> {
                SelectTrigger {
                    class: "my-trigger",
                    SelectValue {}
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains("my-trigger"));
}

#[test]
fn select_trigger_chevron_icon() {
    fn App() -> Element {
        rsx! {
            Select::<String> {
                SelectTrigger {
                    SelectValue {}
                }
            }
        }
    }

    let html = render(App);

    // The styled trigger appends a ChevronDown icon
    assert!(html.contains("svg"));
}

#[test]
fn select_placeholder_text() {
    fn App() -> Element {
        rsx! {
            Select::<String> {
                placeholder: "Pick one...",
                SelectTrigger {
                    SelectValue {}
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains("Pick one..."));
    assert!(html.contains(r#"data-placeholder"#));
}
