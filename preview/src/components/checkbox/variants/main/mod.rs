use crate::components::checkbox::component::{Checkbox, CheckedState};
use crate::components::label::component::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            div { class: "flex items-center gap-3",
                Checkbox { id: "terms" }
                Label { html_for: "terms", "Accept terms and conditions" }
            }
            div { class: "flex items-start gap-3",
                Checkbox { id: "terms-2", default_checked: CheckedState::Checked }
                div { class: "grid gap-2",
                    Label { html_for: "terms-2", "Accept terms and conditions" }
                    p { class: "text-sm text-muted-foreground",
                        "By clicking this checkbox, you agree to the terms and conditions."
                    }
                }
            }
            div { class: "flex items-start gap-3",
                Checkbox { id: "toggle", disabled: true }
                Label { html_for: "toggle", "Enable notifications" }
            }
            Label {
                class: "flex items-start gap-3 rounded-lg border p-3 hover:bg-accent/50 has-[[aria-checked=true]]:border-blue-600 has-[[aria-checked=true]]:bg-blue-50 dark:has-[[aria-checked=true]]:border-blue-900 dark:has-[[aria-checked=true]]:bg-blue-950",
                Checkbox {
                    id: "toggle-2",
                    default_checked: CheckedState::Checked,
                    class: "data-[state=checked]:border-blue-600 data-[state=checked]:bg-blue-600 data-[state=checked]:text-white dark:data-[state=checked]:border-blue-700 dark:data-[state=checked]:bg-blue-700",
                }
                div { class: "grid gap-1.5 font-normal",
                    p { class: "text-sm leading-none font-medium", "Enable notifications" }
                    p { class: "text-sm text-muted-foreground",
                        "You can enable or disable notifications at any time."
                    }
                }
            }
        }
    }
}
