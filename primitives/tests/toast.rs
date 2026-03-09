//! SSR snapshot tests for the toast primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::toast::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn provider_renders_viewport() {
    fn App() -> Element {
        rsx! {
            ToastProvider {
                span { "child" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="toast-viewport""#),
        "provider renders viewport: {html}"
    );
    assert!(
        html.contains(r#"role="region""#),
        "viewport has role=region: {html}"
    );
    assert!(html.contains("child"), "provider renders children: {html}");
}

#[test]
fn provider_renders_toast_list() {
    fn App() -> Element {
        rsx! {
            ToastProvider {
                span { "child" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="toast-list""#),
        "provider renders toast list: {html}"
    );
}

#[test]
fn toast_has_data_slot() {
    fn App() -> Element {
        rsx! {
            ToastProvider {
                Toast {
                    id: 0usize,
                    index: 0usize,
                    title: "Test Toast",
                    toast_type: ToastType::Info,
                    on_close: |_| {},
                    duration: None,
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="toast""#),
        "toast has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="alertdialog""#),
        "toast has role=alertdialog: {html}"
    );
    assert!(
        html.contains(r#"data-type="info""#),
        "toast has data-type: {html}"
    );
}

#[test]
fn toast_renders_title_and_description() {
    fn App() -> Element {
        rsx! {
            ToastProvider {
                Toast {
                    id: 0usize,
                    index: 0usize,
                    title: "Hello Title",
                    description: "Hello Description",
                    toast_type: ToastType::Success,
                    on_close: |_| {},
                    duration: None,
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("Hello Title"), "toast renders title: {html}");
    assert!(
        html.contains("Hello Description"),
        "toast renders description: {html}"
    );
    assert!(
        html.contains(r#"data-slot="toast-title""#),
        "title has data-slot: {html}"
    );
    assert!(
        html.contains(r#"data-slot="toast-description""#),
        "description has data-slot: {html}"
    );
}

#[test]
fn toast_close_button() {
    fn App() -> Element {
        rsx! {
            ToastProvider {
                Toast {
                    id: 0usize,
                    index: 0usize,
                    title: "Test",
                    toast_type: ToastType::Error,
                    on_close: |_| {},
                    duration: None,
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="toast-close""#),
        "close button has data-slot: {html}"
    );
    assert!(
        html.contains(r#"aria-label="close""#),
        "close button has aria-label: {html}"
    );
}
