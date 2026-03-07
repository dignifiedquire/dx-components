//! SSR snapshot tests for the slider primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::slider::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

// ---------------------------------------------------------------------------
// Slider root
// ---------------------------------------------------------------------------

#[test]
fn root_renders_span_element() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Test",
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="slider""#),
        "root has data-slot: {html}"
    );
    // Radix renders a <span>, not <div>
    assert!(html.contains("<span"), "root renders as span: {html}");
    // Should NOT have role="group" (Radix doesn't)
    assert!(
        !html.contains(r#"role="group""#),
        "root should not have role=group: {html}"
    );
}

#[test]
fn root_has_orientation() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Test",
                horizontal: true,
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-orientation="horizontal""#),
        "root has horizontal orientation: {html}"
    );
}

#[test]
fn root_vertical_orientation() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Test",
                horizontal: false,
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-orientation="vertical""#),
        "root has vertical orientation: {html}"
    );
}

#[test]
fn root_disabled_attributes() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Test",
                disabled: true,
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-disabled="""#) || html.contains("data-disabled=\"\""),
        "root has data-disabled when disabled: {html}"
    );
    assert!(
        html.contains("aria-disabled"),
        "root has aria-disabled when disabled: {html}"
    );
}

#[test]
fn root_not_disabled_no_data_disabled() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Test",
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains("data-disabled"),
        "root should not have data-disabled when not disabled: {html}"
    );
}

// ---------------------------------------------------------------------------
// SliderTrack
// ---------------------------------------------------------------------------

#[test]
fn track_attributes() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Test",
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="slider-track""#),
        "track has data-slot: {html}"
    );
    assert!(
        html.contains(r#"data-orientation="horizontal""#),
        "track has orientation: {html}"
    );
}

// ---------------------------------------------------------------------------
// SliderRange
// ---------------------------------------------------------------------------

#[test]
fn range_attributes() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Test",
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="slider-range""#),
        "range has data-slot: {html}"
    );
}

// ---------------------------------------------------------------------------
// SliderThumb
// ---------------------------------------------------------------------------

#[test]
fn thumb_attributes() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Test",
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="slider-thumb""#),
        "thumb has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="slider""#),
        "thumb has role=slider: {html}"
    );
    // Radix renders <span> not <button>
    assert!(
        !html.contains(r#"type="button""#),
        "thumb should not have type=button: {html}"
    );
    assert!(
        html.contains("aria-valuemin"),
        "thumb has aria-valuemin: {html}"
    );
    assert!(
        html.contains("aria-valuemax"),
        "thumb has aria-valuemax: {html}"
    );
    assert!(
        html.contains("aria-valuenow"),
        "thumb has aria-valuenow: {html}"
    );
    assert!(
        html.contains("aria-orientation"),
        "thumb has aria-orientation: {html}"
    );
}

#[test]
fn thumb_disabled_has_no_tabindex() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Test",
                disabled: true,
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }

    let html = render(App);
    // When disabled, tabindex should not be set (Radix: tabIndex={disabled ? undefined : 0})
    assert!(
        !html.contains("tabindex"),
        "thumb should not have tabindex when disabled: {html}"
    );
    assert!(
        html.contains(r#"aria-disabled="true""#),
        "thumb has aria-disabled when disabled: {html}"
    );
}

#[test]
fn thumb_enabled_has_tabindex() {
    fn App() -> Element {
        rsx! {
            Slider {
                label: "Test",
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("tabindex"),
        "thumb should have tabindex when enabled: {html}"
    );
}
