use dioxus::prelude::*;

use crate::find_block_demo;
use crate::layouts::app_layout::use_preload_release;

#[component]
pub(crate) fn ComponentBlockDemo(name: String, variant: String) -> Element {
    // Block demos render outside AppLayout, so they release the preload class
    // themselves — otherwise CSS transitions stay disabled on these pages.
    use_preload_release();

    let Comp = match find_block_demo(&name, &variant) {
        Some(f) => f,
        None => {
            return rsx! {
                div {
                    style: "min-height: 100vh; display: flex; align-items: center; justify-content: center; padding: 2rem;",
                    "Block demo not found: {name}/{variant}"
                }
            };
        }
    };

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/tailwind.css"),
        }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/dx-components-theme.css"),
        }
        div { style: "min-height: 100vh;",
            Comp {}
        }
    }
}
