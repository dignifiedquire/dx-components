use dioxus::prelude::*;

use crate::ui::code_block::CopyButton;
use crate::Route;

#[component]
pub(crate) fn Home() -> Element {
    rsx! {
        main { class: "flex flex-col items-center justify-center",
            // Hero
            div { class: "flex flex-col items-center justify-center gap-4 py-20 px-6 text-center max-w-3xl mx-auto",
                h1 { class: "text-4xl sm:text-5xl font-bold tracking-tight text-foreground",
                    "Build your component library"
                }
                p { class: "text-lg text-muted-foreground max-w-[42rem]",
                    "Accessible, customizable components for Dioxus. "
                    "Copy and paste into your apps. Open source."
                }
                div { class: "flex gap-4 mt-4",
                    Link {
                        to: Route::component("button"),
                        class: "inline-flex items-center justify-center rounded-md bg-primary px-6 py-2.5 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity",
                        "Browse Components"
                    }
                    Link {
                        to: "https://github.com/DioxusLabs/components",
                        class: "inline-flex items-center justify-center rounded-md border border-border px-6 py-2.5 text-sm font-medium text-foreground hover:bg-accent transition-colors",
                        "GitHub"
                    }
                }
            }
            // CLI install snippet
            div { class: "w-full max-w-lg mx-auto px-6 pb-20",
                div { class: "relative flex items-center rounded-lg bg-muted border border-border px-4 py-3 font-mono text-sm",
                    span { class: "text-muted-foreground mr-2", "$" }
                    span { class: "text-foreground", "dx components add button" }
                    CopyButton { position: "absolute", top: "0.5em", right: "0.5em" }
                }
            }
        }
    }
}
