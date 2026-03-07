//! SSR snapshot tests for the dialog primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::dialog::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

// ---------------------------------------------------------------------------
// DialogRoot renders no DOM (pure provider)
// ---------------------------------------------------------------------------

#[test]
fn root_renders_no_dom_element() {
    fn App() -> Element {
        rsx! {
            DialogRoot {
                open: true,
                span { "sentinel" }
            }
        }
    }

    let html = render(App);
    // Root should not add any wrapper div — only children rendered
    assert!(html.contains("sentinel"), "children should render: {html}");
    // Should NOT have data-slot="dialog-overlay" (overlay is separate now)
    assert!(
        !html.contains(r#"data-slot="dialog-overlay""#),
        "root should not render overlay: {html}"
    );
}

// ---------------------------------------------------------------------------
// DialogTrigger
// ---------------------------------------------------------------------------

#[test]
fn trigger_attributes() {
    fn App() -> Element {
        rsx! {
            DialogRoot {
                DialogTrigger { "Open" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dialog-trigger""#),
        "trigger has data-slot: {html}"
    );
    assert!(
        html.contains(r#"aria-haspopup="dialog""#),
        "trigger has aria-haspopup: {html}"
    );
    assert!(
        html.contains(r#"data-state="closed""#),
        "trigger shows closed state: {html}"
    );
    assert!(
        html.contains(r#"type="button""#),
        "trigger is type=button: {html}"
    );
    assert!(html.contains("Open"), "trigger has children: {html}");
}

#[test]
fn trigger_shows_open_state() {
    fn App() -> Element {
        rsx! {
            DialogRoot {
                open: true,
                DialogTrigger { "Open" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-state="open""#),
        "trigger shows open state: {html}"
    );
    assert!(
        html.contains("aria-expanded=true"),
        "trigger has aria-expanded=true: {html}"
    );
}

// ---------------------------------------------------------------------------
// DialogOverlay
// ---------------------------------------------------------------------------

#[test]
fn overlay_renders_when_open() {
    fn App() -> Element {
        rsx! {
            DialogRoot {
                open: true,
                DialogOverlay {}
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dialog-overlay""#),
        "overlay has data-slot: {html}"
    );
    assert!(
        html.contains(r#"data-state="open""#),
        "overlay shows open state: {html}"
    );
}

#[test]
fn overlay_hidden_when_closed() {
    fn App() -> Element {
        rsx! {
            DialogRoot {
                DialogOverlay {}
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains(r#"data-slot="dialog-overlay""#),
        "overlay not rendered when closed: {html}"
    );
}

#[test]
fn overlay_hidden_when_non_modal() {
    fn App() -> Element {
        rsx! {
            DialogRoot {
                modal: false,
                open: true,
                DialogOverlay {}
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains(r#"data-slot="dialog-overlay""#),
        "overlay not rendered in non-modal mode: {html}"
    );
}

// ---------------------------------------------------------------------------
// DialogContent
// ---------------------------------------------------------------------------

#[test]
fn content_attributes() {
    fn App() -> Element {
        rsx! {
            DialogRoot {
                open: true,
                DialogContent {
                    DialogTitle { "Title" }
                    DialogDescription { "Desc" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dialog-content""#),
        "content has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="dialog""#),
        "content has role=dialog: {html}"
    );
    assert!(
        html.contains(r#"aria-modal="true""#),
        "content has aria-modal: {html}"
    );
    assert!(
        html.contains(r#"data-state="open""#),
        "content shows open state: {html}"
    );
    assert!(
        html.contains("aria-labelledby"),
        "content has aria-labelledby: {html}"
    );
    assert!(
        html.contains("aria-describedby"),
        "content has aria-describedby: {html}"
    );
}

#[test]
fn content_no_aria_modal_when_non_modal() {
    fn App() -> Element {
        rsx! {
            DialogRoot {
                modal: false,
                open: true,
                DialogContent {
                    DialogTitle { "Title" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains(r#"aria-modal"#),
        "non-modal content should not have aria-modal: {html}"
    );
}

// ---------------------------------------------------------------------------
// DialogClose
// ---------------------------------------------------------------------------

#[test]
fn close_attributes() {
    fn App() -> Element {
        rsx! {
            DialogRoot {
                open: true,
                DialogContent {
                    DialogClose { "Close" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dialog-close""#),
        "close has data-slot: {html}"
    );
    assert!(
        html.contains(r#"type="button""#),
        "close is type=button: {html}"
    );
    assert!(html.contains("Close"), "close has children: {html}");
}

// ---------------------------------------------------------------------------
// DialogTitle
// ---------------------------------------------------------------------------

#[test]
fn title_attributes() {
    fn App() -> Element {
        rsx! {
            DialogRoot {
                open: true,
                DialogContent {
                    DialogTitle { "My Title" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dialog-title""#),
        "title has data-slot: {html}"
    );
    assert!(html.contains("My Title"), "title has children: {html}");
    // Title should be an h2
    assert!(html.contains("<h2"), "title is h2: {html}");
}

// ---------------------------------------------------------------------------
// DialogDescription
// ---------------------------------------------------------------------------

#[test]
fn description_attributes() {
    fn App() -> Element {
        rsx! {
            DialogRoot {
                open: true,
                DialogContent {
                    DialogDescription { "Some description" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dialog-description""#),
        "description has data-slot: {html}"
    );
    assert!(
        html.contains("Some description"),
        "description has children: {html}"
    );
    // Description should be a p
    assert!(html.contains("<p"), "description is p: {html}");
}

// ---------------------------------------------------------------------------
// ARIA linking (labelledby/describedby IDs match)
// ---------------------------------------------------------------------------

#[test]
fn aria_ids_linked() {
    fn App() -> Element {
        rsx! {
            DialogRoot {
                open: true,
                DialogContent {
                    DialogTitle { "Title" }
                    DialogDescription { "Desc" }
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

    // Title element should have that ID
    let title_id_attr = format!(r#"id="{labelledby_id}""#);
    assert!(
        html.contains(&title_id_attr),
        "title id should match aria-labelledby: expected {title_id_attr} in {html}"
    );

    // Extract aria-describedby value
    let describedby_start = html
        .find("aria-describedby=\"")
        .expect("has aria-describedby");
    let describedby_val_start = describedby_start + "aria-describedby=\"".len();
    let describedby_val_end =
        html[describedby_val_start..].find('"').unwrap() + describedby_val_start;
    let describedby_id = &html[describedby_val_start..describedby_val_end];

    // Description element should have that ID
    let desc_id_attr = format!(r#"id="{describedby_id}""#);
    assert!(
        html.contains(&desc_id_attr),
        "description id should match aria-describedby: expected {desc_id_attr} in {html}"
    );
}
