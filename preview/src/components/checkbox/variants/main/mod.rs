use dioxus::prelude::*;
use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator};
use dioxus_primitives::label::Label;
use dx_icons_lucide::IconCheck;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex items-center gap-2",
            Checkbox {
                style: "width: 1rem; height: 1rem; border-radius: 4px; border: 1px solid var(--border); padding: 0; background: transparent; cursor: pointer;",
                id: "terms",
                name: "terms",
                CheckboxIndicator {
                    style: "display: flex; align-items: center; justify-content: center;",
                    IconCheck { size: 14 }
                }
            }
            Label { html_for: "terms", "Accept terms and conditions" }
        }
    }
}
