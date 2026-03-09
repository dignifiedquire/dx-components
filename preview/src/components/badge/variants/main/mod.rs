use dioxus::prelude::*;

use super::super::component::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2",

            Badge { "Default" }
            Badge { variant: BadgeVariant::Secondary, "Secondary" }
            Badge { variant: BadgeVariant::Destructive, "Destructive" }
            Badge { variant: BadgeVariant::Outline, "Outline" }
            Badge {
                variant: BadgeVariant::Secondary,
                VerifiedIcon {}
                "Verified"
            }
        }
    }
}
