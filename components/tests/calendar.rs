//! SSR snapshot tests for the styled Calendar component.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::calendar::*;
use time::UtcDateTime;

/// Render a component to an HTML string via SSR, with normalized IDs.
fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

// ---------------------------------------------------------------------------
// Calendar renders with data-slot="calendar"
// ---------------------------------------------------------------------------

#[test]
fn calendar_renders_with_data_slot() {
    #[component]
    fn TestApp() -> Element {
        let view_date = UtcDateTime::now().date();
        rsx! {
            Calendar {
                view_date,
                today: view_date,
                CalendarHeader {
                    CalendarNavigation {
                        CalendarPreviousMonthButton { "<" }
                        CalendarMonthTitle {}
                        CalendarNextMonthButton { ">" }
                    }
                }
                CalendarGrid {}
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== calendar_renders_with_data_slot ===\n{html}\n");

    assert!(
        html.contains(r#"data-slot="calendar""#),
        "Calendar root should have data-slot=\"calendar\""
    );
    assert!(
        html.contains(r#"role="application""#),
        "Calendar root should have role=\"application\""
    );
    assert!(
        html.contains(r#"aria-label="Calendar""#),
        "Calendar root should have aria-label=\"Calendar\""
    );
}

// ---------------------------------------------------------------------------
// CalendarGrid renders with data-slot="calendar-grid"
// ---------------------------------------------------------------------------

#[test]
fn calendar_grid_renders() {
    #[component]
    fn TestApp() -> Element {
        let view_date = UtcDateTime::now().date();
        rsx! {
            Calendar {
                view_date,
                today: view_date,
                CalendarGrid {}
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== calendar_grid_renders ===\n{html}\n");

    assert!(
        html.contains(r#"data-slot="calendar-grid""#),
        "CalendarGrid should have data-slot=\"calendar-grid\""
    );
    assert!(
        html.contains(r#"role="grid""#),
        "CalendarGrid should have role=\"grid\""
    );
    // Should render weekday headers
    assert!(
        html.contains(r#"data-slot="calendar-grid-weekday""#),
        "CalendarGrid should render weekday headers"
    );
}

// ---------------------------------------------------------------------------
// Class merge on Calendar root
// ---------------------------------------------------------------------------

#[test]
fn calendar_class_merge() {
    #[component]
    fn TestApp() -> Element {
        let view_date = UtcDateTime::now().date();
        rsx! {
            Calendar {
                class: "my-custom-class",
                view_date,
                today: view_date,
                CalendarGrid {}
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== calendar_class_merge ===\n{html}\n");

    assert!(
        html.contains("my-custom-class"),
        "Custom class should be present on Calendar root"
    );
    assert!(
        html.contains(r#"data-slot="calendar""#),
        "data-slot should still be present after class merge"
    );
}
