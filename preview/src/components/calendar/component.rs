pub use dioxus_primitives::calendar::*;

use dioxus::prelude::*;

#[component]
pub fn CalendarView(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CalendarPreviousMonthButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        dioxus_primitives::calendar::CalendarPreviousMonthButton { attributes,
            svg {
                class: "size-5 fill-none stroke-current [stroke-linecap:round] [stroke-linejoin:round] [stroke-width:2]",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "15 6 9 12 15 18" }
            }
        }
    }
}

#[component]
pub fn CalendarNextMonthButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        dioxus_primitives::calendar::CalendarNextMonthButton { attributes,
            svg {
                class: "size-5 fill-none stroke-current [stroke-linecap:round] [stroke-linejoin:round] [stroke-width:2]",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "9 18 15 12 9 6" }
            }
        }
    }
}
