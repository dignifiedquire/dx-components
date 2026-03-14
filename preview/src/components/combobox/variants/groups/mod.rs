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
            ComboboxInput { placeholder: "Select timezone..." }
            ComboboxContent {
                ComboboxList {
                    ComboboxEmpty { "No timezone found." }
                    ComboboxGroup { heading: "Americas",
                        ComboboxItem { value: "est", text_value: "Eastern Standard Time (EST)", "Eastern Standard Time (EST)" }
                        ComboboxItem { value: "cst", text_value: "Central Standard Time (CST)", "Central Standard Time (CST)" }
                        ComboboxItem { value: "pst", text_value: "Pacific Standard Time (PST)", "Pacific Standard Time (PST)" }
                    }
                    ComboboxSeparator {}
                    ComboboxGroup { heading: "Europe",
                        ComboboxItem { value: "gmt", text_value: "Greenwich Mean Time (GMT)", "Greenwich Mean Time (GMT)" }
                        ComboboxItem { value: "cet", text_value: "Central European Time (CET)", "Central European Time (CET)" }
                        ComboboxItem { value: "eet", text_value: "Eastern European Time (EET)", "Eastern European Time (EET)" }
                    }
                    ComboboxSeparator {}
                    ComboboxGroup { heading: "Asia / Pacific",
                        ComboboxItem { value: "ist", text_value: "India Standard Time (IST)", "India Standard Time (IST)" }
                        ComboboxItem { value: "jst", text_value: "Japan Standard Time (JST)", "Japan Standard Time (JST)" }
                        ComboboxItem { value: "aest", text_value: "Australian Eastern Standard Time (AEST)", "Australian Eastern Standard Time (AEST)" }
                    }
                }
            }
        }
    }
}
