use crate::components::checkbox::component::Checkbox;
use crate::components::label::component::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "items-top flex gap-2",
            Checkbox {
                id: "terms-text",
                name: "terms-text",
            }
            div { class: "grid gap-1.5 leading-none",
                Label { html_for: "terms-text",
                    "Accept terms and conditions"
                }
                p { class: "text-sm text-muted-foreground",
                    "You agree to our Terms of Service and Privacy Policy."
                }
            }
        }
    }
}
