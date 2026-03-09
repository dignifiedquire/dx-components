use dioxus::prelude::*;
use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator};
use dioxus_primitives::label::Label;
use dx_icons_lucide::IconCheck;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "items-top flex gap-2",
            Checkbox {
                style: "width: 1rem; height: 1rem; border-radius: 4px; border: 1px solid var(--border); padding: 0; background: transparent; cursor: pointer;",
                id: "terms-text",
                name: "terms-text",
                CheckboxIndicator {
                    style: "display: flex; align-items: center; justify-content: center;",
                    IconCheck { size: 14 }
                }
            }
            div { class: "grid gap-1.5 leading-none",
                Label { html_for: "terms-text", class: "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
                    "Accept terms and conditions"
                }
                p { class: "text-sm text-muted-foreground",
                    "You agree to our Terms of Service and Privacy Policy."
                }
            }
        }
    }
}
