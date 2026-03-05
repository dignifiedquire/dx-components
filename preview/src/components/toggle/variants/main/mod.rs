use dioxus::prelude::*;
use dioxus_primitives::toggle::{Toggle, ToggleVariant, ToggleSize};

#[component]
pub fn Demo() -> Element {
    rsx! {
        Toggle {
            em { "B" }
        }

        Toggle { variant: ToggleVariant::Outline,
            "Outline"
        }

        Toggle { size: ToggleSize::Sm,
            "Small"
        }

        Toggle { size: ToggleSize::Lg,
            "Large"
        }
    }
}
