use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(String::new);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("../../style.css") }
        Combobox {
            value: value(),
            on_value_change: move |v: String| value.set(v),
            ComboboxInput { placeholder: "Select framework..." }
            ComboboxContent {
                ComboboxList {
                    ComboboxEmpty { "No framework found." }
                    ComboboxItem { value: "next", text_value: "Next.js", "Next.js" }
                    ComboboxItem { value: "sveltekit", text_value: "SvelteKit", "SvelteKit" }
                    ComboboxItem { value: "nuxt", text_value: "Nuxt.js", "Nuxt.js" }
                    ComboboxItem { value: "remix", text_value: "Remix", "Remix" }
                    ComboboxItem { value: "astro", text_value: "Astro", "Astro" }
                }
            }
        }
    }
}
