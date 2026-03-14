#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::tooltip::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

#[test]
fn tooltip_trigger_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Tooltip {
                TooltipTrigger { "Hover me" }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== tooltip_trigger_slot ===\n{html}\n");

    assert!(html.contains(r#"data-slot="tooltip-trigger""#));
    assert!(html.contains(r#"data-state="closed""#));
}

#[test]
fn tooltip_content_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Tooltip {
                default_open: true,
                TooltipContent { "Tooltip text" }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== tooltip_content_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="tooltip-content""#));
    assert!(html.contains(r#"role="tooltip""#));
    assert!(html.contains("z-50 w-fit"));
    assert!(html.contains("rounded-md bg-foreground px-3 py-1.5 text-xs"));
    assert!(html.contains("text-balance text-background"));
}

#[test]
fn tooltip_content_side_attribute() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Tooltip {
                default_open: true,
                TooltipContent {
                    side: Side::Bottom,
                    "Tooltip"
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-side="bottom""#));
}

#[test]
fn tooltip_content_default_side_top() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Tooltip {
                default_open: true,
                TooltipContent { "Tooltip" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-side="top""#));
}

#[test]
fn tooltip_content_animation_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Tooltip {
                default_open: true,
                TooltipContent { "Tooltip" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains("animate-in"));
    assert!(html.contains("fade-in-0"));
    assert!(html.contains("zoom-in-95"));
}

#[test]
fn tooltip_consumer_class_merge() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Tooltip {
                default_open: true,
                TooltipContent {
                    class: "my-custom",
                    "Tooltip"
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains("my-custom"));
}
