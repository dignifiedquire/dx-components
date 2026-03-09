#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::sheet::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

#[test]
fn sheet_trigger_slot() {
    fn App() -> Element {
        rsx! {
            Sheet {
                SheetTrigger { "Open" }
            }
        }
    }

    let html = render(App);
    eprintln!("=== sheet_trigger_slot ===\n{html}\n");

    // SheetTrigger wraps DialogTrigger so it keeps the dialog-trigger slot
    assert!(html.contains(r#"data-slot="dialog-trigger""#));
    assert!(html.contains(r#"data-state="closed""#));
    assert!(html.contains(r#"aria-haspopup="dialog""#));
}

#[test]
fn sheet_header_classes() {
    fn App() -> Element {
        rsx! {
            Sheet {
                SheetHeader { "Header" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="sheet-header""#));
    assert!(html.contains("flex flex-col gap-2"));
}

#[test]
fn sheet_footer_classes() {
    fn App() -> Element {
        rsx! {
            Sheet {
                SheetFooter { "Footer" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="sheet-footer""#));
    assert!(html.contains("flex flex-col-reverse gap-2 sm:flex-row sm:justify-end"));
}

#[test]
fn sheet_title_with_default_open() {
    fn App() -> Element {
        rsx! {
            Sheet {
                default_open: true,
                SheetTitle { "My Title" }
            }
        }
    }

    let html = render(App);
    eprintln!("=== sheet_title_with_default_open ===\n{html}\n");

    assert!(html.contains(r#"data-slot="sheet-title""#));
    assert!(html.contains("text-lg leading-none font-semibold"));
}

#[test]
fn sheet_description_classes() {
    fn App() -> Element {
        rsx! {
            Sheet {
                default_open: true,
                SheetDescription { "Some description" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="sheet-description""#));
    assert!(html.contains("text-sm text-muted-foreground"));
}

#[test]
fn sheet_overlay_classes() {
    fn App() -> Element {
        rsx! {
            Sheet {
                default_open: true,
                SheetOverlay {}
            }
        }
    }

    let html = render(App);
    eprintln!("=== sheet_overlay_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="dialog-overlay""#));
    assert!(html.contains("fixed inset-0 z-50 bg-black/50"));
}

#[test]
fn sheet_content_default_side() {
    fn App() -> Element {
        rsx! {
            Sheet {
                default_open: true,
                SheetContent {
                    show_close: false,
                    "Content"
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== sheet_content_default_side ===\n{html}\n");

    assert!(html.contains(r#"data-slot="sheet-content""#));
    assert!(html.contains(r#"role="dialog""#));
    assert!(html.contains(r#"data-side="right""#));
    // Base classes
    assert!(html.contains("bg-background"));
    assert!(html.contains("fixed"));
    assert!(html.contains("z-50"));
    assert!(html.contains("shadow-lg"));
    // Right-side specific classes
    assert!(html.contains("inset-y-0"));
    assert!(html.contains("right-0"));
    assert!(html.contains("border-l"));
    assert!(html.contains("sm:max-w-sm"));
}

#[test]
fn sheet_content_side_top() {
    fn App() -> Element {
        rsx! {
            Sheet {
                default_open: true,
                SheetContent {
                    side: SheetSide::Top,
                    show_close: false,
                    "Content"
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-side="top""#));
    assert!(html.contains("inset-x-0"));
    assert!(html.contains("top-0"));
    assert!(html.contains("border-b"));
}

#[test]
fn sheet_content_side_bottom() {
    fn App() -> Element {
        rsx! {
            Sheet {
                default_open: true,
                SheetContent {
                    side: SheetSide::Bottom,
                    show_close: false,
                    "Content"
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-side="bottom""#));
    assert!(html.contains("inset-x-0"));
    assert!(html.contains("bottom-0"));
    assert!(html.contains("border-t"));
}

#[test]
fn sheet_content_side_left() {
    fn App() -> Element {
        rsx! {
            Sheet {
                default_open: true,
                SheetContent {
                    side: SheetSide::Left,
                    show_close: false,
                    "Content"
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-side="left""#));
    assert!(html.contains("inset-y-0"));
    assert!(html.contains("left-0"));
    assert!(html.contains("border-r"));
}

#[test]
fn sheet_content_has_close_button() {
    fn App() -> Element {
        rsx! {
            Sheet {
                default_open: true,
                SheetContent {
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
fn sheet_close_slot() {
    fn App() -> Element {
        rsx! {
            Sheet {
                SheetClose { "Close me" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="dialog-close""#));
    assert!(html.contains("Close me"));
}

#[test]
fn sheet_consumer_class_merge() {
    fn App() -> Element {
        rsx! {
            Sheet {
                SheetHeader { class: "my-custom", "Header" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains("my-custom"));
    assert!(html.contains(r#"data-slot="sheet-header""#));
}
