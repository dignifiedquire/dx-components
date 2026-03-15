use dioxus::prelude::*;
use dioxus_primitives::focus_scope::FocusScope;

#[component]
pub fn Demo() -> Element {
    // ---------------------------------------------------------------
    // Test 1: Basic focus trapping + looping
    // Upstream: focus-scope.test.tsx "given a default FocusScope"
    // ---------------------------------------------------------------
    let mut trapped = use_signal(|| false);

    // ---------------------------------------------------------------
    // Test 2: Negative tabindex skipping
    // Upstream: focus-scope.test.tsx "first focusable has a negative tabindex"
    // ---------------------------------------------------------------
    let mut trapped_neg = use_signal(|| false);

    // ---------------------------------------------------------------
    // Test 3: Nested focus scopes
    // Upstream: focus-scope.stories.tsx "Multiple"
    // ---------------------------------------------------------------
    let mut trapped1 = use_signal(|| false);
    let mut trapped2 = use_signal(|| false);

    rsx! {
        div {
            "data-testid": "focus-scope-demos",
            class: "flex flex-col gap-8 p-4",

            // Test 1: Basic trapped + looping scope
            section {
                "data-testid": "basic-trap",
                h3 { "Basic Focus Trap" }
                div {
                    button {
                        "data-testid": "trap-trigger",
                        r#type: "button",
                        onclick: move |_| trapped.set(true),
                        "Trap"
                    }
                    {" "}
                    input { "data-testid": "outside-input-1" }
                    {" "}
                    input { "data-testid": "outside-input-2" }
                }
                if trapped() {
                    FocusScope {
                        r#loop: true,
                        trapped: true,
                        div {
                            "data-testid": "trap-scope",
                            style: "display: inline-flex; flex-direction: column; gap: 8px; padding: 16px; border: 2px solid black;",
                            input {
                                "data-testid": "name-input",
                                r#type: "text",
                                placeholder: "Name",
                            }
                            input {
                                "data-testid": "email-input",
                                r#type: "text",
                                placeholder: "Email",
                            }
                            button {
                                "data-testid": "submit-btn",
                                r#type: "button",
                                "Submit"
                            }
                            button {
                                "data-testid": "close-trap",
                                r#type: "button",
                                onclick: move |_| trapped.set(false),
                                "Close"
                            }
                        }
                    }
                }
                div {
                    input { "data-testid": "outside-input-3" }
                    {" "}
                    input { "data-testid": "outside-input-4" }
                }
            }

            // Test 2: Negative tabindex skipping
            section {
                "data-testid": "negative-tabindex",
                h3 { "Negative Tabindex Skipping" }
                button {
                    "data-testid": "neg-trap-trigger",
                    r#type: "button",
                    onclick: move |_| trapped_neg.set(true),
                    "Trap with negative tabindex"
                }
                if trapped_neg() {
                    FocusScope {
                        r#loop: true,
                        trapped: true,
                        div {
                            "data-testid": "neg-trap-scope",
                            style: "display: inline-flex; flex-direction: column; gap: 8px; padding: 16px; border: 2px solid blue;",
                            input {
                                "data-testid": "neg-name-input",
                                r#type: "text",
                                placeholder: "Name (tabindex -1)",
                                tabindex: "-1",
                            }
                            input {
                                "data-testid": "neg-email-input",
                                r#type: "text",
                                placeholder: "Email",
                            }
                            button {
                                "data-testid": "neg-submit-btn",
                                r#type: "button",
                                "Submit"
                            }
                            button {
                                "data-testid": "neg-close",
                                r#type: "button",
                                onclick: move |_| trapped_neg.set(false),
                                "Close"
                            }
                        }
                    }
                }
            }

            // Test 3: Multiple nested focus scopes
            section {
                "data-testid": "multiple-scopes",
                h3 { "Multiple Focus Scopes" }
                div {
                    style: "display: flex; flex-direction: column; gap: 10px;",
                    button {
                        "data-testid": "multi-trap1-trigger",
                        r#type: "button",
                        onclick: move |_| trapped1.set(true),
                        "Trap 1"
                    }
                    if trapped1() {
                        FocusScope {
                            r#loop: true,
                            trapped: true,
                            div {
                                "data-testid": "multi-scope-1",
                                style: "display: inline-flex; flex-direction: column; gap: 8px; padding: 16px; border: 2px solid green;",
                                h4 { "Scope One" }
                                input {
                                    "data-testid": "multi1-first",
                                    r#type: "text",
                                    placeholder: "First name",
                                }
                                input {
                                    "data-testid": "multi1-last",
                                    r#type: "text",
                                    placeholder: "Last name",
                                }
                                button {
                                    "data-testid": "multi1-close",
                                    r#type: "button",
                                    onclick: move |_| trapped1.set(false),
                                    "Close"
                                }
                            }
                        }
                    }

                    button {
                        "data-testid": "multi-trap2-trigger",
                        r#type: "button",
                        onclick: move |_| trapped2.set(true),
                        "Trap 2"
                    }
                    if trapped2() {
                        FocusScope {
                            r#loop: true,
                            trapped: true,
                            div {
                                "data-testid": "multi-scope-2",
                                style: "display: inline-flex; flex-direction: column; gap: 8px; padding: 16px; border: 2px solid red;",
                                h4 { "Scope Two" }
                                input {
                                    "data-testid": "multi2-first",
                                    r#type: "text",
                                    placeholder: "First name",
                                }
                                input {
                                    "data-testid": "multi2-last",
                                    r#type: "text",
                                    placeholder: "Last name",
                                }
                                button {
                                    "data-testid": "multi2-close",
                                    r#type: "button",
                                    onclick: move |_| trapped2.set(false),
                                    "Close"
                                }
                            }
                        }
                    }

                    input { "data-testid": "multi-outside-input" }
                }
            }
        }
    }
}
