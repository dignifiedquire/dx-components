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
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Popover {
                PopoverTrigger { "Toggle" }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== popover_trigger_slot ===\n{html}\n");

    assert!(html.contains(r#"data-slot="popover-trigger""#));
    assert!(html.contains(r#"data-state="closed""#));
    assert!(html.contains(r#"aria-haspopup="dialog""#));
}

#[test]
fn popover_content_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Popover {
                default_open: true,
                PopoverContent {
                    "Content"
                }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== popover_content_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="popover-content""#));
    assert!(html.contains(r#"role="dialog""#));
    assert!(html.contains("z-50 w-72"));
    assert!(html.contains("rounded-md border bg-popover p-4 text-popover-foreground shadow-md"));
}

#[test]
fn popover_content_side_attribute() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Popover {
                default_open: true,
                PopoverContent {
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
fn popover_content_align_attribute() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Popover {
                default_open: true,
                PopoverContent {
                    align: Align::Start,
                    "Content"
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-align="start""#));
}

#[test]
fn popover_close_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Popover {
                PopoverClose { "Close" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="popover-close""#));
    assert!(html.contains("Close"));
}

#[test]
fn popover_content_animation_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Popover {
                default_open: true,
                PopoverContent {
                    "Content"
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains("data-[state=open]:animate-in"));
    assert!(html.contains("data-[state=open]:fade-in-0"));
    assert!(html.contains("data-[state=open]:zoom-in-95"));
}

#[test]
fn popover_consumer_class_merge() {
    #[component]
    fn TestApp() -> Element {
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

    let html = render(TestApp);

    assert!(html.contains("my-custom"));
}
