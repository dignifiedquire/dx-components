use dioxus::prelude::*;

use crate::find_block_demo;

#[component]
pub(crate) fn ComponentBlockDemo(name: String, variant: String) -> Element {
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
