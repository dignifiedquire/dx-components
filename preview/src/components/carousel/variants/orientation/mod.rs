use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Carousel { total_slides: 5, orientation: CarouselOrientation::Vertical, class: "w-full max-w-xs",
            CarouselContent { class: "-mt-1 h-[200px]",
                for i in 0..5 {
                    CarouselItem { class: "pt-1 md:basis-1/2",
                        div { class: "p-1",
                            div { class: "flex items-center justify-center rounded-xl border bg-card p-6",
                                span { class: "text-3xl font-semibold", "{i + 1}" }
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
