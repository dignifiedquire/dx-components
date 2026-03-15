use dioxus::prelude::*;
use dioxus_primitives::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

/// Demo for use_presence — tested through Collapsible which uses it internally.
///
/// Matches upstream presence.stories.tsx scenarios:
/// 1. Basic (no animation) — instant show/hide
/// 2. Unmount animation only — fade-out on close (exit animation keeps element visible)
/// 3. Open and close animation — fade-in on open, fade-out on close
/// 4. Multiple animations — fade + slide on open/close
#[component]
pub fn Demo() -> Element {
    let mut basic_open = use_signal(|| false);
    let mut unmount_open = use_signal(|| false);
    let mut open_close_open = use_signal(|| false);
    let mut multi_open = use_signal(|| false);

    rsx! {
        div {
            "data-testid": "presence-demos",
            style: "display: flex; flex-direction: column; gap: 2rem; padding: 2rem;",

            // --- Test 1: Basic (no animation) ---
            // Upstream: presence.stories.tsx "Basic"
            section {
                "data-testid": "basic-section",
                h3 { "Basic (no animation)" }
                p { "Content appears/disappears instantly without animation." }

                Collapsible {
                    open: basic_open(),
                    on_open_change: move |v: bool| basic_open.set(v),

                    CollapsibleTrigger {
                        button {
                            "data-testid": "basic-trigger",
                            "Toggle"
                        }
                    }
                    CollapsibleContent {
                        div {
                            "data-testid": "basic-content",
                            style: "padding: 1rem; background: #f0f0f0; margin-top: 0.5rem;",
                            "Basic content — no animation"
                        }
                    }
                }
            }

            // --- Test 2: Unmount animation ---
            // Upstream: presence.stories.tsx "WithUnmountAnimation"
            section {
                "data-testid": "unmount-section",
                h3 { "Unmount Animation" }
                p { "Content fades out when closing (presence stays during exit animation)." }

                Collapsible {
                    open: unmount_open(),
                    on_open_change: move |v: bool| unmount_open.set(v),

                    CollapsibleTrigger {
                        button {
                            "data-testid": "unmount-trigger",
                            "Toggle"
                        }
                    }
                    CollapsibleContent {
                        class: "unmount-animation",
                        div {
                            "data-testid": "unmount-content",
                            style: "padding: 1rem; background: #e0f0ff; margin-top: 0.5rem;",
                            "This content fades out on close"
                        }
                    }
                }
            }

            // --- Test 3: Open and close animation ---
            // Upstream: presence.stories.tsx "WithOpenAndCloseAnimation"
            section {
                "data-testid": "open-close-section",
                h3 { "Open & Close Animation" }
                p { "Content fades in when opening and fades out when closing." }

                Collapsible {
                    open: open_close_open(),
                    on_open_change: move |v: bool| open_close_open.set(v),

                    CollapsibleTrigger {
                        button {
                            "data-testid": "open-close-trigger",
                            "Toggle"
                        }
                    }
                    CollapsibleContent {
                        class: "open-close-animation",
                        div {
                            "data-testid": "open-close-content",
                            style: "padding: 1rem; background: #f0ffe0; margin-top: 0.5rem;",
                            "This content fades in and out"
                        }
                    }
                }
            }

            // --- Test 4: Multiple animations ---
            // Upstream: presence.stories.tsx "WithMultipleOpenAndCloseAnimations"
            section {
                "data-testid": "multi-section",
                h3 { "Multiple Animations" }
                p { "Content fades and slides on open/close." }

                Collapsible {
                    open: multi_open(),
                    on_open_change: move |v: bool| multi_open.set(v),

                    CollapsibleTrigger {
                        button {
                            "data-testid": "multi-trigger",
                            "Toggle"
                        }
                    }
                    CollapsibleContent {
                        class: "multiple-animations",
                        div {
                            "data-testid": "multi-content",
                            style: "padding: 1rem; background: #fff0e0; margin-top: 0.5rem;",
                            "This content fades and slides"
                        }
                    }
                }
            }
        }
    }
}
