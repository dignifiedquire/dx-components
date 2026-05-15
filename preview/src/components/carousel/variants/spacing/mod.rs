use super::super::component::*;
use crate::components::card::{Card, CardContent};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Carousel {
            total_slides: 5,
            slides_per_view: 3,
            class: "w-full max-w-sm",
            CarouselContent { class: "-ml-1",
                for i in 0..5 {
                    CarouselItem { class: "basis-1/3 pl-1",
                        div { class: "p-1",
                            Card {
                                CardContent { class: "flex aspect-square items-center justify-center p-6",
                                    span { class: "text-2xl font-semibold", "{i + 1}" }
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
