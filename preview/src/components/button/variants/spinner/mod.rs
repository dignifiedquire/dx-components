use super::super::component::*;
use dioxus::prelude::*;
use dioxus_components::spinner::Spinner;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex gap-2",
            Button { variant: ButtonVariant::Outline, disabled: true,
                Spinner { "data-icon": "inline-start" }
                "Generating"
            }
            Button { variant: ButtonVariant::Secondary, disabled: true,
                "Downloading"
                Spinner { "data-icon": "inline-start" }
            }
        }
    }
}
