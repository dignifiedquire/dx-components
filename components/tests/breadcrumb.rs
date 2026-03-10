use dioxus::prelude::*;
use dioxus_components::breadcrumb::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn breadcrumb_nav_and_aria() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbItem { BreadcrumbLink { href: "/", "Home" } }
                    BreadcrumbSeparator {}
                    BreadcrumbItem { BreadcrumbPage { "Current" } }
                }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== breadcrumb_nav_and_aria ===\n{html}\n");

    assert!(html.contains("<nav"));
    assert!(html.contains(r#"data-slot="breadcrumb""#));
    assert!(html.contains(r#"aria-label="breadcrumb""#));
}

#[test]
fn breadcrumb_list_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbItem { "A" }
                }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== breadcrumb_list_classes ===\n{html}\n");

    assert!(html.contains("<ol"));
    assert!(html.contains(r#"data-slot="breadcrumb-list""#));
    assert!(html.contains("flex flex-wrap"));
    assert!(html.contains("text-muted-foreground"));
}

#[test]
fn breadcrumb_item_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbItem { "Test" }
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains("<li"));
    assert!(html.contains(r#"data-slot="breadcrumb-item""#));
    assert!(html.contains("inline-flex items-center"));
}

#[test]
fn breadcrumb_link_has_href() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbItem {
                        BreadcrumbLink { href: "/docs", "Docs" }
                    }
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="breadcrumb-link""#));
    assert!(html.contains(r#"href="/docs""#));
    assert!(html.contains("hover:text-foreground"));
}

#[test]
fn breadcrumb_page_aria_current() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbItem {
                        BreadcrumbPage { "Current" }
                    }
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="breadcrumb-page""#));
    assert!(html.contains(r#"aria-current="page""#));
    assert!(html.contains("text-foreground"));
}

#[test]
fn breadcrumb_separator_has_chevron() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbSeparator {}
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="breadcrumb-separator""#));
    assert!(html.contains(r#"aria-hidden="true""#));
    assert!(html.contains("<svg"));
}

#[test]
fn breadcrumb_ellipsis_sr_only() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbEllipsis {}
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="breadcrumb-ellipsis""#));
    assert!(html.contains("sr-only"));
    assert!(html.contains("More"));
}
