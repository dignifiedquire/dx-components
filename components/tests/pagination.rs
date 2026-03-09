use dioxus::prelude::*;
use dioxus_components::pagination::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn pagination_nav_structure() {
    fn App() -> Element {
        rsx! {
            Pagination {
                PaginationContent {
                    PaginationItem { PaginationLink { "1" } }
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== pagination_nav_structure ===\n{html}\n");

    assert!(html.contains("<nav"));
    assert!(html.contains(r#"data-slot="pagination""#));
    assert!(html.contains(r#"role="navigation""#));
    assert!(html.contains(r#"aria-label="pagination""#));
    assert!(html.contains("mx-auto"));
}

#[test]
fn pagination_content_is_ul() {
    fn App() -> Element {
        rsx! {
            Pagination {
                PaginationContent {
                    PaginationItem { "A" }
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains("<ul"));
    assert!(html.contains(r#"data-slot="pagination-content""#));
    assert!(html.contains("flex flex-row items-center gap-1"));
}

#[test]
fn pagination_link_active_state() {
    fn App() -> Element {
        rsx! {
            Pagination {
                PaginationContent {
                    PaginationItem {
                        PaginationLink { is_active: true, "1" }
                    }
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== pagination_link_active_state ===\n{html}\n");

    assert!(html.contains(r#"data-slot="pagination-link""#));
    assert!(html.contains("data-active=true"));
    assert!(html.contains(r#"aria-current="page""#));
    // Active uses outline variant
    assert!(html.contains("bg-background"));
}

#[test]
fn pagination_link_inactive_state() {
    fn App() -> Element {
        rsx! {
            Pagination {
                PaginationContent {
                    PaginationItem {
                        PaginationLink { "2" }
                    }
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains("data-active=false"));
    // Inactive uses ghost variant
    assert!(html.contains("hover:bg-accent"));
}

#[test]
fn pagination_previous_has_icon() {
    fn App() -> Element {
        rsx! {
            Pagination {
                PaginationContent {
                    PaginationItem { PaginationPrevious {} }
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== pagination_previous_has_icon ===\n{html}\n");

    assert!(html.contains(r#"aria-label="Go to previous page""#));
    assert!(html.contains("<svg"));
    assert!(html.contains("Previous"));
}

#[test]
fn pagination_next_has_icon() {
    fn App() -> Element {
        rsx! {
            Pagination {
                PaginationContent {
                    PaginationItem { PaginationNext {} }
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"aria-label="Go to next page""#));
    assert!(html.contains("<svg"));
    assert!(html.contains("Next"));
}

#[test]
fn pagination_ellipsis_sr_only() {
    fn App() -> Element {
        rsx! {
            Pagination {
                PaginationContent {
                    PaginationItem { PaginationEllipsis {} }
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="pagination-ellipsis""#));
    assert!(html.contains("sr-only"));
    assert!(html.contains("More pages"));
}
