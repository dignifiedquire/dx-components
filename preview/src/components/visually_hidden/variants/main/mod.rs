use dioxus::prelude::*;
use dioxus_primitives::visually_hidden::VisuallyHidden;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            "data-testid": "visually-hidden-demos",
            class: "flex flex-col gap-8 p-4",

            // ---------------------------------------------------------
            // Test 1: Basic — button with hidden label and visible icon
            // Upstream: visually-hidden.stories.tsx "Basic"
            // ---------------------------------------------------------
            section {
                "data-testid": "basic",
                h3 { "Basic" }
                button {
                    "data-testid": "save-button",
                    r#type: "button",
                    VisuallyHidden {
                        "data-testid": "hidden-text",
                        "Save the file"
                    }
                    span { "aria-hidden": "true", "data-testid": "visible-icon", "\u{1F4BE}" }
                }
                p { style: "margin-top: 8px; color: #666; font-size: 14px;",
                    "The button above shows only an icon, but screen readers read \"Save the file\"."
                }
            }

            // ---------------------------------------------------------
            // Test 2: Verify hidden styles are applied
            // ---------------------------------------------------------
            section {
                "data-testid": "styles",
                h3 { "Hidden Styles Verification" }
                div { style: "position: relative; border: 1px solid #ccc; padding: 16px; min-height: 50px;",
                    VisuallyHidden {
                        "data-testid": "hidden-span",
                        "This text is visually hidden but accessible"
                    }
                    p { "The hidden content is inside this box (invisible to sighted users)." }
                }
            }
        }
    }
}
