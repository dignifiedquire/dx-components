use dioxus::prelude::*;
use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator};
use dx_icons_tabler::IconCheck;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "items-top flex gap-2",
            Checkbox { name: "terms-text",
                CheckboxIndicator {
                    IconCheck { size: 14 }
                }
            }
            div { class: "grid gap-1.5 leading-none",
                label { class: "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
                    "Accept terms and conditions"
                }
                p { class: "text-sm text-muted-foreground",
                    "You agree to our Terms of Service and Privacy Policy."
                }
            }
        }
    }
}
