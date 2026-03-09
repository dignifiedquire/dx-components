#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::popover::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

#[test]
fn popover_trigger_slot() {
    fn App() -> Element {
        rsx! {
            Popover {
                PopoverTrigger { "Toggle" }
            }
        }
    }

    let html = render(App);
    eprintln!("=== popover_trigger_slot ===\n{html}\n");

    assert!(html.contains(r#"data-slot="popover-trigger""#));
    assert!(html.contains(r#"data-state="closed""#));
    assert!(html.contains(r#"aria-haspopup="dialog""#));
}

#[test]
fn popover_content_classes() {
    fn App() -> Element {
        rsx! {
            Popover {
                default_open: true,
                PopoverContent {
                    "Content"
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== popover_content_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="popover-content""#));
    assert!(html.contains(r#"role="dialog""#));
    assert!(html.contains("z-50 w-72"));
    assert!(html.contains("rounded-md border bg-popover p-4 text-popover-foreground shadow-md"));
}

#[test]
fn popover_content_side_attribute() {
    fn App() -> Element {
        rsx! {
            Popover {
                default_open: true,
                PopoverContent {
                    side: ContentSide::Top,
                    "Content"
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-side="top""#));
}

#[test]
fn popover_content_align_attribute() {
    fn App() -> Element {
        rsx! {
            Popover {
                default_open: true,
                PopoverContent {
                    align: ContentAlign::Start,
                    "Content"
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-align="start""#));
}

#[test]
fn popover_close_slot() {
    fn App() -> Element {
        rsx! {
            Popover {
                PopoverClose { "Close" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="popover-close""#));
    assert!(html.contains("Close"));
}

#[test]
fn popover_content_animation_classes() {
    fn App() -> Element {
        rsx! {
            Popover {
                default_open: true,
                PopoverContent {
                    "Content"
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains("data-[state=open]:animate-in"));
    assert!(html.contains("data-[state=open]:fade-in-0"));
    assert!(html.contains("data-[state=open]:zoom-in-95"));
}

#[test]
fn popover_consumer_class_merge() {
    fn App() -> Element {
        rsx! {
            Popover {
                default_open: true,
                PopoverContent {
                    class: "my-custom",
                    "Content"
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains("my-custom"));
}
