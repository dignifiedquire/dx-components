use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(String::new);

    rsx! {
        Combobox {
            value: value(),
            on_value_change: move |v: String| value.set(v),
            ComboboxInput { placeholder: "Select timezone..." }
            ComboboxContent {
                ComboboxList {
                    ComboboxEmpty { "No timezone found." }
                    ComboboxGroup { heading: "Americas",
                        ComboboxItem { value: "est", "Eastern Standard Time (EST)" }
                        ComboboxItem { value: "cst", "Central Standard Time (CST)" }
                        ComboboxItem { value: "pst", "Pacific Standard Time (PST)" }
                    }
                    ComboboxSeparator {}
                    ComboboxGroup { heading: "Europe",
                        ComboboxItem { value: "gmt", "Greenwich Mean Time (GMT)" }
                        ComboboxItem { value: "cet", "Central European Time (CET)" }
                        ComboboxItem { value: "eet", "Eastern European Time (EET)" }
                    }
                    ComboboxSeparator {}
                    ComboboxGroup { heading: "Asia / Pacific",
                        ComboboxItem { value: "ist", "India Standard Time (IST)" }
                        ComboboxItem { value: "jst", "Japan Standard Time (JST)" }
                        ComboboxItem { value: "aest", "Australian Eastern Standard Time (AEST)" }
                    }
                }
            }
        }
    }
}
