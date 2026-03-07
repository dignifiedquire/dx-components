//! SSR snapshot tests for the avatar primitive.
//!
//! Each test renders a specific avatar configuration and asserts the exact
//! HTML output matches the expected string. This ensures our HTML structure
//! (data-slot, style, class, etc.) matches Radix UI's avatar.
//!
//! In SSR, `use_effect` does not run, so `AvatarImage` status stays at `Idle`.
//! The image renders with `style="display: none;"` (since `Idle != Loaded`)
//! and the fallback renders (since `can_render` is true with no `delay_ms`
//! and `Idle != Loaded`).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::avatar::*;

/// Render a component to an HTML string via SSR.
///
/// The returned HTML is stripped of generated IDs (dxc-N) so snapshots are
/// stable across runs.
fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    // Normalize auto-generated IDs so snapshots are deterministic.
    // Replace `dxc-N` with `dxc-ID` everywhere.
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

// ---------------------------------------------------------------------------
// Snapshot: avatar root renders a span with data-slot
// ---------------------------------------------------------------------------

#[test]
fn renders_avatar_root() {
    fn App() -> Element {
        rsx! {
            Avatar {
                "avatar content"
            }
        }
    }

    let html = render(App);

    // Avatar root is a <span>
    assert!(html.contains("<span"), "Avatar should render as a <span>");

    // data-slot="avatar"
    assert!(
        html.contains(r#"data-slot="avatar""#),
        "Avatar should have data-slot=\"avatar\""
    );

    // Children are rendered
    assert!(
        html.contains("avatar content"),
        "Avatar should render its children"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: fallback renders in SSR (no image loaded)
// ---------------------------------------------------------------------------

#[test]
fn renders_fallback() {
    fn App() -> Element {
        rsx! {
            Avatar {
                AvatarImage { src: "https://example.com/avatar.jpg", alt: "User" }
                AvatarFallback { "JD" }
            }
        }
    }

    let html = render(App);

    // Avatar root
    assert!(html.contains(r#"data-slot="avatar""#));

    // AvatarFallback renders a <span> with data-slot="avatar-fallback"
    assert!(
        html.contains(r#"data-slot="avatar-fallback""#),
        "AvatarFallback should render in SSR (status is Idle, not Loaded)"
    );

    // Fallback text is visible
    assert!(
        html.contains("JD"),
        "Fallback text should be rendered in SSR"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: image is hidden while loading in SSR
// ---------------------------------------------------------------------------

#[test]
fn image_hidden_while_loading() {
    fn App() -> Element {
        rsx! {
            Avatar {
                AvatarImage { src: "https://example.com/avatar.jpg", alt: "User" }
                AvatarFallback { "JD" }
            }
        }
    }

    let html = render(App);

    // AvatarImage renders an <img> tag
    assert!(html.contains("<img"), "AvatarImage should render an <img>");

    // data-slot="avatar-image"
    assert!(
        html.contains(r#"data-slot="avatar-image""#),
        "AvatarImage should have data-slot=\"avatar-image\""
    );

    // Image is hidden via inline style while status != Loaded
    assert!(
        html.contains(r#"style="display: none;""#),
        "AvatarImage should be hidden (display: none) while loading in SSR"
    );

    // src is set
    assert!(
        html.contains(r#"src="https://example.com/avatar.jpg""#),
        "AvatarImage should have the src attribute"
    );

    // alt is set
    assert!(
        html.contains(r#"alt="User""#),
        "AvatarImage should have the alt attribute"
    );
}

// ---------------------------------------------------------------------------
// Snapshot: class props are forwarded
// ---------------------------------------------------------------------------

#[test]
fn with_class() {
    fn App() -> Element {
        rsx! {
            Avatar { class: "avatar-root-class",
                AvatarFallback { class: "fallback-class", "FB" }
            }
        }
    }

    let html = render(App);

    // Avatar root has the custom class
    assert!(
        html.contains("avatar-root-class"),
        "Avatar should forward the class prop"
    );

    // AvatarFallback has the custom class
    assert!(
        html.contains("fallback-class"),
        "AvatarFallback should forward the class prop"
    );

    // Both data-slots are present
    assert!(html.contains(r#"data-slot="avatar""#));
    assert!(html.contains(r#"data-slot="avatar-fallback""#));

    // Fallback content is rendered
    assert!(html.contains("FB"));
}
