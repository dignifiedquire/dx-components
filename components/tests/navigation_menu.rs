//! SSR snapshot tests for the styled NavigationMenu component.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::navigation_menu::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn navigation_menu_has_shadcn_classes() {
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
    assert!(html.contains("max-w-max"), "root has shadcn class: {html}");
    assert!(
        html.contains("data-slot=\"navigation-menu\""),
        "root has data-slot: {html}"
    );
}

#[test]
fn navigation_menu_list_has_shadcn_classes() {
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
    assert!(html.contains("list-none"), "list has shadcn class: {html}");
}

#[test]
fn navigation_menu_item_has_shadcn_classes() {
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
    assert!(html.contains("relative"), "item has relative class: {html}");
}

#[test]
fn navigation_menu_trigger_has_shadcn_classes() {
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
        html.contains("rounded-md"),
        "trigger has rounded-md: {html}"
    );
    assert!(
        html.contains("bg-background"),
        "trigger has bg-background: {html}"
    );
}

#[test]
fn navigation_menu_link_has_shadcn_classes() {
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
    assert!(html.contains("rounded-sm"), "link has rounded-sm: {html}");
    assert!(
        html.contains("hover:bg-accent"),
        "link has hover class: {html}"
    );
}

#[test]
fn navigation_menu_indicator_has_shadcn_classes() {
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
        html.contains("overflow-hidden"),
        "indicator has overflow-hidden: {html}"
    );
}

#[test]
fn navigation_menu_viewport_has_shadcn_classes() {
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
        html.contains("bg-popover"),
        "viewport has bg-popover: {html}"
    );
    assert!(
        html.contains("rounded-md"),
        "viewport has rounded-md: {html}"
    );
}

#[test]
fn full_styled_navigation_menu_composition() {
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
