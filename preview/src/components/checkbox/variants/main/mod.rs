use dioxus::prelude::*;
use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator};

#[component]
pub fn Demo() -> Element {
    rsx! {
        Checkbox { name: "tos-check", aria_label: "Demo Checkbox",
            CheckboxIndicator {
                svg {
                    class: "size-3.5",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "3",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M5 13l4 4L19 7" }
                }
            }
        }
    }
}
