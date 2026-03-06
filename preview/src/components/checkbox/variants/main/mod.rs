use dioxus::prelude::*;
use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator};
use dioxus_primitives::label::Label;
use dx_icons_tabler::IconCheck;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex items-center gap-2",
            Checkbox { name: "terms",
                CheckboxIndicator {
                    IconCheck { size: 14 }
                }
            }
            Label { html_for: "terms", "Accept terms and conditions" }
        }
    }
}
