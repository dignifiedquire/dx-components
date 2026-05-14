use crate::components::avatar::component::{
    Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage,
};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex flex-row flex-wrap items-center gap-6 md:gap-12",
            // Plain avatar with greyscaled image.
            Avatar {
                AvatarImage {
                    src: "https://github.com/shadcn.png",
                    alt: "@shadcn",
                    class: "grayscale",
                }
                AvatarFallback { "CN" }
            }
            // Avatar with a status-indicator badge in the bottom-right corner.
            Avatar {
                AvatarImage { src: "https://github.com/evilrabbit.png", alt: "@evilrabbit" }
                AvatarFallback { "ER" }
                AvatarBadge { class: "bg-green-600 dark:bg-green-800" }
            }
            // Stacked avatar group with overflow count.
            AvatarGroup { class: "grayscale",
                Avatar {
                    AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
                    AvatarFallback { "CN" }
                }
                Avatar {
                    AvatarImage { src: "https://github.com/maxleiter.png", alt: "@maxleiter" }
                    AvatarFallback { "LR" }
                }
                Avatar {
                    AvatarImage { src: "https://github.com/evilrabbit.png", alt: "@evilrabbit" }
                    AvatarFallback { "ER" }
                }
                AvatarGroupCount { "+3" }
            }
        }
    }
}
