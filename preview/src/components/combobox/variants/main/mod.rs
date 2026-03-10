use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(String::new);

    rsx! {
        Combobox {
            value: value(),
            on_value_change: move |v: String| value.set(v),
            ComboboxInput { placeholder: "Select framework..." }
            ComboboxContent {
                ComboboxList {
                    ComboboxEmpty { "No framework found." }
                    ComboboxItem { value: "next", "Next.js" }
                    ComboboxItem { value: "sveltekit", "SvelteKit" }
                    ComboboxItem { value: "nuxt", "Nuxt.js" }
                    ComboboxItem { value: "remix", "Remix" }
                    ComboboxItem { value: "astro", "Astro" }
                }
            }
        }
    }
}
