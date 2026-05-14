A calendar is composed of a `Calendar` root that owns the selection / view state and a tree of slot components inside `CalendarView` that arrange the navigation chrome and day grid.

```rust
use dioxus::prelude::*;
use dioxus_primitives::calendar::{
    Calendar, CalendarGrid, CalendarHeader, CalendarNavigation, CalendarNextMonthButton,
    CalendarPreviousMonthButton, CalendarSelectMonth, CalendarSelectYear, CalendarView,
};

#[component]
fn Demo() -> Element {
    rsx! {
        Calendar {
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

The pieces:

- **`Calendar`** — root. Owns `selected_date`, `view_date`, optional `min_date` / `max_date`, plus the `on_date_change` and `on_view_change` callbacks.
- **`CalendarView`** — wraps a single visible month. Render multiple `CalendarView`s inside one `Calendar` for a multi-month layout.
- **`CalendarHeader`** — month/year navigation chrome above the grid.
- **`CalendarNavigation`** — flex row that hosts the prev / next / select-month / select-year controls.
- **`CalendarPreviousMonthButton`** / **`CalendarNextMonthButton`** — paginate the visible month.
- **`CalendarSelectMonth`** / **`CalendarSelectYear`** — dropdown jumps (shadcn's `captionLayout="dropdown"` equivalent).
- **`CalendarTitle`** — text title alternative when you don't want the dropdowns.
- **`CalendarGrid`** — the day-grid itself.

For range selection, replace `Calendar` with `RangeCalendar` — same slot tree.
