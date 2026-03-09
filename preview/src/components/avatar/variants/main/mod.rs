use crate::components::avatar::component::{Avatar, AvatarFallback, AvatarImage, AvatarSize};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex flex-row items-center gap-4",
            div { class: "flex flex-col items-center gap-2",
                p { class: "text-sm text-muted-foreground", "Default" }
                Avatar {
                    aria_label: "Basic avatar",
                    AvatarImage {
                        src: "https://avatars.githubusercontent.com/u/66571940?s=96&v=4",
                        alt: "User avatar",
                    }
                    AvatarFallback { "EA" }
                }
            }
            div { class: "flex flex-col items-center gap-2",
                p { class: "text-sm text-muted-foreground", "Small" }
                Avatar {
                    size: AvatarSize::Sm,
                    aria_label: "Small avatar",
                    AvatarImage {
                        src: "https://avatars.githubusercontent.com/u/66571940?s=96&v=4",
                        alt: "User avatar",
                    }
                    AvatarFallback { "EA" }
                }
            }
            div { class: "flex flex-col items-center gap-2",
                p { class: "text-sm text-muted-foreground", "Large" }
                Avatar {
                    size: AvatarSize::Lg,
                    aria_label: "Large avatar",
                    AvatarImage {
                        src: "https://avatars.githubusercontent.com/u/66571940?s=96&v=4",
                        alt: "User avatar",
                    }
                    AvatarFallback { "EA" }
                }
            }
            div { class: "flex flex-col items-center gap-2",
                p { class: "text-sm text-muted-foreground", "Fallback" }
                Avatar {
                    aria_label: "Fallback avatar",
                    AvatarImage {
                        src: "https://invalid-url.example/image.jpg",
                        alt: "Invalid image",
                    }
                    AvatarFallback { "JK" }
                }
            }
        }
    }
}
