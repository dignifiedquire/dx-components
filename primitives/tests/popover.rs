//! SSR snapshot tests for the popover primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::popover::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

// ---------------------------------------------------------------------------
// PopoverRoot renders no DOM
// ---------------------------------------------------------------------------

#[test]
fn root_renders_no_dom_element() {
    fn App() -> Element {
        rsx! {
            PopoverRoot {
                span { "sentinel" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("sentinel"), "children should render: {html}");
}

// ---------------------------------------------------------------------------
// PopoverTrigger
// ---------------------------------------------------------------------------

#[test]
fn trigger_attributes() {
    fn App() -> Element {
        rsx! {
            PopoverRoot {
                PopoverTrigger { "Open" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="popover-trigger""#),
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
    assert!(
        html.contains(r#"type="button""#),
        "trigger is type=button: {html}"
    );
    assert!(html.contains("Open"), "trigger has children: {html}");
}

// ---------------------------------------------------------------------------
// PopoverContent
// ---------------------------------------------------------------------------

#[test]
fn content_renders_when_open() {
    fn App() -> Element {
        rsx! {
            PopoverRoot {
                open: true,
                PopoverContent {
                    p { "Hello" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="popover-content""#),
        "content has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="dialog""#),
        "content has role=dialog: {html}"
    );
    assert!(
        html.contains(r#"data-state="open""#),
        "content shows open: {html}"
    );
    assert!(
        html.contains(r#"data-side="bottom""#),
        "default side is bottom: {html}"
    );
    assert!(
        html.contains(r#"data-align="center""#),
        "default align is center: {html}"
    );
    assert!(html.contains("Hello"), "content has children: {html}");
}

#[test]
fn content_hidden_when_closed() {
    fn App() -> Element {
        rsx! {
            PopoverRoot {
                PopoverContent {
                    p { "Hidden" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains(r#"data-slot="popover-content""#),
        "content not rendered when closed: {html}"
    );
}

#[test]
fn content_no_aria_modal_when_not_modal() {
    fn App() -> Element {
        rsx! {
            PopoverRoot {
                open: true,
                PopoverContent {
                    p { "Content" }
                }
            }
        }
    }

    let html = render(App);
    // modal defaults to false, so no aria-modal
    assert!(
        !html.contains("aria-modal"),
        "non-modal popover should not have aria-modal: {html}"
    );
}

#[test]
fn content_has_aria_modal_when_modal() {
    fn App() -> Element {
        rsx! {
            PopoverRoot {
                modal: true,
                open: true,
                PopoverContent {
                    p { "Content" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"aria-modal="true""#),
        "modal popover has aria-modal: {html}"
    );
}

#[test]
fn content_custom_side_and_align() {
    fn App() -> Element {
        rsx! {
            PopoverRoot {
                open: true,
                PopoverContent {
                    side: dioxus_primitives::ContentSide::Top,
                    align: dioxus_primitives::ContentAlign::Start,
                    p { "Content" }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains(r#"data-side="top""#), "custom side: {html}");
    assert!(
        html.contains(r#"data-align="start""#),
        "custom align: {html}"
    );
}

// ---------------------------------------------------------------------------
// PopoverClose
// ---------------------------------------------------------------------------

#[test]
fn close_attributes() {
    fn App() -> Element {
        rsx! {
            PopoverRoot {
                open: true,
                PopoverContent {
                    PopoverClose { "Close" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="popover-close""#),
        "close has data-slot: {html}"
    );
    assert!(
        html.contains(r#"type="button""#),
        "close is type=button: {html}"
    );
    assert!(html.contains("Close"), "close has children: {html}");
}
