//! SSR snapshot tests for the styled Sidebar component.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::sidebar::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn sidebar_provider_renders() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            SidebarProvider { "Content" }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"sidebar-wrapper\""),
        "provider has wrapper slot: {html}"
    );
    assert!(html.contains("Content"), "renders children: {html}");
}

#[test]
fn sidebar_renders_with_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            SidebarProvider {
                Sidebar { "Sidebar content" }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"sidebar\""),
        "sidebar has slot: {html}"
    );
}

#[test]
fn sidebar_header_has_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            SidebarProvider {
                Sidebar {
                    SidebarHeader { "Header" }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"sidebar-header\""),
        "header has slot: {html}"
    );
    assert!(
        html.contains("data-sidebar=\"header\""),
        "header data-sidebar: {html}"
    );
}

#[test]
fn sidebar_content_has_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            SidebarProvider {
                Sidebar {
                    SidebarContent { "Content" }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"sidebar-content\""),
        "content has slot: {html}"
    );
    assert!(
        html.contains("data-sidebar=\"content\""),
        "content data-sidebar: {html}"
    );
}

#[test]
fn sidebar_footer_has_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            SidebarProvider {
                Sidebar {
                    SidebarFooter { "Footer" }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"sidebar-footer\""),
        "footer has slot: {html}"
    );
    assert!(
        html.contains("data-sidebar=\"footer\""),
        "footer data-sidebar: {html}"
    );
}

#[test]
fn sidebar_menu_renders() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            SidebarProvider {
                Sidebar {
                    SidebarContent {
                        SidebarMenu {
                            SidebarMenuItem {
                                SidebarMenuButton { "Home" }
                            }
                        }
                    }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"sidebar-menu\""),
        "menu has slot: {html}"
    );
    assert!(
        html.contains("data-slot=\"sidebar-menu-item\""),
        "menu-item has slot: {html}"
    );
    assert!(
        html.contains("data-slot=\"sidebar-menu-button\""),
        "menu-button has slot: {html}"
    );
}

#[test]
fn sidebar_group_renders() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            SidebarProvider {
                Sidebar {
                    SidebarContent {
                        SidebarGroup {
                            SidebarGroupLabel { "Group" }
                            SidebarGroupContent { "Items" }
                        }
                    }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"sidebar-group\""),
        "group has slot: {html}"
    );
    assert!(
        html.contains("data-slot=\"sidebar-group-label\""),
        "group-label has slot: {html}"
    );
    assert!(
        html.contains("data-slot=\"sidebar-group-content\""),
        "group-content has slot: {html}"
    );
}

#[test]
fn sidebar_trigger_renders() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            SidebarProvider {
                SidebarTrigger {}
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"sidebar-trigger\""),
        "trigger has slot: {html}"
    );
    assert!(
        html.contains("data-sidebar=\"trigger\""),
        "trigger data-sidebar: {html}"
    );
}

#[test]
fn sidebar_separator_renders() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            SidebarProvider {
                Sidebar {
                    SidebarSeparator {}
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"sidebar-separator\""),
        "separator has slot: {html}"
    );
}

#[test]
fn sidebar_menu_badge_renders() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            SidebarProvider {
                Sidebar {
                    SidebarContent {
                        SidebarMenu {
                            SidebarMenuItem {
                                SidebarMenuButton { "Home" }
                                SidebarMenuBadge { "3" }
                            }
                        }
                    }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"sidebar-menu-badge\""),
        "badge has slot: {html}"
    );
    assert!(html.contains("3"), "badge text: {html}");
}

#[test]
fn sidebar_inset_renders() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            SidebarProvider {
                Sidebar { "Nav" }
                SidebarInset { "Main content" }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"sidebar-inset\""),
        "inset has slot: {html}"
    );
    assert!(html.contains("Main content"), "inset content: {html}");
}

#[test]
fn full_sidebar_composition() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            SidebarProvider {
                Sidebar {
                    SidebarHeader { "App" }
                    SidebarContent {
                        SidebarGroup {
                            SidebarGroupLabel { "Navigation" }
                            SidebarGroupContent {
                                SidebarMenu {
                                    SidebarMenuItem {
                                        SidebarMenuButton { "Dashboard" }
                                    }
                                }
                            }
                        }
                    }
                    SidebarFooter { "v1.0" }
                }
                SidebarInset { "Page" }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"sidebar-wrapper\""),
        "wrapper: {html}"
    );
    assert!(html.contains("data-slot=\"sidebar\""), "sidebar: {html}");
    assert!(
        html.contains("data-slot=\"sidebar-header\""),
        "header: {html}"
    );
    assert!(
        html.contains("data-slot=\"sidebar-content\""),
        "content: {html}"
    );
    assert!(
        html.contains("data-slot=\"sidebar-group\""),
        "group: {html}"
    );
    assert!(html.contains("data-slot=\"sidebar-menu\""), "menu: {html}");
    assert!(
        html.contains("data-slot=\"sidebar-footer\""),
        "footer: {html}"
    );
    assert!(
        html.contains("data-slot=\"sidebar-inset\""),
        "inset: {html}"
    );
}
