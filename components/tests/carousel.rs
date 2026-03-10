//! SSR snapshot tests for the styled Carousel component.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::carousel::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn carousel_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Carousel { total_slides: 3,
                CarouselContent {
                    CarouselItem { "Slide 1" }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("relative"), "root has relative: {html}");
    assert!(html.contains("data-slot=\"carousel\""), "root slot: {html}");
}

#[test]
fn carousel_content_has_viewport_wrapper() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Carousel { total_slides: 1,
                CarouselContent {
                    CarouselItem { "1" }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-slot=\"carousel-viewport\""),
        "has viewport wrapper: {html}"
    );
    assert!(
        html.contains("overflow-hidden"),
        "viewport overflow: {html}"
    );
}

#[test]
fn carousel_item_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Carousel { total_slides: 1,
                CarouselContent {
                    CarouselItem { "Slide" }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("min-w-0"), "item min-w-0: {html}");
    assert!(html.contains("shrink-0"), "item shrink-0: {html}");
    assert!(html.contains("basis-full"), "item basis-full: {html}");
    assert!(html.contains("pl-4"), "item pl-4: {html}");
}

#[test]
fn carousel_previous_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Carousel { total_slides: 3,
                CarouselContent {
                    CarouselItem { "1" }
                }
                CarouselPrevious {}
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("rounded-full"), "prev rounded: {html}");
    assert!(html.contains("size-8"), "prev size: {html}");
    assert!(html.contains("absolute"), "prev absolute: {html}");
}

#[test]
fn carousel_next_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Carousel { total_slides: 3,
                CarouselContent {
                    CarouselItem { "1" }
                }
                CarouselNext {}
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("rounded-full"), "next rounded: {html}");
    assert!(html.contains("-right-12"), "next position: {html}");
}

#[test]
fn carousel_vertical_orientation() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Carousel { orientation: CarouselOrientation::Vertical, total_slides: 2,
                CarouselContent {
                    CarouselItem { "1" }
                    CarouselItem { "2" }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-orientation=\"vertical\""),
        "vertical: {html}"
    );
    assert!(html.contains("flex-col"), "vertical flex-col: {html}");
    assert!(html.contains("pt-4"), "vertical padding: {html}");
}

#[test]
fn full_styled_carousel_composition() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Carousel { total_slides: 3,
                CarouselContent {
                    CarouselItem { "Slide 1" }
                    CarouselItem { "Slide 2" }
                    CarouselItem { "Slide 3" }
                }
                CarouselPrevious {}
                CarouselNext {}
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("data-slot=\"carousel\""), "root: {html}");
    assert!(
        html.contains("data-slot=\"carousel-content\""),
        "content: {html}"
    );
    assert!(html.contains("data-slot=\"carousel-item\""), "item: {html}");
    assert!(
        html.contains("data-slot=\"carousel-previous\""),
        "previous: {html}"
    );
    assert!(html.contains("data-slot=\"carousel-next\""), "next: {html}");
}
