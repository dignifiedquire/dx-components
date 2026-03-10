use crate::components::label::component::Label;
use crate::components::switch::component::Switch;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut emails = use_signal(|| true);
    let mut marketing = use_signal(|| false);

    rsx! {
        div { class: "space-y-4",
            div { class: "flex items-center justify-between gap-4 rounded-lg border p-4",
                div { class: "space-y-0.5",
                    Label { html_for: "emails", "Marketing emails" }
                    p { class: "text-sm text-muted-foreground",
                        "Receive emails about new products, features, and more."
                    }
                }
                Switch {
                    id: "emails",
                    checked: emails(),
                    on_checked_change: move |v| emails.set(v),
                }
            }
            div { class: "flex items-center justify-between gap-4 rounded-lg border p-4",
                div { class: "space-y-0.5",
                    Label { html_for: "marketing", "Security emails" }
                    p { class: "text-sm text-muted-foreground",
                        "Receive emails about your account security."
                    }
                }
                Switch {
                    id: "marketing",
                    checked: marketing(),
                    on_checked_change: move |v| marketing.set(v),
                }
            }
        }
    }
}
