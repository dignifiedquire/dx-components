use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Carousel { total_slides: 5, class: "w-full max-w-sm",
            CarouselContent { class: "-ml-1",
                for i in 0..5 {
                    CarouselItem { class: "pl-1 md:basis-1/2 lg:basis-1/3",
                        div { class: "p-1",
                            div { class: "flex aspect-square items-center justify-center rounded-xl border bg-card p-6",
                                span { class: "text-2xl font-semibold", "{i + 1}" }
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
