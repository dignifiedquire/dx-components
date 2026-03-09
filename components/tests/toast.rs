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
