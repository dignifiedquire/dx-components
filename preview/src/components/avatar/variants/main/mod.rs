use dioxus::prelude::*;
use dioxus_primitives::avatar::{Avatar, AvatarFallback, AvatarImage};

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            align_items: "center",
            justify_content: "between",
            gap: "1rem",
            div { class: "avatar-item",
                p { class: "avatar-label", "Basic Usage" }
                Avatar {
                    aria_label: "Basic avatar",
                    AvatarImage {
                        src: "https://avatars.githubusercontent.com/u/66571940?s=96&v=4",
                        alt: "User avatar",
                    }
                    AvatarFallback { "EA" }
                }
            }
            div { class: "avatar-item",
                p { class: "avatar-label", "Rounded" }
                Avatar {
                    class: "rounded-lg",
                    aria_label: "Basic avatar",
                    AvatarImage {
                        src: "https://avatars.githubusercontent.com/u/66571940?s=96&v=4",
                        alt: "User avatar",
                    }
                    AvatarFallback { "EA" }
                }
            }
            div { class: "avatar-item",
                p { class: "avatar-label", "Error State" }
                Avatar {
                    class: "size-10",
                    aria_label: "Error avatar",
                    AvatarImage {
                        src: "https://invalid-url.example/image.jpg",
                        alt: "Invalid image",
                    }
                    AvatarFallback { "JK" }
                }
            }
            div { class: "avatar-item",
                p { class: "avatar-label", "Large Size" }
                Avatar {
                    class: "size-12",
                    aria_label: "Large avatar",
                    AvatarImage {
                        src: asset!("/assets/dioxus-logo.png", ImageAssetOptions::new().with_avif()),
                        alt: "Large avatar",
                    }
                    AvatarFallback { "DX" }
                }
            }
        }
    }
}
