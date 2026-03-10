use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Carousel { total_slides: 5, class: "w-full max-w-xs",
            CarouselContent {
                for i in 0..5 {
                    CarouselItem {
                        div { class: "p-1",
                            div { class: "flex aspect-square items-center justify-center rounded-xl border bg-card p-6",
                                span { class: "text-4xl font-semibold", "{i + 1}" }
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
