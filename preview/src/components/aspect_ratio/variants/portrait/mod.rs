use crate::components::aspect_ratio::component::AspectRatio;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            "data-testid": "aspect-ratio-portrait",
            class: "w-full max-w-[10rem]",
            AspectRatio {
                ratio: 9.0 / 16.0,
                class: "rounded-lg bg-muted",
                img {
                    src: "https://avatar.vercel.sh/shadcn1",
                    alt: "Photo",
                    class: "h-full w-full rounded-lg object-cover grayscale dark:brightness-20",
                }
            }
        }
    }
}
