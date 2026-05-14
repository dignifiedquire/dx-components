use super::super::component::*;
use dioxus::prelude::*;
use time::{macros::date, Date, Month, UtcDateTime, Weekday};

// Localised calendar — passes its own French weekday / month formatters so
// the demo visibly differs from the default English calendar without
// requiring the viewer to switch the site's language. Production code would
// normally feed `on_format_weekday` / `on_format_month` through
// `dioxus_i18n`'s `tid!` macro and pick up the active locale.
fn fr_weekday(w: Weekday) -> String {
    match w {
        Weekday::Monday => "Lu",
        Weekday::Tuesday => "Ma",
        Weekday::Wednesday => "Me",
        Weekday::Thursday => "Je",
        Weekday::Friday => "Ve",
        Weekday::Saturday => "Sa",
        Weekday::Sunday => "Di",
    }
    .to_string()
}

fn fr_month(m: Month) -> String {
    match m {
        Month::January => "Janvier",
        Month::February => "Février",
        Month::March => "Mars",
        Month::April => "Avril",
        Month::May => "Mai",
        Month::June => "Juin",
        Month::July => "Juillet",
        Month::August => "Août",
        Month::September => "Septembre",
        Month::October => "Octobre",
        Month::November => "Novembre",
        Month::December => "Décembre",
    }
    .to_string()
}

#[component]
pub fn Demo() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);
    let mut view_date = use_signal(|| UtcDateTime::now().date());
    rsx! {
        div { class: "calendar-example",
            Calendar {
                selected_date: selected_date(),
                on_date_change: move |date| selected_date.set(date),
                view_date: view_date(),
                on_view_change: move |new_view: Date| view_date.set(new_view),
                on_format_weekday: fr_weekday,
                on_format_month: fr_month,
                min_date: date!(1995 - 07 - 21),
                max_date: date!(2035 - 09 - 11),
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
}
