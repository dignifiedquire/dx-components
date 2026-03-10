#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::alert_dialog::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

#[test]
fn alert_dialog_trigger_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            AlertDialog {
                AlertDialogTrigger { "Delete" }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== alert_dialog_trigger_slot ===\n{html}\n");

    assert!(html.contains(r#"data-slot="alert-dialog-trigger""#));
    assert!(html.contains(r#"data-state="closed""#));
    assert!(html.contains(r#"aria-haspopup="dialog""#));
}

#[test]
fn alert_dialog_overlay_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            AlertDialog {
                default_open: true,
                AlertDialogOverlay {}
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="alert-dialog-overlay""#));
    assert!(html.contains("fixed inset-0 z-50 bg-black/50"));
}

#[test]
fn alert_dialog_content_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            AlertDialog {
                default_open: true,
                AlertDialogContent {
                    "Content"
                }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== alert_dialog_content_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="alert-dialog-content""#));
    assert!(html.contains(r#"role="alertdialog""#));
    assert!(html.contains("fixed top-[50%] left-[50%] z-50"));
    assert!(html.contains("rounded-lg border bg-background p-6 shadow-lg"));
}

#[test]
fn alert_dialog_header_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            AlertDialog {
                AlertDialogHeader { "Header" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="alert-dialog-header""#));
    assert!(html.contains("flex flex-col gap-2 text-center sm:text-left"));
}

#[test]
fn alert_dialog_footer_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            AlertDialog {
                AlertDialogFooter { "Footer" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="alert-dialog-footer""#));
    assert!(html.contains("flex flex-col-reverse gap-2 sm:flex-row sm:justify-end"));
}

#[test]
fn alert_dialog_title_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            AlertDialog {
                default_open: true,
                AlertDialogTitle { "Confirm" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="alert-dialog-title""#));
    assert!(html.contains("text-lg font-semibold"));
}

#[test]
fn alert_dialog_description_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            AlertDialog {
                default_open: true,
                AlertDialogDescription { "Are you sure?" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="alert-dialog-description""#));
    assert!(html.contains("text-sm text-muted-foreground"));
}

#[test]
fn alert_dialog_action_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            AlertDialog {
                AlertDialogAction { "Confirm" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="alert-dialog-action""#));
    assert!(html.contains("Confirm"));
}

#[test]
fn alert_dialog_cancel_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            AlertDialog {
                AlertDialogCancel { "Cancel" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="alert-dialog-cancel""#));
    assert!(html.contains("Cancel"));
}
