//! SSR snapshot tests for the styled card (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::card::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn card_data_slot_and_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Card {
                CardContent { "Content" }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== card_data_slot_and_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="card""#));
    assert!(html.contains("rounded-xl border"));
    assert!(html.contains("bg-card"));
    assert!(html.contains("shadow-sm"));
}

#[test]
fn card_header_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Card {
                CardHeader {
                    CardTitle { "Title" }
                    CardDescription { "Description" }
                }
                CardContent { "Content" }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-slot="card-header""#));
    assert!(html.contains(r#"data-slot="card-title""#));
    assert!(html.contains(r#"data-slot="card-description""#));
}

#[test]
fn card_title_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Card {
                CardHeader {
                    CardTitle { "Title" }
                }
                CardContent { "Content" }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("leading-none font-semibold"));
}

#[test]
fn card_description_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Card {
                CardHeader {
                    CardDescription { "Description" }
                }
                CardContent { "Content" }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("text-muted-foreground"));
}

#[test]
fn card_content_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Card {
                CardContent { "Content" }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-slot="card-content""#));
}

#[test]
fn card_footer_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Card {
                CardContent { "Content" }
                CardFooter { "Footer" }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-slot="card-footer""#));
    assert!(html.contains("items-center"));
}

#[test]
fn card_action_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Card {
                CardHeader {
                    CardTitle { "Title" }
                    CardAction { "Action" }
                }
                CardContent { "Content" }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-slot="card-action""#));
}

#[test]
fn consumer_class_merges() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Card { class: "max-w-sm",
                CardContent { "Content" }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("max-w-sm"));
}
