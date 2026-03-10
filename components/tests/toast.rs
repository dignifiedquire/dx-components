//! SSR snapshot tests for the styled toast provider.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::toast::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

// ---------------------------------------------------------------------------
// ToastProvider renders
// ---------------------------------------------------------------------------

#[test]
fn toast_provider_renders() {
    fn App() -> Element {
        rsx! {
            ToastProvider { "hello" }
        }
    }

    let html = render(App);
    eprintln!("=== toast_provider_renders ===\n{html}\n");

    assert!(
        html.contains("hello"),
        "toast provider should render children: {html}"
    );
    assert!(
        html.contains("fixed top-0"),
        "toast provider should include viewport classes: {html}"
    );
}

#[test]
fn toast_viewport_has_responsive_classes() {
    fn App() -> Element {
        rsx! {
            ToastProvider { "content" }
        }
    }

    let html = render(App);
    assert!(html.contains("z-[100]"), "viewport z-index: {html}");
    assert!(html.contains("max-h-screen"), "viewport max-h: {html}");
    assert!(
        html.contains("flex-col-reverse"),
        "viewport flex-col-reverse: {html}"
    );
}

#[test]
fn toast_provider_custom_class() {
    fn App() -> Element {
        rsx! {
            ToastProvider { class: "my-custom", "content" }
        }
    }

    let html = render(App);
    assert!(html.contains("my-custom"), "custom class merged: {html}");
    assert!(html.contains("fixed"), "still has fixed: {html}");
}

#[test]
fn toast_provider_renders_multiple_children() {
    fn App() -> Element {
        rsx! {
            ToastProvider {
                div { "Child 1" }
                div { "Child 2" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("Child 1"), "first child: {html}");
    assert!(html.contains("Child 2"), "second child: {html}");
}

#[test]
fn toast_viewport_responsive_breakpoints() {
    fn App() -> Element {
        rsx! {
            ToastProvider { "test" }
        }
    }

    let html = render(App);
    assert!(html.contains("sm:bottom-0"), "sm breakpoint: {html}");
    assert!(html.contains("sm:right-0"), "sm right: {html}");
    assert!(html.contains("md:max-w-[420px]"), "md max width: {html}");
}
