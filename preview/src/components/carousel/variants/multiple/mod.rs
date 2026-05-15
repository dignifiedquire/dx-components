use super::super::component::*;
use crate::components::card::{Card, CardContent};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Carousel {
            total_slides: 5,
            slides_per_view: 3,
            class: "mx-auto max-w-sm",
            CarouselContent {
                for i in 0..5 {
                    CarouselItem { class: "basis-1/3",
                        div { class: "p-1",
                            Card {
                                CardContent { class: "flex aspect-square items-center justify-center p-6",
                                    span { class: "text-3xl font-semibold", "{i + 1}" }
                                }
                            }
                        }
                    }
                }
            }
            CarouselPrevious {}
            CarouselNext {}
        }
    }
}
