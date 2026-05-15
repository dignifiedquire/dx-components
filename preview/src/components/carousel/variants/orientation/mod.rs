use super::super::component::*;
use crate::components::card::{Card, CardContent};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Carousel {
            total_slides: 5,
            slides_per_view: 2,
            orientation: CarouselOrientation::Vertical,
            class: "w-full max-w-xs",
            CarouselContent { class: "-mt-1 h-[200px]",
                for i in 0..5 {
                    // `min-h-0` so the slide honours its `basis-1/2` slot
                    // (100px of the 200px track) instead of growing to the
                    // card's intrinsic height — our transform steps by an
                    // exact slide fraction, so every slide must be uniform.
                    CarouselItem { class: "pt-1 basis-1/2 min-h-0",
                        div { class: "h-full p-1",
                            Card { class: "h-full",
                                CardContent { class: "flex h-full items-center justify-center p-6",
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
