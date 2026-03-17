use dioxus::prelude::*;
use dioxus_primitives::arrow::Arrow;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            "data-testid": "arrow-demos",
            class: "flex flex-col gap-8 p-4",

            // ---------------------------------------------------------
            // Test 1: Styled arrow (default with fill)
            // Upstream: arrow.stories.tsx "Styled"
            // ---------------------------------------------------------
            section {
                "data-testid": "styled",
                h3 { "Styled" }
                Arrow {
                    width: 20.0,
                    height: 10.0,
                    style: "fill: crimson; vertical-align: middle;",
                    "data-testid": "styled-arrow",
                }
            }

            // ---------------------------------------------------------
            // Test 2: Custom sizes
            // Upstream: arrow.stories.tsx "CustomSizes"
            // ---------------------------------------------------------
            section {
                "data-testid": "custom-sizes",
                h3 { "Custom Sizes" }
                div { style: "display: flex; gap: 16px; align-items: center;",
                    Arrow {
                        width: 40.0,
                        height: 10.0,
                        style: "vertical-align: middle;",
                        "data-testid": "arrow-40x10",
                    }
                    Arrow {
                        width: 50.0,
                        height: 30.0,
                        style: "vertical-align: middle;",
                        "data-testid": "arrow-50x30",
                    }
                    Arrow {
                        width: 20.0,
                        height: 100.0,
                        style: "vertical-align: middle;",
                        "data-testid": "arrow-20x100",
                    }
                }
            }

            // ---------------------------------------------------------
            // Test 3: Default size (width=10, height=5)
            // Upstream: arrow.test.tsx verifies width/height attributes
            // ---------------------------------------------------------
            section {
                "data-testid": "default-arrow",
                h3 { "Default Size" }
                Arrow {
                    "data-testid": "arrow-default",
                }
            }

            // ---------------------------------------------------------
            // Test 4: Styled with CSS fill
            // Custom children not supported due to Dioxus SVG conditional
            // rendering limitation — use CSS to style the arrow instead.
            // ---------------------------------------------------------
            section {
                "data-testid": "css-styled",
                h3 { "CSS Styled Arrow" }
                Arrow {
                    width: 20.0,
                    height: 10.0,
                    style: "fill: tomato;",
                    "data-testid": "arrow-css-styled",
                }
            }
        }
    }
}
