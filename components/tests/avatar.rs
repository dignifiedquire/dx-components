//! SSR snapshot tests for the styled avatar (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::avatar::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

// ---------------------------------------------------------------------------
// Avatar root classes
// ---------------------------------------------------------------------------

#[test]
fn avatar_base_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Avatar {
                AvatarFallback { "AB" }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains(
            "group/avatar relative flex size-8 shrink-0 overflow-hidden rounded-full select-none"
        ),
        "avatar should have base classes: {html}"
    );
    assert!(
        html.contains(r#"data-slot="avatar""#),
        "avatar should have data-slot: {html}"
    );
    assert!(
        html.contains(r#"data-size="default""#),
        "avatar should have default data-size: {html}"
    );
}

#[test]
fn avatar_size_sm() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Avatar { size: AvatarSize::Sm,
                AvatarFallback { "SM" }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains(r#"data-size="sm""#),
        "should have data-size=sm: {html}"
    );
}

#[test]
fn avatar_size_lg() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Avatar { size: AvatarSize::Lg,
                AvatarFallback { "LG" }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains(r#"data-size="lg""#),
        "should have data-size=lg: {html}"
    );
}

#[test]
fn avatar_consumer_class() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Avatar { class: "border-2",
                AvatarFallback { "AB" }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("border-2"),
        "consumer class should merge: {html}"
    );
}

// ---------------------------------------------------------------------------
// AvatarImage classes
// ---------------------------------------------------------------------------

#[test]
fn avatar_image_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Avatar {
                AvatarImage { src: "https://example.com/avatar.jpg", alt: "User" }
                AvatarFallback { "AB" }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("aspect-square size-full"),
        "image should have shadcn classes: {html}"
    );
    assert!(
        html.contains(r#"data-slot="avatar-image""#),
        "image should have data-slot: {html}"
    );
}

// ---------------------------------------------------------------------------
// AvatarFallback classes
// ---------------------------------------------------------------------------

#[test]
fn avatar_fallback_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Avatar {
                AvatarFallback { "JD" }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("flex size-full items-center justify-center rounded-full bg-muted text-sm text-muted-foreground"),
        "fallback should have shadcn classes: {html}"
    );
    assert!(
        html.contains(r#"data-slot="avatar-fallback""#),
        "fallback should have data-slot: {html}"
    );
    assert!(
        html.contains("JD"),
        "fallback should render children: {html}"
    );
}

// ---------------------------------------------------------------------------
// AvatarBadge
// ---------------------------------------------------------------------------

#[test]
fn avatar_badge_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Avatar {
                AvatarFallback { "AB" }
                AvatarBadge {}
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains(r#"data-slot="avatar-badge""#),
        "badge should have data-slot: {html}"
    );
    assert!(
        html.contains("absolute right-0 bottom-0 z-10"),
        "badge should have positioning classes: {html}"
    );
}

// ---------------------------------------------------------------------------
// AvatarGroup
// ---------------------------------------------------------------------------

#[test]
fn avatar_group_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            AvatarGroup {
                Avatar { AvatarFallback { "A" } }
                Avatar { AvatarFallback { "B" } }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains(r#"data-slot="avatar-group""#),
        "group should have data-slot: {html}"
    );
    assert!(
        html.contains("group/avatar-group flex -space-x-2"),
        "group should have layout classes: {html}"
    );
}

// ---------------------------------------------------------------------------
// AvatarGroupCount
// ---------------------------------------------------------------------------

#[test]
fn avatar_group_count_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            AvatarGroup {
                AvatarGroupCount { "+3" }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains(r#"data-slot="avatar-group-count""#),
        "group count should have data-slot: {html}"
    );
    assert!(
        html.contains("+3"),
        "group count should render children: {html}"
    );
}
