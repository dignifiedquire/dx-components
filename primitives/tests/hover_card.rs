//! SSR snapshot tests for the hover card primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::hover_card::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

// ---------------------------------------------------------------------------
// HoverCardRoot renders no DOM
// ---------------------------------------------------------------------------

#[test]
fn root_renders_no_dom_element() {
    fn App() -> Element {
        rsx! {
            HoverCardRoot {
                span { "sentinel" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("sentinel"), "children should render: {html}");
    assert!(
        !html.contains("data-slot=\"hover-card\""),
        "root renders no DOM element: {html}"
    );
}

// ---------------------------------------------------------------------------
// HoverCardTrigger
// ---------------------------------------------------------------------------

#[test]
fn trigger_renders_as_anchor() {
    fn App() -> Element {
        rsx! {
            HoverCardRoot {
                HoverCardTrigger { "@dioxuslabs" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("<a"), "trigger renders as anchor: {html}");
    assert!(
        html.contains(r#"data-slot="hover-card-trigger""#),
        "trigger has data-slot: {html}"
    );
    assert!(
        html.contains(r#"data-state="closed""#),
        "trigger shows closed state: {html}"
    );
    assert!(html.contains("@dioxuslabs"), "trigger has children: {html}");
}

#[test]
fn trigger_shows_open_state() {
    fn App() -> Element {
        rsx! {
            HoverCardRoot {
                open: true,
                HoverCardTrigger { "Trigger" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-state="open""#),
        "trigger shows open state: {html}"
    );
}

#[test]
fn trigger_accepts_href() {
    fn App() -> Element {
        rsx! {
            HoverCardRoot {
                HoverCardTrigger { href: "https://example.com", "Link" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("https://example.com"),
        "trigger has href: {html}"
    );
}

// ---------------------------------------------------------------------------
// HoverCardContent
// ---------------------------------------------------------------------------

#[test]
fn content_renders_when_open() {
    fn App() -> Element {
        rsx! {
            HoverCardRoot {
                open: true,
                HoverCardContent {
                    p { "Card content" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="hover-card-content""#),
        "content has data-slot: {html}"
    );
    assert!(
        html.contains(r#"data-state="open""#),
        "content shows open state: {html}"
    );
    assert!(
        html.contains(r#"data-side="bottom""#),
        "default side is bottom: {html}"
    );
    assert!(
        html.contains(r#"data-align="center""#),
        "default align is center: {html}"
    );
    assert!(
        html.contains("Card content"),
        "content has children: {html}"
    );
    // No role attribute (HoverCard is not a tooltip in Radix)
    assert!(
        !html.contains("role=\"tooltip\""),
        "content should not have role=tooltip: {html}"
    );
}

#[test]
fn content_hidden_when_closed() {
    fn App() -> Element {
        rsx! {
            HoverCardRoot {
                HoverCardContent {
                    p { "Hidden" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains(r#"data-slot="hover-card-content""#),
        "content not rendered when closed: {html}"
    );
}

#[test]
fn content_custom_side_and_align() {
    fn App() -> Element {
        rsx! {
            HoverCardRoot {
                open: true,
                HoverCardContent {
                    side: dioxus_primitives::ContentSide::Left,
                    align: dioxus_primitives::ContentAlign::Start,
                    p { "Content" }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains(r#"data-side="left""#), "custom side: {html}");
    assert!(
        html.contains(r#"data-align="start""#),
        "custom align: {html}"
    );
}

// ---------------------------------------------------------------------------
// Backward compat alias
// ---------------------------------------------------------------------------

#[test]
fn hover_card_alias_works() {
    fn App() -> Element {
        rsx! {
            HoverCard {
                HoverCardTrigger { "Trigger" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="hover-card-trigger""#),
        "HoverCard alias works: {html}"
    );
}
