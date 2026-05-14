A calendar lets the user pick a date (or a range of dates) from a familiar month grid. The component is keyboard-accessible — Arrow keys move the focus cell, Home / End jump to row edges, PageUp / PageDown change months. Selection is driven through the `selected_date` + `on_date_change` props for single-date pickers, or `selected_range` + `on_range_change` for ranges.

Sub-components compose a calendar's chrome: `CalendarPreviousMonthButton` / `CalendarNextMonthButton` paginate, `CalendarSelectMonth` / `CalendarSelectYear` are dropdown jumps, and `CalendarGrid` is the day grid itself. See [Anatomy](#anatomy) for the full tree.

```rust
use dioxus::prelude::*;
use dioxus_components::calendar::*;
use time::{Date, UtcDateTime};

#[component]
fn DatePicker() -> Element {
    let mut selected = use_signal(|| None::<Date>);
    let mut view = use_signal(|| UtcDateTime::now().date());
    rsx! {
        Calendar {
            selected_date: selected(),
            on_date_change: move |d| selected.set(d),
            view_date: view(),
            on_view_change: move |d| view.set(d),
            CalendarView {
                CalendarHeader {
                    CalendarNavigation {
                        CalendarPreviousMonthButton {}
                        CalendarSelectMonth {}
                        CalendarSelectYear {}
                        CalendarNextMonthButton {}
                    }
                }
                CalendarGrid {}
            }
        }
    }
}
```

For range selection, swap `Calendar` for `RangeCalendar`, `selected_date` for `selected_range`, and `on_date_change` for `on_range_change`. Everything else stays the same.

The component does not own its own popover — pair it with [`Popover`](/docs/components/popover) to build a click-to-open date picker, or use the higher-level [`DatePicker`](/docs/components/date_picker) which packages the two together.
