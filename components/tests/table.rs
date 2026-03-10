//! SSR snapshot tests for the styled table (shadcn match).

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::table::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn table_container_and_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Table {
                TableBody {
                    TableRow {
                        TableCell { "Data" }
                    }
                }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== table_container_and_slot ===\n{html}\n");

    // Container div wraps the table
    assert!(html.contains(r#"data-slot="table-container""#));
    assert!(html.contains("overflow-x-auto"));

    // Table element
    assert!(html.contains(r#"data-slot="table""#));
    assert!(html.contains("<table"));
    assert!(html.contains("w-full caption-bottom text-sm"));
}

#[test]
fn table_header_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Table {
                TableHeader {
                    TableRow {
                        TableHead { "Name" }
                    }
                }
                TableBody {
                    TableRow {
                        TableCell { "Data" }
                    }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-slot="table-header""#));
    assert!(html.contains("<thead"));
}

#[test]
fn table_body_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Table {
                TableBody {
                    TableRow {
                        TableCell { "Data" }
                    }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-slot="table-body""#));
    assert!(html.contains("<tbody"));
}

#[test]
fn table_row_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Table {
                TableBody {
                    TableRow {
                        TableCell { "Data" }
                    }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-slot="table-row""#));
    assert!(html.contains("border-b transition-colors"));
}

#[test]
fn table_head_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Table {
                TableHeader {
                    TableRow {
                        TableHead { "Name" }
                    }
                }
                TableBody {
                    TableRow {
                        TableCell { "Data" }
                    }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-slot="table-head""#));
    assert!(html.contains("<th"));
    assert!(html.contains("h-10"));
    assert!(html.contains("font-medium"));
}

#[test]
fn table_cell_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Table {
                TableBody {
                    TableRow {
                        TableCell { "Data" }
                    }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-slot="table-cell""#));
    assert!(html.contains("<td"));
    assert!(html.contains("p-2 align-middle"));
}

#[test]
fn table_footer_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Table {
                TableBody {
                    TableRow {
                        TableCell { "Data" }
                    }
                }
                TableFooter {
                    TableRow {
                        TableCell { "Total" }
                    }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-slot="table-footer""#));
    assert!(html.contains("<tfoot"));
    assert!(html.contains("bg-muted/50"));
}

#[test]
fn table_caption_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Table {
                TableBody {
                    TableRow {
                        TableCell { "Data" }
                    }
                }
                TableCaption { "A list of items." }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains(r#"data-slot="table-caption""#));
    assert!(html.contains("<caption"));
    assert!(html.contains("text-muted-foreground"));
}
