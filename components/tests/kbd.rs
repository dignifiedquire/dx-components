//! SSR snapshot tests for the styled kbd (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::kbd::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn kbd_data_slot_and_classes() {
    fn App() -> Element {
        rsx! { Kbd { "K" } }
    }

    let html = render(App);
    eprintln!("=== kbd_data_slot_and_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="kbd""#));
    assert!(html.contains("<kbd"));
    assert!(html.contains("bg-muted"));
    assert!(html.contains("text-muted-foreground"));
    assert!(html.contains("rounded-sm"));
    assert!(html.contains("font-sans"));
}

#[test]
fn kbd_group_data_slot() {
    fn App() -> Element {
        rsx! {
            KbdGroup {
                Kbd { "Ctrl" }
                Kbd { "K" }
            }
        }
    }

    let html = render(App);
    eprintln!("=== kbd_group_data_slot ===\n{html}\n");

    assert!(html.contains(r#"data-slot="kbd-group""#));
    assert!(html.contains("inline-flex items-center gap-1"));
}

#[test]
fn consumer_class_merges() {
    fn App() -> Element {
        rsx! { Kbd { class: "text-lg", "K" } }
    }

    let html = render(App);
    assert!(html.contains("text-lg"));
}
