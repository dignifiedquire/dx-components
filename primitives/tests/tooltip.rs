//! SSR snapshot tests for the tooltip primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::tooltip::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

// ---------------------------------------------------------------------------
// TooltipRoot renders no DOM
// ---------------------------------------------------------------------------

#[test]
fn root_renders_no_dom_element() {
    fn App() -> Element {
        rsx! {
            TooltipRoot {
                span { "sentinel" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("sentinel"), "children should render: {html}");
    // Root should NOT produce any wrapper element
    assert!(
        !html.contains("data-slot=\"tooltip\""),
        "root renders no DOM element: {html}"
    );
}

// ---------------------------------------------------------------------------
// TooltipTrigger
// ---------------------------------------------------------------------------

#[test]
fn trigger_renders_as_button() {
    fn App() -> Element {
        rsx! {
            TooltipRoot {
                TooltipTrigger { "Hover me" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("<button"),
        "trigger renders as button: {html}"
    );
    assert!(
        html.contains(r#"data-slot="tooltip-trigger""#),
        "trigger has data-slot: {html}"
    );
    assert!(
        html.contains(r#"data-state="closed""#),
        "trigger shows closed state: {html}"
    );
    assert!(html.contains("Hover me"), "trigger has children: {html}");
}

#[test]
fn trigger_no_aria_describedby_when_closed() {
    fn App() -> Element {
        rsx! {
            TooltipRoot {
                TooltipTrigger { "Trigger" }
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains("aria-describedby"),
        "trigger has no aria-describedby when closed: {html}"
    );
}

#[test]
fn trigger_has_aria_describedby_when_open() {
    fn App() -> Element {
        rsx! {
            TooltipRoot {
                open: true,
                TooltipTrigger { "Trigger" }
                TooltipContent { "Content" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("aria-describedby"),
        "trigger has aria-describedby when open: {html}"
    );
}

#[test]
fn trigger_shows_open_state() {
    fn App() -> Element {
        rsx! {
            TooltipRoot {
                open: true,
                TooltipTrigger { "Trigger" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-state="open""#),
        "trigger shows open state: {html}"
    );
}

// ---------------------------------------------------------------------------
// TooltipContent
// ---------------------------------------------------------------------------

#[test]
fn content_renders_when_open() {
    fn App() -> Element {
        rsx! {
            TooltipRoot {
                open: true,
                TooltipContent {
                    p { "Tooltip text" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="tooltip-content""#),
        "content has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="tooltip""#),
        "content has role=tooltip: {html}"
    );
    assert!(
        html.contains(r#"data-state="open""#),
        "content shows open state: {html}"
    );
    assert!(
        html.contains(r#"data-side="top""#),
        "default side is top: {html}"
    );
    assert!(
        html.contains(r#"data-align="center""#),
        "default align is center: {html}"
    );
    assert!(
        html.contains("Tooltip text"),
        "content has children: {html}"
    );
}

#[test]
fn content_hidden_when_closed() {
    fn App() -> Element {
        rsx! {
            TooltipRoot {
                TooltipContent {
                    p { "Hidden" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains(r#"data-slot="tooltip-content""#),
        "content not rendered when closed: {html}"
    );
}

#[test]
fn content_custom_side_and_align() {
    fn App() -> Element {
        rsx! {
            TooltipRoot {
                open: true,
                TooltipContent {
                    side: dioxus_primitives::ContentSide::Right,
                    align: dioxus_primitives::ContentAlign::End,
                    p { "Content" }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains(r#"data-side="right""#), "custom side: {html}");
    assert!(html.contains(r#"data-align="end""#), "custom align: {html}");
}

// ---------------------------------------------------------------------------
// Backward compat alias
// ---------------------------------------------------------------------------

#[test]
fn tooltip_alias_works() {
    fn App() -> Element {
        rsx! {
            Tooltip {
                TooltipTrigger { "Trigger" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="tooltip-trigger""#),
        "Tooltip alias works: {html}"
    );
}

// ---------------------------------------------------------------------------
// ARIA linking (describedby ID matches content ID)
// ---------------------------------------------------------------------------

#[test]
fn aria_describedby_links_to_content_id() {
    fn App() -> Element {
        rsx! {
            TooltipRoot {
                open: true,
                TooltipTrigger { "Trigger" }
                TooltipContent { "Content" }
            }
        }
    }

    let html = render(App);

    // Extract aria-describedby value from trigger
    let describedby_start = html
        .find("aria-describedby=\"")
        .expect("has aria-describedby");
    let val_start = describedby_start + "aria-describedby=\"".len();
    let val_end = html[val_start..].find('"').unwrap() + val_start;
    let describedby_id = &html[val_start..val_end];

    // Content element should have that ID
    let id_attr = format!(r#"id="{describedby_id}""#);
    assert!(
        html.contains(&id_attr),
        "content id matches aria-describedby: expected {id_attr} in {html}"
    );
}
