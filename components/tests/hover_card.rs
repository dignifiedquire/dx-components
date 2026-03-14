#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::hover_card::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

#[test]
fn hover_card_trigger_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            HoverCard {
                HoverCardTrigger { "Hover me" }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== hover_card_trigger_slot ===\n{html}\n");

    assert!(html.contains(r#"data-slot="hover-card-trigger""#));
    assert!(html.contains(r#"data-state="closed""#));
    // HoverCardTrigger renders as <a>
    assert!(html.contains("<a "));
}

#[test]
fn hover_card_content_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            HoverCard {
                default_open: true,
                HoverCardContent { "Card content" }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== hover_card_content_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="hover-card-content""#));
    assert!(html.contains("z-50 w-64"));
    assert!(html.contains("rounded-md border bg-popover p-4 text-popover-foreground shadow-md"));
}

#[test]
fn hover_card_content_side_attribute() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            HoverCard {
                default_open: true,
                HoverCardContent {
                    side: Side::Top,
                    "Content"
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-side="top""#));
}

#[test]
fn hover_card_content_default_side_bottom() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            HoverCard {
                default_open: true,
                HoverCardContent { "Content" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-side="bottom""#));
}

#[test]
fn hover_card_content_animation_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            HoverCard {
                default_open: true,
                HoverCardContent { "Content" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains("data-[state=open]:animate-in"));
    assert!(html.contains("data-[state=open]:fade-in-0"));
    assert!(html.contains("data-[state=open]:zoom-in-95"));
}

#[test]
fn hover_card_consumer_class_merge() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            HoverCard {
                default_open: true,
                HoverCardContent {
                    class: "my-custom",
                    "Content"
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains("my-custom"));
}
