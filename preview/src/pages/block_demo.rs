use dioxus::prelude::*;

use crate::components;

#[component]
pub(crate) fn ComponentBlockDemo(name: String, variant: String) -> Element {
    let Some(demo) = components::DEMOS.iter().find(|d| d.name == name).cloned() else {
        return rsx! {
            div { "Block component not found" }
        };
    };

    let variant = match demo.variants.iter().find(|v| v.name == variant) {
        Some(v) => v,
        None => {
            return rsx! {
                div {
                    style: "min-height: 100vh; display: flex; align-items: center; justify-content: center; padding: 2rem;",
                    "Variant content not found: {variant}"
                }
            };
        }
    };

    let Comp = variant.component;

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/dx-components-theme.css"),
        }
        div { style: "min-height: 100vh;", Comp {} }
    }
}
