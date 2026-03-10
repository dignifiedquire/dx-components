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
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Select::<String> {
                SelectTrigger {
                    SelectValue {}
                }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== select_trigger_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="select-trigger""#));
    assert!(html.contains(r#"role="combobox""#));
    assert!(html.contains("rounded-md border border-input"));
    assert!(html.contains("shadow-xs"));
}

#[test]
fn select_trigger_consumer_class_merge() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Select::<String> {
                SelectTrigger {
                    class: "my-trigger",
                    SelectValue {}
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains("my-trigger"));
}

#[test]
fn select_trigger_chevron_icon() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Select::<String> {
                SelectTrigger {
                    SelectValue {}
                }
            }
        }
    }

    let html = render(TestApp);

    // The styled trigger appends a ChevronDown icon
    assert!(html.contains("svg"));
}

#[test]
fn select_placeholder_text() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Select::<String> {
                placeholder: "Pick one...",
                SelectTrigger {
                    SelectValue {}
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains("Pick one..."));
    assert!(html.contains(r#"data-placeholder"#));
}
