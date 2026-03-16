use dioxus::prelude::*;
use dioxus_primitives::portal::Portal;

#[component]
pub fn Demo() -> Element {
    let mut show_base = use_signal(|| false);
    let mut show_multi = use_signal(|| false);

    rsx! {
        div {
            "data-testid": "portal-demos",
            class: "flex flex-col gap-8 p-4",

            // ---------------------------------------------------------
            // Test 1: Base — content escapes overflow container
            // Upstream: portal.stories.tsx "Base"
            // ---------------------------------------------------------
            section {
                "data-testid": "portal-base",
                h3 { "Base Portal" }
                div {
                    style: "max-width: 300px; max-height: 100px; overflow: hidden; border: 1px solid gray; padding: 8px;",
                    "data-testid": "overflow-container",
                    p { "This content is inside an overflow:hidden container." }
                    button {
                        "data-testid": "base-toggle",
                        r#type: "button",
                        onclick: move |_| show_base.toggle(),
                        if show_base() { "Hide portal" } else { "Show portal" }
                    }
                    if show_base() {
                        Portal {
                            div {
                                "data-testid": "base-portal-content",
                                style: "padding: 10px; margin: 10px; border: 2px solid blue; background: #eef;",
                                p { "This content is rendered through a Portal." }
                                p { "It appears in the PortalHost, outside the overflow container." }
                            }
                        }
                    }
                }
            }

            // ---------------------------------------------------------
            // Test 2: Multiple portals — z-index ordering
            // Upstream: portal.stories.tsx "Chromatic" zIndex section
            // ---------------------------------------------------------
            section {
                "data-testid": "portal-multi",
                h3 { "Multiple Portals (ordering)" }
                button {
                    "data-testid": "multi-toggle",
                    r#type: "button",
                    onclick: move |_| show_multi.toggle(),
                    if show_multi() { "Hide portals" } else { "Show portals" }
                }
                if show_multi() {
                    Portal {
                        div {
                            "data-testid": "multi-portal-1",
                            style: "padding: 8px; margin: 4px; border: 2px solid red; background: #fee;",
                            "Portal 1 (red)"
                        }
                    }
                    Portal {
                        div {
                            "data-testid": "multi-portal-2",
                            style: "padding: 8px; margin: 4px; border: 2px solid green; background: #efe;",
                            "Portal 2 (green)"
                        }
                    }
                    Portal {
                        div {
                            "data-testid": "multi-portal-3",
                            style: "padding: 8px; margin: 4px; border: 2px solid blue; background: #eef;",
                            "Portal 3 (blue)"
                        }
                    }
                }
            }

            // ---------------------------------------------------------
            // Test 3: Inline fallback — no PortalHost limitation test
            // This section does NOT use PortalHost, so Portal renders inline.
            // Note: In this demo we DO have a PortalHost below, so this
            // test is verified via a separate check that content appears
            // in the host, not inline.
            // ---------------------------------------------------------
            section {
                "data-testid": "portal-location",
                h3 { "Portal renders in PortalHost (not inline)" }
                p { "The portal content below should NOT appear inside this section." }
                Portal {
                    div {
                        "data-testid": "location-portal-content",
                        style: "padding: 8px; margin: 4px; border: 2px solid purple; background: #fef;",
                        "I should be in the PortalHost"
                    }
                }
            }
        }
        // PortalHost is provided by the app layout — no need to add one here.
    }
}
