#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::slider::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

#[test]
fn slider_root_classes() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Volume",
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== slider_root_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="slider""#));
    assert!(html.contains("touch-none"));
    assert!(html.contains("select-none"));
}

#[test]
fn slider_track_classes() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Volume",
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="slider-track""#));
    assert!(html.contains("rounded-full bg-muted"));
}

#[test]
fn slider_range_classes() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Volume",
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="slider-range""#));
    assert!(html.contains("bg-primary"));
}

#[test]
fn slider_thumb_classes() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Volume",
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="slider-thumb""#));
    assert!(html.contains(r#"role="slider""#));
    assert!(html.contains("rounded-full"));
    assert!(html.contains("border-primary"));
}

#[test]
fn slider_consumer_class_merge() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Volume",
                class: "my-slider",
                SliderTrack {
                    class: "my-track",
                    SliderRange { class: "my-range" }
                    SliderThumb { class: "my-thumb" }
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains("my-slider"));
    assert!(html.contains("my-track"));
    assert!(html.contains("my-range"));
    assert!(html.contains("my-thumb"));
}

#[test]
fn slider_orientation_attribute() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Volume",
                horizontal: false,
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-orientation="vertical""#));
}
