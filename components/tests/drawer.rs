//! SSR snapshot tests for the styled Drawer component.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::drawer::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn drawer_trigger_renders() {
    fn App() -> Element {
        rsx! {
            Drawer {
                DrawerTrigger { "Open" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dialog-trigger""#),
        "trigger has dialog-trigger slot: {html}"
    );
    assert!(
        html.contains(r#"data-state="closed""#),
        "initially closed: {html}"
    );
}

#[test]
fn drawer_header_has_shadcn_classes() {
    fn App() -> Element {
        rsx! {
            Drawer {
                DrawerHeader { "Header" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="drawer-header""#),
        "has header slot: {html}"
    );
    assert!(
        html.contains("flex flex-col gap-1.5 p-4"),
        "header classes: {html}"
    );
}

#[test]
fn drawer_footer_has_shadcn_classes() {
    fn App() -> Element {
        rsx! {
            Drawer {
                DrawerFooter { "Footer" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="drawer-footer""#),
        "has footer slot: {html}"
    );
    assert!(
        html.contains("mt-auto flex flex-col gap-2 p-4"),
        "footer classes: {html}"
    );
}

#[test]
fn drawer_title_has_shadcn_classes() {
    fn App() -> Element {
        rsx! {
            Drawer {
                default_open: true,
                DrawerTitle { "My Title" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="drawer-title""#),
        "has title slot: {html}"
    );
    assert!(
        html.contains("font-semibold text-foreground"),
        "title classes: {html}"
    );
}

#[test]
fn drawer_description_has_shadcn_classes() {
    fn App() -> Element {
        rsx! {
            Drawer {
                default_open: true,
                DrawerDescription { "Some description" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="drawer-description""#),
        "has description slot: {html}"
    );
    assert!(
        html.contains("text-sm text-muted-foreground"),
        "description classes: {html}"
    );
}

#[test]
fn drawer_overlay_has_shadcn_classes() {
    fn App() -> Element {
        rsx! {
            Drawer {
                default_open: true,
                DrawerOverlay {}
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("fixed inset-0 z-50 bg-black/50"),
        "overlay classes: {html}"
    );
}

#[test]
fn drawer_content_default_direction_bottom() {
    fn App() -> Element {
        rsx! {
            Drawer {
                default_open: true,
                DrawerContent { "Content" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="drawer-content""#),
        "has content slot: {html}"
    );
    assert!(
        html.contains(r#"data-vaul-drawer-direction="bottom""#),
        "default direction bottom: {html}"
    );
    assert!(html.contains("bg-background"), "base class: {html}");
    assert!(html.contains("rounded-t-[10px]"), "bottom rounding: {html}");
    assert!(html.contains("border-t"), "bottom border: {html}");
}

#[test]
fn drawer_content_direction_top() {
    fn App() -> Element {
        rsx! {
            Drawer {
                default_open: true,
                DrawerContent { direction: DrawerDirection::Top, "Content" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-vaul-drawer-direction="top""#),
        "direction top: {html}"
    );
    assert!(html.contains("rounded-b-[10px]"), "top rounding: {html}");
    assert!(html.contains("border-b"), "top border: {html}");
}

#[test]
fn drawer_content_direction_left() {
    fn App() -> Element {
        rsx! {
            Drawer {
                default_open: true,
                DrawerContent { direction: DrawerDirection::Left, "Content" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-vaul-drawer-direction="left""#),
        "direction left: {html}"
    );
    assert!(html.contains("rounded-r-[10px]"), "left rounding: {html}");
    assert!(html.contains("border-r"), "left border: {html}");
    assert!(html.contains("max-w-[500px]"), "max width: {html}");
}

#[test]
fn drawer_content_direction_right() {
    fn App() -> Element {
        rsx! {
            Drawer {
                default_open: true,
                DrawerContent { direction: DrawerDirection::Right, "Content" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-vaul-drawer-direction="right""#),
        "direction right: {html}"
    );
    assert!(html.contains("rounded-l-[10px]"), "right rounding: {html}");
    assert!(html.contains("border-l"), "right border: {html}");
}

#[test]
fn drawer_close_renders() {
    fn App() -> Element {
        rsx! {
            Drawer {
                DrawerClose { "Close me" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="dialog-close""#),
        "close slot: {html}"
    );
    assert!(html.contains("Close me"), "close text: {html}");
}

#[test]
fn drawer_custom_class_merge() {
    fn App() -> Element {
        rsx! {
            Drawer {
                DrawerHeader { class: "my-custom", "Header" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("my-custom"), "custom class merged: {html}");
    assert!(
        html.contains(r#"data-slot="drawer-header""#),
        "still has slot: {html}"
    );
}
