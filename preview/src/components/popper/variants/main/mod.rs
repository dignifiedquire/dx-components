use dioxus::prelude::*;
use dioxus_primitives::popper::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            "data-testid": "popper-demos",
            class: "flex flex-col gap-12 p-8",

            // 1. Styled — default side (bottom) and align (center)
            section {
                "data-testid": "styled",
                h3 { class: "text-lg font-semibold mb-4", "Styled (Bottom / Center)" }
                div { class: "py-8 flex justify-center",
                    Popper {
                        PopperAnchor {
                            button {
                                class: "px-4 py-2 border-2 border-gray-800 rounded bg-gray-100 dark:bg-gray-700 dark:text-white",
                                "Anchor"
                            }
                        }
                        PopperContent {
                            side_offset: 5.0,
                            div {
                                class: "px-4 py-2 bg-gray-800 text-white rounded text-sm shadow-lg",
                                "Floating content"
                            }
                        }
                    }
                }
            }

            // 2. Side: Top — anchor placed low so content has room above
            section {
                "data-testid": "side-top",
                h3 { class: "text-lg font-semibold mb-4", "Side: Top" }
                div { class: "pt-16 pb-4 flex justify-center",
                    Popper {
                        PopperAnchor {
                            button {
                                class: "px-4 py-2 border-2 border-blue-600 rounded bg-blue-50",
                                "Top"
                            }
                        }
                        PopperContent {
                            side: Side::Top,
                            side_offset: 5.0,
                            div {
                                class: "px-4 py-2 bg-blue-600 text-white rounded text-sm shadow-lg",
                                "Top content"
                            }
                        }
                    }
                }
            }

            // 3. Side: Right — anchor left-aligned so content has room to the right
            section {
                "data-testid": "side-right",
                h3 { class: "text-lg font-semibold mb-4", "Side: Right" }
                div { class: "py-4 flex justify-start pl-4",
                    Popper {
                        PopperAnchor {
                            button {
                                class: "px-4 py-2 border-2 border-green-600 rounded bg-green-50",
                                "Right"
                            }
                        }
                        PopperContent {
                            side: Side::Right,
                            side_offset: 5.0,
                            div {
                                class: "px-4 py-2 bg-green-600 text-white rounded text-sm shadow-lg",
                                "Right content"
                            }
                        }
                    }
                }
            }

            // 4. Side: Left — anchor right-aligned so content has room to the left
            section {
                "data-testid": "side-left",
                h3 { class: "text-lg font-semibold mb-4", "Side: Left" }
                div { class: "py-4 flex justify-end pr-4",
                    Popper {
                        PopperAnchor {
                            button {
                                class: "px-4 py-2 border-2 border-purple-600 rounded bg-purple-50",
                                "Left"
                            }
                        }
                        PopperContent {
                            side: Side::Left,
                            side_offset: 5.0,
                            div {
                                class: "px-4 py-2 bg-purple-600 text-white rounded text-sm shadow-lg",
                                "Left content"
                            }
                        }
                    }
                }
            }

            // 5. With Arrow
            section {
                "data-testid": "with-arrow",
                h3 { class: "text-lg font-semibold mb-4", "With Arrow" }
                div { class: "pt-16 pb-4 flex justify-center",
                    Popper {
                        PopperAnchor {
                            button {
                                class: "px-4 py-2 border-2 border-orange-600 rounded bg-orange-50",
                                "Arrow"
                            }
                        }
                        PopperContent {
                            side: Side::Top,
                            side_offset: 0.0,
                            arrow_width: 20.0,
                            arrow_height: 10.0,
                            div {
                                class: "px-4 py-2 bg-orange-600 text-white rounded text-sm shadow-lg",
                                "With arrow"
                            }
                            PopperArrow {
                                width: 20.0,
                                height: 10.0,
                                style: "fill: #ea580c;",
                            }
                        }
                    }
                }
            }

            // 6. Align: Start
            section {
                "data-testid": "align-start",
                h3 { class: "text-lg font-semibold mb-4", "Align: Start" }
                div { class: "py-4 flex justify-center",
                    Popper {
                        PopperAnchor {
                            button {
                                class: "px-16 py-2 border-2 border-teal-600 rounded bg-teal-50",
                                "Wide anchor"
                            }
                        }
                        PopperContent {
                            side_offset: 5.0,
                            align: Align::Start,
                            div {
                                class: "px-4 py-2 bg-teal-600 text-white rounded text-sm shadow-lg",
                                "Start aligned"
                            }
                        }
                    }
                }
            }

            // 7. Align: End
            section {
                "data-testid": "align-end",
                h3 { class: "text-lg font-semibold mb-4", "Align: End" }
                div { class: "py-4 flex justify-center",
                    Popper {
                        PopperAnchor {
                            button {
                                class: "px-16 py-2 border-2 border-pink-600 rounded bg-pink-50",
                                "Wide anchor"
                            }
                        }
                        PopperContent {
                            side_offset: 5.0,
                            align: Align::End,
                            div {
                                class: "px-4 py-2 bg-pink-600 text-white rounded text-sm shadow-lg",
                                "End aligned"
                            }
                        }
                    }
                }
            }

            // 8. With Portal — content escapes overflow:hidden parent
            section {
                "data-testid": "with-portal",
                h3 { class: "text-lg font-semibold mb-4", "With Portal" }
                div {
                    class: "py-4 flex justify-center",
                    style: "overflow: hidden; position: relative;",
                    Popper {
                        PopperAnchor {
                            button {
                                class: "px-4 py-2 border-2 border-red-600 rounded bg-red-50",
                                "Portal"
                            }
                        }
                        PopperContent {
                            side_offset: 5.0,
                            portal: true,
                            div {
                                class: "px-4 py-2 bg-red-600 text-white rounded text-sm shadow-lg",
                                "data-testid": "portal-content-inner",
                                "Portal content"
                            }
                        }
                    }
                }
            }
        }
    }
}
