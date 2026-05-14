### Calendar

Root component for single-date selection. Owns the selection and the visible month.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `selected_date` | `Option<Date>` | `None` | Currently selected date. |
| `on_date_change` | `Callback<Option<Date>>` | — | Fires when the user picks (or clears) a date. |
| `view_date` | `Date` | today | The month being shown — controls which page of the calendar is visible. |
| `on_view_change` | `Callback<Date>` | — | Fires when navigation moves to a different month (prev / next / select). |
| `min_date` | `Option<Date>` | `None` | Earliest selectable date (uses [`time::Date`]). |
| `max_date` | `Option<Date>` | `None` | Latest selectable date. |
| `disabled` | `bool` | `false` | Disables the whole calendar. |
| `unavailable` | `Option<Unavailable>` | `None` | Mark specific ranges as un-pickable (greyed out but still rendered). |
| `class` | `Option<String>` | — | Forwarded to the root element. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the root element. |
| `children` | `Element` | — | Should include one or more `CalendarView`s. |

### RangeCalendar

Drop-in replacement for `Calendar` that selects a `DateRange` instead of a single `Date`.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `selected_range` | `Option<DateRange>` | `None` | Currently selected range. |
| `on_range_change` | `Callback<Option<DateRange>>` | — | Fires when the user finishes a range. |
| `view_date`, `on_view_change`, `min_date`, `max_date`, `disabled`, `unavailable`, `class`, `attributes`, `children` | same as `Calendar` | | See above. |

### CalendarView

Renders one visible month. Place multiple inside a `Calendar` for a side-by-side multi-month layout.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `offset_months` | `i32` | `0` | Offsets which month this view renders (0 = current `view_date`, 1 = next, -1 = previous, …). Useful inside a multi-month layout. |
| `class` | `Option<String>` | — | Forwarded to the view wrapper. |
| `children` | `Element` | — | Header + grid for this month. |

### CalendarHeader / CalendarNavigation

Layout helpers — just `<div>`s with classes; they accept `class` and forward attributes and render their children.

### CalendarPreviousMonthButton / CalendarNextMonthButton

Buttons that advance the visible month by ±1. Both accept `class` and `attributes`; they read the navigation state from context, so no other props.

### CalendarSelectMonth / CalendarSelectYear

Dropdowns that jump to a specific month / year of the current view. Both accept `class` and `attributes`. `CalendarSelectYear` paginates years within the `[min_date, max_date]` window.

### CalendarTitle

Renders the textual "Month Year" caption — use this instead of the select dropdowns when you don't need quick-jump.

### CalendarGrid

The grid of weekday cells. Accepts `class` and forwards `attributes`. Each cell renders a `<button>` with `data-state` reflecting `selected` / `today` / `unavailable` / `disabled`.

#### Data attributes

| Attribute | Where | Values |
| --- | --- | --- |
| `[data-slot]` | every part | `"calendar"`, `"calendar-grid"`, `"calendar-cell"`, etc. |
| `[data-state]` | grid cell | `"selected"`, `"today"`, `"in-range"`, `"range-start"`, `"range-end"` |
| `[data-disabled]` | grid cell | present when out of `[min_date, max_date]` |
| `[data-unavailable]` | grid cell | present when inside an `unavailable` range |

### DateRange

```rust
pub struct DateRange {
    start: Date,
    end: Date,
}
```

Use `DateRange::new(start, end)` to construct; `contains(date)`, `start()`, `end()` accessors.

### Unavailable

Helper for marking ranges as unavailable.

```rust
let unavail = Unavailable::new(&[
    DateRange::new(date!(2026 - 07 - 04), date!(2026 - 07 - 04)),
    DateRange::new(date!(2026 - 12 - 24), date!(2026 - 12 - 26)),
]);
```
