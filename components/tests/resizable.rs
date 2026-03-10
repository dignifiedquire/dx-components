//! SSR snapshot tests for the styled Resizable component.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::resizable::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn panel_group_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 50.0, "Left" }
                ResizableHandle {}
                ResizablePanel { default_size: 50.0, "Right" }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"resizable-panel-group\""),
        "group slot: {html}"
    );
    assert!(html.contains("flex"), "has flex: {html}");
    assert!(html.contains("h-full"), "has h-full: {html}");
    assert!(html.contains("w-full"), "has w-full: {html}");
}

#[test]
fn vertical_group_has_flex_col() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ResizablePanelGroup {
                orientation: dioxus_primitives::direction::Orientation::Vertical,
                ResizablePanel { default_size: 50.0, "Top" }
                ResizableHandle {}
                ResizablePanel { default_size: 50.0, "Bottom" }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("flex-col"), "vertical flex-col: {html}");
}

#[test]
fn panel_renders_with_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 50.0, "Content" }
                ResizableHandle {}
                ResizablePanel { default_size: 50.0, "Content" }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"resizable-panel\""),
        "panel has slot: {html}"
    );
}

#[test]
fn handle_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 50.0, "Left" }
                ResizableHandle {}
                ResizablePanel { default_size: 50.0, "Right" }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"resizable-handle\""),
        "handle slot: {html}"
    );
    assert!(html.contains("bg-border"), "handle bg-border: {html}");
    assert!(html.contains("w-px"), "handle w-px: {html}");
    assert!(html.contains("items-center"), "handle items-center: {html}");
    assert!(
        html.contains("justify-center"),
        "handle justify-center: {html}"
    );
}

#[test]
fn handle_has_focus_ring() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 50.0, "Left" }
                ResizableHandle {}
                ResizablePanel { default_size: 50.0, "Right" }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("focus-visible:ring-1"), "focus ring: {html}");
    assert!(
        html.contains("focus-visible:ring-ring"),
        "ring color: {html}"
    );
}

#[test]
fn handle_with_grip_icon() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 50.0, "Left" }
                ResizableHandle { with_handle: true }
                ResizablePanel { default_size: 50.0, "Right" }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("rounded-xs"), "grip container: {html}");
    assert!(html.contains("size-2.5"), "grip icon size: {html}");
    assert!(html.contains("<circle"), "has SVG circles: {html}");
}

#[test]
fn handle_without_grip_has_no_svg() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 50.0, "Left" }
                ResizableHandle {}
                ResizablePanel { default_size: 50.0, "Right" }
            }
        }
    }

    let html = render(TestApp);
    assert!(!html.contains("<circle"), "no SVG circles: {html}");
}

#[test]
fn custom_class_merge() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ResizablePanelGroup { class: "my-custom",
                ResizablePanel { default_size: 50.0, "Left" }
                ResizableHandle {}
                ResizablePanel { default_size: 50.0, "Right" }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("my-custom"), "custom class: {html}");
    assert!(html.contains("flex"), "still has flex: {html}");
}

#[test]
fn full_composition() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { default_size: 30.0, "A" }
                ResizableHandle { with_handle: true }
                ResizablePanel { default_size: 70.0, "B" }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"resizable-panel-group\""),
        "group: {html}"
    );
    assert!(
        html.contains("data-slot=\"resizable-panel\""),
        "panel: {html}"
    );
    assert!(
        html.contains("data-slot=\"resizable-handle\""),
        "handle: {html}"
    );
    assert!(html.contains("role=\"separator\""), "separator: {html}");
    assert!(html.contains("role=\"group\""), "group role: {html}");
}
