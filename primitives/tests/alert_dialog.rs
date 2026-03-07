//! SSR snapshot tests for the alert dialog primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

// ---------------------------------------------------------------------------
// AlertDialogRoot renders no DOM (pure provider, wraps DialogRoot)
// ---------------------------------------------------------------------------

#[test]
fn root_renders_no_dom_element() {
    fn App() -> Element {
        rsx! {
            AlertDialogRoot {
                open: true,
                span { "sentinel" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("sentinel"), "children should render: {html}");
    assert!(
        !html.contains(r#"data-slot="alert-dialog-overlay""#),
        "root should not render overlay: {html}"
    );
}

// ---------------------------------------------------------------------------
// AlertDialogTrigger
// ---------------------------------------------------------------------------

#[test]
fn trigger_attributes() {
    fn App() -> Element {
        rsx! {
            AlertDialogRoot {
                AlertDialogTrigger { "Delete" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="alert-dialog-trigger""#),
        "trigger has data-slot: {html}"
    );
    assert!(
        html.contains(r#"aria-haspopup="dialog""#),
        "trigger has aria-haspopup: {html}"
    );
    assert!(
        html.contains(r#"data-state="closed""#),
        "trigger shows closed: {html}"
    );
    assert!(html.contains("Delete"), "trigger has children: {html}");
}

// ---------------------------------------------------------------------------
// AlertDialogOverlay
// ---------------------------------------------------------------------------

#[test]
fn overlay_renders_when_open() {
    fn App() -> Element {
        rsx! {
            AlertDialogRoot {
                open: true,
                AlertDialogOverlay {}
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="alert-dialog-overlay""#),
        "overlay has data-slot: {html}"
    );
    assert!(
        html.contains(r#"data-state="open""#),
        "overlay shows open: {html}"
    );
}

#[test]
fn overlay_hidden_when_closed() {
    fn App() -> Element {
        rsx! {
            AlertDialogRoot {
                AlertDialogOverlay {}
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains(r#"data-slot="alert-dialog-overlay""#),
        "overlay not rendered when closed: {html}"
    );
}

// ---------------------------------------------------------------------------
// AlertDialogContent
// ---------------------------------------------------------------------------

#[test]
fn content_attributes() {
    fn App() -> Element {
        rsx! {
            AlertDialogRoot {
                open: true,
                AlertDialogContent {
                    AlertDialogTitle { "Confirm" }
                    AlertDialogDescription { "Are you sure?" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="alert-dialog-content""#),
        "content has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="alertdialog""#),
        "content has role=alertdialog: {html}"
    );
    assert!(
        html.contains(r#"aria-modal="true""#),
        "content has aria-modal: {html}"
    );
    assert!(
        html.contains(r#"data-state="open""#),
        "content shows open: {html}"
    );
}

// ---------------------------------------------------------------------------
// AlertDialogAction and AlertDialogCancel
// ---------------------------------------------------------------------------

#[test]
fn action_attributes() {
    fn App() -> Element {
        rsx! {
            AlertDialogRoot {
                open: true,
                AlertDialogContent {
                    AlertDialogAction { "Delete" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="alert-dialog-action""#),
        "action has data-slot: {html}"
    );
    assert!(
        html.contains(r#"type="button""#),
        "action is type=button: {html}"
    );
    assert!(html.contains("Delete"), "action has children: {html}");
}

#[test]
fn cancel_attributes() {
    fn App() -> Element {
        rsx! {
            AlertDialogRoot {
                open: true,
                AlertDialogContent {
                    AlertDialogCancel { "Cancel" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="alert-dialog-cancel""#),
        "cancel has data-slot: {html}"
    );
    assert!(
        html.contains(r#"type="button""#),
        "cancel is type=button: {html}"
    );
    assert!(html.contains("Cancel"), "cancel has children: {html}");
}

// ---------------------------------------------------------------------------
// AlertDialogFooter
// ---------------------------------------------------------------------------

#[test]
fn footer_attributes() {
    fn App() -> Element {
        rsx! {
            AlertDialogRoot {
                open: true,
                AlertDialogContent {
                    AlertDialogFooter {
                        AlertDialogCancel { "No" }
                        AlertDialogAction { "Yes" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="alert-dialog-footer""#),
        "footer has data-slot: {html}"
    );
}

// ---------------------------------------------------------------------------
// AlertDialogTitle and AlertDialogDescription
// ---------------------------------------------------------------------------

#[test]
fn title_attributes() {
    fn App() -> Element {
        rsx! {
            AlertDialogRoot {
                open: true,
                AlertDialogContent {
                    AlertDialogTitle { "Are you sure?" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="alert-dialog-title""#),
        "title has data-slot: {html}"
    );
    assert!(html.contains("<h2"), "title is h2: {html}");
    assert!(html.contains("Are you sure?"), "title has children: {html}");
}

#[test]
fn description_attributes() {
    fn App() -> Element {
        rsx! {
            AlertDialogRoot {
                open: true,
                AlertDialogContent {
                    AlertDialogDescription { "This cannot be undone." }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="alert-dialog-description""#),
        "description has data-slot: {html}"
    );
    assert!(html.contains("<p"), "description is p: {html}");
    assert!(
        html.contains("This cannot be undone."),
        "description has children: {html}"
    );
}

// ---------------------------------------------------------------------------
// ARIA linking
// ---------------------------------------------------------------------------

#[test]
fn aria_ids_linked() {
    fn App() -> Element {
        rsx! {
            AlertDialogRoot {
                open: true,
                AlertDialogContent {
                    AlertDialogTitle { "Title" }
                    AlertDialogDescription { "Desc" }
                }
            }
        }
    }

    let html = render(App);

    // Extract aria-labelledby value
    let labelledby_start = html
        .find("aria-labelledby=\"")
        .expect("has aria-labelledby");
    let labelledby_val_start = labelledby_start + "aria-labelledby=\"".len();
    let labelledby_val_end = html[labelledby_val_start..].find('"').unwrap() + labelledby_val_start;
    let labelledby_id = &html[labelledby_val_start..labelledby_val_end];

    let title_id_attr = format!(r#"id="{labelledby_id}""#);
    assert!(
        html.contains(&title_id_attr),
        "title id should match aria-labelledby: {html}"
    );
}
