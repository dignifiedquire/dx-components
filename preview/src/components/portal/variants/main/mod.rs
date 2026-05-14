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
            // Test 1: Portal is a no-op pass-through — children render
            // inline where Portal sits in the tree.
            // ---------------------------------------------------------
            section {
                "data-testid": "portal-base",
                h3 { "Base Portal (renders inline)" }
                div {
                    "data-testid": "inline-container",
                    style: "border: 1px solid gray; padding: 8px;",
                    p { "Portal renders its children right here — no DOM re-parenting." }
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
                                p { "This content is rendered through Portal." }
                                p { "It appears inline, inside its parent section." }
                            }
                        }
                    }
                }
            }

            // ---------------------------------------------------------
            // Test 2: Multiple portals render inline in source order.
            // ---------------------------------------------------------
            section {
                "data-testid": "portal-multi",
                h3 { "Multiple Portals (inline, in order)" }
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
        }
    }
}
