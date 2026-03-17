use dioxus::prelude::*;
use dioxus_primitives::announce::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            "data-testid": "announce-demos",
            class: "flex flex-col gap-12 p-8",

            // 1. Polite announcement (default)
            section {
                "data-testid": "polite",
                h3 { class: "text-lg font-semibold mb-4", "Polite (default)" }
                Announce {
                    "data-testid": "announce-polite",
                    "Status update: task completed"
                }
            }

            // 2. Assertive announcement
            section {
                "data-testid": "assertive",
                h3 { class: "text-lg font-semibold mb-4", "Assertive" }
                Announce {
                    r#type: AnnounceType::Assertive,
                    "data-testid": "announce-assertive",
                    "Error: form submission failed"
                }
            }

            // 3. Off (no announcement)
            section {
                "data-testid": "off",
                h3 { class: "text-lg font-semibold mb-4", "Off" }
                Announce {
                    r#type: AnnounceType::Off,
                    "data-testid": "announce-off",
                    "This is not announced"
                }
            }

            // 4. Custom role (log)
            section {
                "data-testid": "log-role",
                h3 { class: "text-lg font-semibold mb-4", "Log Role" }
                Announce {
                    r#type: AnnounceType::Polite,
                    role: RegionRole::Log,
                    "data-testid": "announce-log",
                    "Log entry: user signed in"
                }
            }

            // 5. With region identifier
            section {
                "data-testid": "with-identifier",
                h3 { class: "text-lg font-semibold mb-4", "Region Identifier" }
                Announce {
                    region_identifier: "toast-region".to_string(),
                    "data-testid": "announce-identified",
                    "Toast: saved successfully"
                }
            }

            // 6. Atomic
            section {
                "data-testid": "atomic",
                h3 { class: "text-lg font-semibold mb-4", "Atomic" }
                Announce {
                    aria_atomic: true,
                    "data-testid": "announce-atomic",
                    span { "Count: " }
                    span { "42" }
                }
            }

            // 7. With aria-relevant
            section {
                "data-testid": "relevant",
                h3 { class: "text-lg font-semibold mb-4", "Aria-Relevant" }
                Announce {
                    aria_relevant: "additions removals".to_string(),
                    "data-testid": "announce-relevant",
                    "Tracking additions and removals"
                }
            }
        }
    }
}
