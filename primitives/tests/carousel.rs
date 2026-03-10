//! SSR snapshot tests for the Carousel primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::carousel::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn root_renders_with_region_role() {
    fn App() -> Element {
        rsx! {
            Carousel { total_slides: 3,
                CarouselContent {
                    CarouselItem { "Slide 1" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"carousel\""),
        "root has data-slot: {html}"
    );
    assert!(html.contains("role=\"region\""), "root has role: {html}");
    assert!(
        html.contains("aria-roledescription=\"carousel\""),
        "root has aria-roledescription: {html}"
    );
    assert!(
        html.contains("data-orientation=\"horizontal\""),
        "default horizontal: {html}"
    );
}

#[test]
fn content_renders() {
    fn App() -> Element {
        rsx! {
            Carousel { total_slides: 1,
                CarouselContent {
                    CarouselItem { "Slide" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"carousel-content\""),
        "content has data-slot: {html}"
    );
}

#[test]
fn item_renders_as_slide() {
    fn App() -> Element {
        rsx! {
            Carousel { total_slides: 1,
                CarouselContent {
                    CarouselItem { "Slide 1" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"carousel-item\""),
        "item has data-slot: {html}"
    );
    assert!(
        html.contains("role=\"group\""),
        "item has group role: {html}"
    );
    assert!(
        html.contains("aria-roledescription=\"slide\""),
        "item has slide roledescription: {html}"
    );
}

#[test]
fn previous_button_disabled_at_start() {
    fn App() -> Element {
        rsx! {
            Carousel { total_slides: 3, initial_index: 0,
                CarouselContent {
                    CarouselItem { "1" }
                    CarouselItem { "2" }
                    CarouselItem { "3" }
                }
                CarouselPrevious {}
                CarouselNext {}
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"carousel-previous\""),
        "previous has data-slot: {html}"
    );
    assert!(
        html.contains("aria-label=\"Previous slide\""),
        "previous has aria-label: {html}"
    );
    // At index 0, previous should be disabled
    assert!(
        html.contains("disabled"),
        "previous disabled at start: {html}"
    );
}

#[test]
fn next_button_enabled_at_start() {
    fn App() -> Element {
        rsx! {
            Carousel { total_slides: 3, initial_index: 0,
                CarouselContent {
                    CarouselItem { "1" }
                    CarouselItem { "2" }
                    CarouselItem { "3" }
                }
                CarouselNext {}
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"carousel-next\""),
        "next has data-slot: {html}"
    );
    assert!(
        html.contains("aria-label=\"Next slide\""),
        "next has aria-label: {html}"
    );
}

#[test]
fn next_button_disabled_at_end() {
    fn App() -> Element {
        rsx! {
            Carousel { total_slides: 3, initial_index: 2,
                CarouselContent {
                    CarouselItem { "1" }
                    CarouselItem { "2" }
                    CarouselItem { "3" }
                }
                CarouselNext {}
            }
        }
    }

    let html = render(App);
    // At last index, next should be disabled
    assert!(html.contains("disabled"), "next disabled at end: {html}");
}

#[test]
fn vertical_orientation() {
    fn App() -> Element {
        rsx! {
            Carousel { orientation: CarouselOrientation::Vertical, total_slides: 2,
                CarouselContent {
                    CarouselItem { "1" }
                    CarouselItem { "2" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-orientation=\"vertical\""),
        "has vertical orientation: {html}"
    );
}

#[test]
fn default_button_text() {
    fn App() -> Element {
        rsx! {
            Carousel { total_slides: 3,
                CarouselContent {
                    CarouselItem { "1" }
                }
                CarouselPrevious {}
                CarouselNext {}
            }
        }
    }

    let html = render(App);
    assert!(html.contains("Previous"), "previous text: {html}");
    assert!(html.contains("Next"), "next text: {html}");
}

#[test]
fn full_composition() {
    fn App() -> Element {
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

    let html = render(App);
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
