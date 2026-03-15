use dioxus::prelude::*;
use dioxus_primitives::dismissable_layer::{DismissableEvent, DismissableLayer};

#[component]
pub fn Demo() -> Element {
    // ---------------------------------------------------------------
    // Test 1: Basic dismissable layer (escape + outside click)
    // Upstream: dismissable-layer.stories.tsx "Basic"
    // ---------------------------------------------------------------
    let mut basic_open = use_signal(|| false);
    let mut basic_dismiss_count = use_signal(|| 0u32);

    // ---------------------------------------------------------------
    // Test 2: Escape-only dismissal
    // ---------------------------------------------------------------
    let mut escape_open = use_signal(|| false);
    let mut escape_count = use_signal(|| 0u32);

    // ---------------------------------------------------------------
    // Test 3: Nested dismissable layers
    // Upstream: dismissable-layer.stories.tsx "Nested"
    // ---------------------------------------------------------------
    let mut nested_outer_open = use_signal(|| false);
    let mut nested_inner_open = use_signal(|| false);
    let mut nested_outer_dismiss = use_signal(|| 0u32);
    let mut nested_inner_dismiss = use_signal(|| 0u32);

    // ---------------------------------------------------------------
    // Test 4: Prevention — escape prevented, pointer-down-outside not prevented
    // Upstream: dismissable-layer.stories.tsx "Basic" with checkboxes
    // ---------------------------------------------------------------
    let mut prevent_open = use_signal(|| false);
    let mut prevent_escape_count = use_signal(|| 0u32);
    let mut prevent_pointer_count = use_signal(|| 0u32);

    // ---------------------------------------------------------------
    // Test 5: disableOutsidePointerEvents
    // Upstream: dismissable-layer.stories.tsx "Basic" disableOutsidePointerEvents checkbox
    // ---------------------------------------------------------------
    let mut disable_pe_open = use_signal(|| false);

    rsx! {
        div {
            "data-testid": "dismissable-layer-demos",
            class: "flex flex-col gap-8 p-4",

            // Test 1: Basic open/close + escape + outside click
            section {
                "data-testid": "basic-dismiss",
                h3 { "Basic Dismissable Layer" }
                div { style: "margin-bottom: 8px;",
                    button {
                        "data-testid": "basic-trigger",
                        r#type: "button",
                        onclick: move |_| basic_open.set(true),
                        "Open layer"
                    }
                }
                if basic_open() {
                    DismissableLayer {
                        on_dismiss: move |_| {
                            basic_open.set(false);
                            basic_dismiss_count += 1;
                        },
                        div {
                            "data-testid": "basic-layer",
                            style: "display: inline-flex; justify-content: center; align-items: center; width: 300px; height: 200px; background: #333; border-radius: 8px; color: white; flex-direction: column; gap: 8px;",
                            p { "Dismissable content" }
                            input {
                                "data-testid": "basic-layer-input",
                                r#type: "text",
                                placeholder: "Type here",
                            }
                            button {
                                "data-testid": "basic-close",
                                r#type: "button",
                                onclick: move |_| basic_open.set(false),
                                "Close"
                            }
                        }
                    }
                }
                div { style: "margin-top: 8px;",
                    input {
                        "data-testid": "basic-outside-input",
                        r#type: "text",
                        value: "outside input",
                    }
                    {" "}
                    button {
                        "data-testid": "basic-outside-btn",
                        r#type: "button",
                        "Outside button"
                    }
                }
                p {
                    "data-testid": "basic-dismiss-count",
                    "Dismiss count: {basic_dismiss_count}"
                }
            }

            // Test 2: Escape key only
            section {
                "data-testid": "escape-dismiss",
                h3 { "Escape Key Dismissal" }
                button {
                    "data-testid": "escape-trigger",
                    r#type: "button",
                    onclick: move |_| escape_open.set(true),
                    "Open layer"
                }
                if escape_open() {
                    DismissableLayer {
                        on_escape_key_down: move |_| {
                            escape_count += 1;
                        },
                        on_dismiss: move |_| {
                            escape_open.set(false);
                        },
                        div {
                            "data-testid": "escape-layer",
                            style: "display: inline-flex; padding: 20px; background: #666; border-radius: 8px; color: white;",
                            tabindex: "0",
                            "Press Escape to dismiss"
                        }
                    }
                }
                p {
                    "data-testid": "escape-count",
                    "Escape count: {escape_count}"
                }
            }

            // Test 3: Nested layers — only topmost responds to Escape
            section {
                "data-testid": "nested-dismiss",
                h3 { "Nested Dismissable Layers" }
                button {
                    "data-testid": "nested-outer-trigger",
                    r#type: "button",
                    onclick: move |_| nested_outer_open.set(true),
                    "Open outer"
                }
                if nested_outer_open() {
                    DismissableLayer {
                        on_dismiss: move |_| {
                            nested_outer_open.set(false);
                            nested_inner_open.set(false);
                            nested_outer_dismiss += 1;
                        },
                        div {
                            "data-testid": "nested-outer-layer",
                            style: "display: inline-block; padding: 20px; background: rgba(0,0,0,0.2); border-radius: 8px;",
                            p { "Outer layer" }
                            button {
                                "data-testid": "nested-inner-trigger",
                                r#type: "button",
                                onclick: move |_| nested_inner_open.set(true),
                                "Open inner"
                            }
                            if nested_inner_open() {
                                DismissableLayer {
                                    on_dismiss: move |_| {
                                        nested_inner_open.set(false);
                                        nested_inner_dismiss += 1;
                                    },
                                    div {
                                        "data-testid": "nested-inner-layer",
                                        style: "display: inline-block; padding: 20px; background: rgba(0,0,0,0.3); border-radius: 8px; margin-top: 8px;",
                                        p { "Inner layer" }
                                        input {
                                            "data-testid": "nested-inner-input",
                                            r#type: "text",
                                            placeholder: "Inner input",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                button {
                    "data-testid": "nested-outside-btn",
                    r#type: "button",
                    "Outside button"
                }
                p {
                    "data-testid": "nested-outer-dismiss-count",
                    "Outer dismiss count: {nested_outer_dismiss}"
                }
                p {
                    "data-testid": "nested-inner-dismiss-count",
                    "Inner dismiss count: {nested_inner_dismiss}"
                }
            }

            // Test 4: Prevention — escape is prevented, pointer-down-outside is not
            // Upstream: Basic story with dismiss checkboxes
            section {
                "data-testid": "prevent-dismiss",
                h3 { "Prevention (escape prevented, pointer not prevented)" }
                button {
                    "data-testid": "prevent-trigger",
                    r#type: "button",
                    onclick: move |_| prevent_open.set(true),
                    "Open layer"
                }
                if prevent_open() {
                    DismissableLayer {
                        on_escape_key_down: move |event: DismissableEvent| {
                            prevent_escape_count += 1;
                            // Prevent dismissal on escape
                            event.prevent_default();
                        },
                        on_pointer_down_outside: move |_: DismissableEvent| {
                            prevent_pointer_count += 1;
                            // Don't prevent — allow dismissal on pointer down outside
                        },
                        on_dismiss: move |_| {
                            prevent_open.set(false);
                        },
                        div {
                            "data-testid": "prevent-layer",
                            style: "display: inline-flex; padding: 20px; background: #946; border-radius: 8px; color: white;",
                            "Escape is prevented, click outside is not"
                        }
                    }
                }
                p {
                    "data-testid": "prevent-escape-count",
                    "Escape callback count: {prevent_escape_count}"
                }
                p {
                    "data-testid": "prevent-pointer-count",
                    "Pointer outside callback count: {prevent_pointer_count}"
                }
                button {
                    "data-testid": "prevent-outside-btn",
                    r#type: "button",
                    "Outside button"
                }
            }

            // Test 5: disableOutsidePointerEvents
            // Upstream: Basic story with disableOutsidePointerEvents checkbox
            section {
                "data-testid": "disable-pe-dismiss",
                h3 { "Disable Outside Pointer Events" }
                button {
                    "data-testid": "disable-pe-trigger",
                    r#type: "button",
                    onclick: move |_| disable_pe_open.set(true),
                    "Open layer"
                }
                if disable_pe_open() {
                    DismissableLayer {
                        disable_outside_pointer_events: true,
                        on_dismiss: move |_| {
                            disable_pe_open.set(false);
                        },
                        div {
                            "data-testid": "disable-pe-layer",
                            style: "display: inline-flex; padding: 20px; background: #469; border-radius: 8px; color: white;",
                            "Outside pointer events disabled"
                        }
                    }
                }
                button {
                    "data-testid": "disable-pe-outside-btn",
                    r#type: "button",
                    onclick: move |_| {},
                    "Outside button (should be unclickable)"
                }
            }
        }
    }
}
