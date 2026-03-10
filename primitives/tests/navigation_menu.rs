//! SSR snapshot tests for the NavigationMenu primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::navigation_menu::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn root_renders_nav() {
    fn App() -> Element {
        rsx! {
            NavigationMenu {
                NavigationMenuList {
                    NavigationMenuItem {
                        NavigationMenuLink { "Home" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("<nav"), "renders nav element: {html}");
    assert!(
        html.contains("data-slot=\"navigation-menu\""),
        "has root data-slot: {html}"
    );
    assert!(
        html.contains("data-orientation=\"horizontal\""),
        "default horizontal: {html}"
    );
}

#[test]
fn list_renders_ul() {
    fn App() -> Element {
        rsx! {
            NavigationMenu {
                NavigationMenuList {
                    NavigationMenuItem {
                        NavigationMenuLink { "Home" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("<ul"), "renders ul element: {html}");
    assert!(
        html.contains("data-slot=\"navigation-menu-list\""),
        "has list data-slot: {html}"
    );
}

#[test]
fn item_renders_li() {
    fn App() -> Element {
        rsx! {
            NavigationMenu {
                NavigationMenuList {
                    NavigationMenuItem {
                        NavigationMenuLink { "Home" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("<li"), "renders li element: {html}");
    assert!(
        html.contains("data-slot=\"navigation-menu-item\""),
        "has item data-slot: {html}"
    );
}

#[test]
fn trigger_renders_button() {
    fn App() -> Element {
        rsx! {
            NavigationMenu {
                NavigationMenuList {
                    NavigationMenuItem {
                        NavigationMenuTrigger { "Products" }
                        NavigationMenuContent {
                            div { "Content" }
                        }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"navigation-menu-trigger\""),
        "has trigger data-slot: {html}"
    );
    assert!(
        html.contains("type=\"button\""),
        "trigger type is button: {html}"
    );
    assert!(
        html.contains("data-state=\"closed\""),
        "initially closed: {html}"
    );
    assert!(
        html.contains("aria-expanded=false"),
        "aria-expanded false: {html}"
    );
}

#[test]
fn content_hidden_when_closed() {
    fn App() -> Element {
        rsx! {
            NavigationMenu {
                NavigationMenuList {
                    NavigationMenuItem {
                        NavigationMenuTrigger { "Products" }
                        NavigationMenuContent {
                            div { "Hidden Content" }
                        }
                    }
                }
            }
        }
    }

    let html = render(App);
    // Content should not be rendered when closed
    assert!(
        !html.contains("data-slot=\"navigation-menu-content\""),
        "content not rendered when closed: {html}"
    );
    assert!(
        !html.contains("Hidden Content"),
        "content text not visible: {html}"
    );
}

#[test]
fn link_renders_anchor() {
    fn App() -> Element {
        rsx! {
            NavigationMenu {
                NavigationMenuList {
                    NavigationMenuItem {
                        NavigationMenuLink { href: "#about", "About" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("<a"), "renders anchor: {html}");
    assert!(
        html.contains("data-slot=\"navigation-menu-link\""),
        "has link data-slot: {html}"
    );
    assert!(html.contains("href=\"#about\""), "has href: {html}");
}

#[test]
fn link_active_state() {
    fn App() -> Element {
        rsx! {
            NavigationMenu {
                NavigationMenuList {
                    NavigationMenuItem {
                        NavigationMenuLink { href: "#about", active: true, "About" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-active=\"true\""),
        "has data-active: {html}"
    );
}

#[test]
fn indicator_renders() {
    fn App() -> Element {
        rsx! {
            NavigationMenu {
                NavigationMenuList {
                    NavigationMenuItem {
                        NavigationMenuLink { "Home" }
                    }
                }
                NavigationMenuIndicator {}
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"navigation-menu-indicator\""),
        "has indicator data-slot: {html}"
    );
    assert!(
        html.contains("data-state=\"hidden\""),
        "indicator hidden when no item open: {html}"
    );
}

#[test]
fn viewport_renders() {
    fn App() -> Element {
        rsx! {
            NavigationMenu {
                NavigationMenuList {
                    NavigationMenuItem {
                        NavigationMenuLink { "Home" }
                    }
                }
                NavigationMenuViewport {}
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"navigation-menu-viewport\""),
        "has viewport data-slot: {html}"
    );
    assert!(
        html.contains("data-state=\"closed\""),
        "viewport closed when no item open: {html}"
    );
}

#[test]
fn vertical_orientation() {
    fn App() -> Element {
        rsx! {
            NavigationMenu { orientation: NavigationMenuOrientation::Vertical,
                NavigationMenuList {
                    NavigationMenuItem {
                        NavigationMenuLink { "Home" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-orientation=\"vertical\""),
        "has vertical orientation: {html}"
    );
}

#[test]
fn full_composition() {
    fn App() -> Element {
        rsx! {
            NavigationMenu {
                NavigationMenuList {
                    NavigationMenuItem {
                        NavigationMenuTrigger { "Products" }
                        NavigationMenuContent {
                            NavigationMenuLink { href: "#analytics", "Analytics" }
                        }
                    }
                    NavigationMenuItem {
                        NavigationMenuLink { href: "#about", active: true, "About" }
                    }
                }
                NavigationMenuIndicator {}
                NavigationMenuViewport {}
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"navigation-menu\""),
        "root: {html}"
    );
    assert!(
        html.contains("data-slot=\"navigation-menu-list\""),
        "list: {html}"
    );
    assert!(
        html.contains("data-slot=\"navigation-menu-item\""),
        "item: {html}"
    );
    assert!(
        html.contains("data-slot=\"navigation-menu-trigger\""),
        "trigger: {html}"
    );
    assert!(
        html.contains("data-slot=\"navigation-menu-link\""),
        "link: {html}"
    );
    assert!(
        html.contains("data-slot=\"navigation-menu-indicator\""),
        "indicator: {html}"
    );
    assert!(
        html.contains("data-slot=\"navigation-menu-viewport\""),
        "viewport: {html}"
    );
}
