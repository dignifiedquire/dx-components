//! SSR snapshot tests for the styled radio group (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::radio_group::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

// ---------------------------------------------------------------------------
// Base classes
// ---------------------------------------------------------------------------

#[test]
fn radio_group_base_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            RadioGroup {
                RadioGroupItem { value: "a" }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== radio_group_base_classes ===\n{html}\n");

    assert!(
        html.contains("grid gap-3"),
        "radio group should have grid gap-3: {html}"
    );
    assert!(
        html.contains(r#"data-slot="radio-group""#),
        "should have data-slot=radio-group: {html}"
    );
}

// ---------------------------------------------------------------------------
// Item classes
// ---------------------------------------------------------------------------

#[test]
fn radio_group_item_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            RadioGroup {
                RadioGroupItem { value: "a" }
            }
        }
    }

    let html = render(TestApp);

    assert!(
        html.contains("aspect-square cursor-pointer size-4 shrink-0 rounded-full"),
        "radio group item should have base classes: {html}"
    );
    assert!(
        html.contains(r#"data-slot="radio-group-item""#),
        "should have data-slot=radio-group-item: {html}"
    );
}

// ---------------------------------------------------------------------------
// Indicator (composed internally)
// ---------------------------------------------------------------------------

#[test]
fn radio_group_item_indicator() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            RadioGroup {
                RadioGroupItem { value: "a" }
            }
        }
    }

    let html = render(TestApp);

    assert!(
        html.contains(r#"data-slot="radio-group-indicator""#),
        "should have data-slot=radio-group-indicator: {html}"
    );
}

// ---------------------------------------------------------------------------
// Class merge
// ---------------------------------------------------------------------------

#[test]
fn radio_group_class_merge() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            RadioGroup {
                class: "my-group",
                RadioGroupItem { value: "a" }
            }
        }
    }

    let html = render(TestApp);

    assert!(
        html.contains("my-group"),
        "consumer class should be applied: {html}"
    );
}
