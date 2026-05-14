use dioxus::prelude::*;
use dx_icons_lucide::{IconBadgeCheck, IconBookmark};

use super::super::component::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2",
            Badge { variant: BadgeVariant::Secondary,
                IconBadgeCheck { class: "size-3" }
                "Verified"
            }
            Badge { variant: BadgeVariant::Outline,
                "Bookmark"
                IconBookmark { class: "size-3" }
            }
        }
    }
}
