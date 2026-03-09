//! Styled calendar.
//!
//! Wraps the unstyled `dioxus_primitives::calendar` primitives and includes
//! a CSS stylesheet that targets `data-slot` attributes.

use dioxus::prelude::*;

pub use dioxus_primitives::calendar::{
    Calendar as CalendarPrimitive, CalendarDay, CalendarGrid, CalendarHeader, CalendarMonthTitle,
    CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton, CalendarProps,
    CalendarSelectMonth, CalendarSelectYear, DateRange, RangeCalendar, RangeCalendarProps,
};

/// Styled Calendar — includes CSS for data-slot styling.
///
/// All sub-components (`CalendarHeader`, `CalendarGrid`, etc.) are re-exported
/// from the primitive and styled via CSS `[data-slot]` selectors.
#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./calendar.css") }
        CalendarPrimitive { ..props }
    }
}
