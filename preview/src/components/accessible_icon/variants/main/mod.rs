use dioxus::prelude::*;
use dioxus_primitives::accessible_icon::AccessibleIcon;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            "data-testid": "accessible-icon-demos",
            class: "flex flex-col gap-12 p-8",

            // 1. Styled — button with accessible icon (matches upstream Styled story)
            section {
                "data-testid": "styled",
                h3 { class: "text-lg font-semibold mb-4", "Styled" }
                button {
                    "data-testid": "icon-button",
                    class: "inline-flex items-center justify-center w-10 h-10 border rounded hover:bg-gray-100",
                    AccessibleIcon {
                        label: "Close",
                        svg {
                            "data-testid": "icon-svg",
                            view_box: "0 0 32 32",
                            width: "24",
                            height: "24",
                            fill: "none",
                            stroke: "currentColor",
                            path { d: "M2 30 L30 2 M30 30 L2 2" }
                        }
                    }
                }
            }

            // 2. Chromatic — inline icon in text (matches upstream Chromatic story)
            section {
                "data-testid": "chromatic",
                h3 { class: "text-lg font-semibold mb-4", "Inline" }
                p {
                    "data-testid": "inline-text",
                    "Some text with an inline icon "
                    AccessibleIcon {
                        label: "Warning",
                        svg {
                            "data-testid": "inline-svg",
                            view_box: "0 0 32 32",
                            width: "16",
                            height: "16",
                            fill: "none",
                            stroke: "currentColor",
                            path { d: "M2 30 L30 2 M30 30 L2 2" }
                        }
                    }
                    " and more text"
                }
            }
        }
    }
}
