use crate::components::avatar::component::{Avatar, AvatarFallback, AvatarImage, AvatarSize};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex flex-row items-center gap-6",
            div { class: "flex flex-col items-center gap-2",
                p { class: "text-sm text-muted-foreground", "Sm" }
                Avatar { size: AvatarSize::Sm,
                    AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
                    AvatarFallback { "CN" }
                }
            }
            div { class: "flex flex-col items-center gap-2",
                p { class: "text-sm text-muted-foreground", "Default" }
                Avatar {
                    AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
                    AvatarFallback { "CN" }
                }
            }
            div { class: "flex flex-col items-center gap-2",
                p { class: "text-sm text-muted-foreground", "Lg" }
                Avatar { size: AvatarSize::Lg,
                    AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
                    AvatarFallback { "CN" }
                }
            }
        }
    }
}
