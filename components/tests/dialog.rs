#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::dialog::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

#[test]
fn dialog_trigger_slot() {
    fn App() -> Element {
        rsx! {
            Dialog {
                DialogTrigger { "Open" }
            }
        }
    }

    let html = render(App);
    eprintln!("=== dialog_trigger_slot ===\n{html}\n");

    assert!(html.contains(r#"data-slot="dialog-trigger""#));
    assert!(html.contains(r#"data-state="closed""#));
    assert!(html.contains(r#"aria-haspopup="dialog""#));
}

#[test]
fn dialog_header_classes() {
    fn App() -> Element {
        rsx! {
            Dialog {
                DialogHeader { "Header" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="dialog-header""#));
    assert!(html.contains("flex flex-col gap-2 text-center sm:text-left"));
}

#[test]
fn dialog_footer_classes() {
    fn App() -> Element {
        rsx! {
            Dialog {
                DialogFooter { "Footer" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="dialog-footer""#));
    assert!(html.contains("flex flex-col-reverse gap-2 sm:flex-row sm:justify-end"));
}

#[test]
fn dialog_title_with_default_open() {
    fn App() -> Element {
        rsx! {
            Dialog {
                default_open: true,
                DialogTitle { "My Title" }
            }
        }
    }

    let html = render(App);
    eprintln!("=== dialog_title_with_default_open ===\n{html}\n");

    assert!(html.contains(r#"data-slot="dialog-title""#));
    assert!(html.contains("text-lg leading-none font-semibold"));
}

#[test]
fn dialog_description_classes() {
    fn App() -> Element {
        rsx! {
            Dialog {
                default_open: true,
                DialogDescription { "Some description" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="dialog-description""#));
    assert!(html.contains("text-sm text-muted-foreground"));
}

#[test]
fn dialog_overlay_classes() {
    fn App() -> Element {
        rsx! {
            Dialog {
                default_open: true,
                DialogOverlay {}
            }
        }
    }

    let html = render(App);
    eprintln!("=== dialog_overlay_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="dialog-overlay""#));
    assert!(html.contains("fixed inset-0 z-50 bg-black/50"));
}

#[test]
fn dialog_content_classes() {
    fn App() -> Element {
        rsx! {
            Dialog {
                default_open: true,
                DialogContent {
                    show_close: false,
                    "Content"
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== dialog_content_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="dialog-content""#));
    assert!(html.contains(r#"role="dialog""#));
    assert!(html.contains("fixed top-[50%] left-[50%] z-50"));
    assert!(html.contains("rounded-lg border bg-background p-6 shadow-lg"));
}

#[test]
fn dialog_content_has_close_button() {
    fn App() -> Element {
        rsx! {
            Dialog {
                default_open: true,
                DialogContent {
                    "Content"
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="dialog-close""#));
    assert!(html.contains("<svg"));
    assert!(html.contains("sr-only"));
    assert!(html.contains("Close"));
}

#[test]
fn dialog_close_slot() {
    fn App() -> Element {
        rsx! {
            Dialog {
                DialogClose { "Close me" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="dialog-close""#));
    assert!(html.contains("Close me"));
}

#[test]
fn dialog_consumer_class_merge() {
    fn App() -> Element {
        rsx! {
            Dialog {
                DialogHeader { class: "my-custom", "Header" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains("my-custom"));
}
