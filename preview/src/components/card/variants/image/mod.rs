use super::super::component::*;
use crate::components::badge::{Badge, BadgeVariant};
use crate::components::button::Button;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Card { class: "relative mx-auto w-full max-w-sm pt-0",
            // Dark overlay sitting on top of the image for legibility.
            div { class: "absolute inset-0 z-30 aspect-video bg-black/35" }
            img {
                src: "https://avatar.vercel.sh/shadcn1",
                alt: "Event cover",
                class: "relative z-20 aspect-video w-full object-cover brightness-60 grayscale dark:brightness-40",
            }
            CardHeader {
                CardAction {
                    Badge { variant: BadgeVariant::Secondary, "Featured" }
                }
                CardTitle { "Design systems meetup" }
                CardDescription {
                    "A practical talk on component APIs, accessibility, and shipping faster."
                }
            }
            CardFooter {
                Button { class: "w-full", "View Event" }
            }
        }
    }
}
