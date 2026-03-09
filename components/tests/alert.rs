//! SSR snapshot tests for the styled alert (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::alert::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn default_variant_classes() {
    fn App() -> Element {
        rsx! {
            Alert {
                AlertTitle { "Title" }
                AlertDescription { "Description" }
            }
        }
    }

    let html = render(App);
    eprintln!("=== default_variant_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="alert""#));
    assert!(html.contains(r#"role="alert""#));
    assert!(html.contains("bg-card text-card-foreground"));
    assert!(html.contains("rounded-lg border"));
}

#[test]
fn destructive_variant_classes() {
    fn App() -> Element {
        rsx! {
            Alert { variant: AlertVariant::Destructive,
                AlertTitle { "Error" }
                AlertDescription { "Something went wrong." }
            }
        }
    }

    let html = render(App);
    eprintln!("=== destructive_variant_classes ===\n{html}\n");

    assert!(html.contains("text-destructive"));
}

#[test]
fn title_data_slot() {
    fn App() -> Element {
        rsx! {
            Alert {
                AlertTitle { "Title" }
                AlertDescription { "Desc" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains(r#"data-slot="alert-title""#));
    assert!(html.contains("font-medium tracking-tight"));
}

#[test]
fn description_data_slot() {
    fn App() -> Element {
        rsx! {
            Alert {
                AlertTitle { "Title" }
                AlertDescription { "Desc" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains(r#"data-slot="alert-description""#));
    assert!(html.contains("text-muted-foreground"));
}

#[test]
fn consumer_class_merges() {
    fn App() -> Element {
        rsx! {
            Alert { class: "max-w-md",
                AlertTitle { "Title" }
                AlertDescription { "Desc" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("max-w-md"));
}
