//! SSR snapshot tests for the styled badge (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::badge::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn default_variant() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Badge { "Default" } }
    }

    let html = render(TestApp);
    eprintln!("=== default_variant ===\n{html}\n");

    assert!(html.contains(r#"data-slot="badge""#));
    assert!(html.contains(r#"data-variant="default""#));
    assert!(html.contains("bg-primary text-primary-foreground"));
    assert!(html.contains("rounded-full"));
}

#[test]
fn secondary_variant() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Badge { variant: BadgeVariant::Secondary, "Secondary" } }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-variant="secondary""#));
    assert!(html.contains("bg-secondary text-secondary-foreground"));
}

#[test]
fn destructive_variant() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Badge { variant: BadgeVariant::Destructive, "Destructive" } }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-variant="destructive""#));
    assert!(html.contains("bg-destructive"));
}

#[test]
fn outline_variant() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Badge { variant: BadgeVariant::Outline, "Outline" } }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-variant="outline""#));
    assert!(html.contains("text-foreground"));
}

#[test]
fn ghost_variant() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Badge { variant: BadgeVariant::Ghost, "Ghost" } }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-variant="ghost""#));
}

#[test]
fn link_variant() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Badge { variant: BadgeVariant::Link, "Link" } }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-variant="link""#));
    assert!(html.contains("text-primary"));
}

#[test]
fn renders_span() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Badge { "Test" } }
    }

    let html = render(TestApp);
    assert!(html.contains("<span"));
}

#[test]
fn consumer_class_merges() {
    #[component]
    fn TestApp() -> Element {
        rsx! { Badge { class: "ml-2", "Custom" } }
    }

    let html = render(TestApp);
    assert!(html.contains("ml-2"));
}
