//! SSR snapshot tests for the Resizable primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::resizable::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn panel_group_renders_with_role() {
    fn App() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 50.0, "Left" }
                ResizableHandle {}
                ResizablePanel { default_size: 50.0, "Right" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"resizable-panel-group\""),
        "group has data-slot: {html}"
    );
    assert!(html.contains("role=\"group\""), "group has role: {html}");
    assert!(
        html.contains("data-orientation=\"horizontal\""),
        "default horizontal: {html}"
    );
}

#[test]
fn panel_renders_with_size() {
    fn App() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 30.0, "Left" }
                ResizableHandle {}
                ResizablePanel { default_size: 70.0, "Right" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"resizable-panel\""),
        "panel has data-slot: {html}"
    );
    assert!(
        html.contains("data-panel-size=\"30\""),
        "panel has size 30: {html}"
    );
    assert!(
        html.contains("data-panel-size=\"70\""),
        "panel has size 70: {html}"
    );
}

#[test]
fn panel_has_flex_style() {
    fn App() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 40.0, "Content" }
                ResizableHandle {}
                ResizablePanel { default_size: 60.0, "Content" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("flex: 0 0 40%"),
        "panel has flex style: {html}"
    );
    assert!(
        html.contains("flex: 0 0 60%"),
        "panel has flex style: {html}"
    );
}

#[test]
fn handle_renders_as_separator() {
    fn App() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 50.0, "Left" }
                ResizableHandle {}
                ResizablePanel { default_size: 50.0, "Right" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"resizable-handle\""),
        "handle has data-slot: {html}"
    );
    assert!(
        html.contains("role=\"separator\""),
        "handle has separator role: {html}"
    );
    assert!(
        html.contains("tabindex=\"0\""),
        "handle is focusable: {html}"
    );
}

#[test]
fn handle_has_perpendicular_orientation() {
    fn App() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 50.0, "Left" }
                ResizableHandle {}
                ResizablePanel { default_size: 50.0, "Right" }
            }
        }
    }

    let html = render(App);
    // Group is horizontal, so handle aria-orientation should be vertical
    assert!(
        html.contains("aria-orientation=\"vertical\""),
        "handle perpendicular orientation: {html}"
    );
}

#[test]
fn vertical_orientation() {
    fn App() -> Element {
        rsx! {
            ResizablePanelGroup { orientation: dioxus_primitives::direction::Orientation::Vertical,
                ResizablePanel { default_size: 50.0, "Top" }
                ResizableHandle {}
                ResizablePanel { default_size: 50.0, "Bottom" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-orientation=\"vertical\""),
        "vertical group: {html}"
    );
    // Vertical group → handle is horizontal
    assert!(
        html.contains("aria-orientation=\"horizontal\""),
        "handle horizontal for vertical group: {html}"
    );
}

#[test]
fn handle_aria_values() {
    fn App() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 30.0, min_size: 10.0, "Left" }
                ResizableHandle {}
                ResizablePanel { default_size: 70.0, "Right" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("aria-valuenow=\"30\""),
        "handle aria-valuenow: {html}"
    );
    assert!(
        html.contains("aria-valuemin=\"10\""),
        "handle aria-valuemin: {html}"
    );
    assert!(
        html.contains("aria-valuemax=\"100\""),
        "handle aria-valuemax: {html}"
    );
}

#[test]
fn disabled_handle_not_focusable() {
    fn App() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 50.0, "Left" }
                ResizableHandle { disabled: true }
                ResizablePanel { default_size: 50.0, "Right" }
            }
        }
    }

    let html = render(App);
    // Disabled handle should not have tabindex
    assert!(
        !html.contains("tabindex=\"0\""),
        "disabled handle not focusable: {html}"
    );
}

#[test]
fn three_panel_layout() {
    fn App() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 25.0, "A" }
                ResizableHandle {}
                ResizablePanel { default_size: 50.0, "B" }
                ResizableHandle {}
                ResizablePanel { default_size: 25.0, "C" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-panel-size=\"25\""),
        "panel A size: {html}"
    );
    assert!(
        html.contains("data-panel-size=\"50\""),
        "panel B size: {html}"
    );
    // Should have two handles
    assert_eq!(
        html.matches("data-slot=\"resizable-handle\"").count(),
        2,
        "two handles: {html}"
    );
    // Should have three panels
    assert_eq!(
        html.matches("data-slot=\"resizable-panel\"").count(),
        3,
        "three panels: {html}"
    );
}

#[test]
fn handle_data_dragging_false_initially() {
    fn App() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 50.0, "Left" }
                ResizableHandle {}
                ResizablePanel { default_size: 50.0, "Right" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-dragging=\"false\""),
        "initially not dragging: {html}"
    );
}
