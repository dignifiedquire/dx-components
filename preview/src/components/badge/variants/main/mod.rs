use dioxus::prelude::*;

use super::super::component::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex w-full flex-wrap justify-center gap-2",
            Badge { "Badge" }
            Badge { variant: BadgeVariant::Secondary, "Secondary" }
            Badge { variant: BadgeVariant::Destructive, "Destructive" }
            Badge { variant: BadgeVariant::Outline, "Outline" }
        }
    }
}
