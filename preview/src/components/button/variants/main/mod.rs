use dioxus::prelude::*;
use dioxus_primitives::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { display: "flex", flex_direction: "column", gap: "0.5rem",
            Button { "Default" }

            Button { variant: ButtonVariant::Secondary, "Secondary" }

            Button { variant: ButtonVariant::Destructive, "Destructive" }

            Button { variant: ButtonVariant::Outline, "Outline" }

            Button { variant: ButtonVariant::Ghost, "Ghost" }

            Button { variant: ButtonVariant::Link, "Link" }

            Button { size: ButtonSize::Sm, "Small" }

            Button { size: ButtonSize::Lg, "Large" }

            Button { class: "w-full", "Full Width" }
        }
    }
}
