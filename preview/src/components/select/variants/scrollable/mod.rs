use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Select { placeholder: "Select a timezone...",
            SelectTrigger { aria_label: "Timezone", width: "17rem", SelectValue {} }
            SelectContent { aria_label: "Timezones",
                SelectGroup {
                    SelectLabel { "North America" }
                    SelectItem { value: "est", text_value: "Eastern Standard Time (EST)", "Eastern Standard Time (EST)" }
                    SelectItem { value: "cst", text_value: "Central Standard Time (CST)", "Central Standard Time (CST)" }
                    SelectItem { value: "mst", text_value: "Mountain Standard Time (MST)", "Mountain Standard Time (MST)" }
                    SelectItem { value: "pst", text_value: "Pacific Standard Time (PST)", "Pacific Standard Time (PST)" }
                    SelectItem { value: "akst", text_value: "Alaska Standard Time (AKST)", "Alaska Standard Time (AKST)" }
                    SelectItem { value: "hst", text_value: "Hawaii Standard Time (HST)", "Hawaii Standard Time (HST)" }
                }
                SelectSeparator {}
                SelectGroup {
                    SelectLabel { "Europe & Africa" }
                    SelectItem { value: "gmt", text_value: "Greenwich Mean Time (GMT)", "Greenwich Mean Time (GMT)" }
                    SelectItem { value: "cet", text_value: "Central European Time (CET)", "Central European Time (CET)" }
                    SelectItem { value: "eet", text_value: "Eastern European Time (EET)", "Eastern European Time (EET)" }
                    SelectItem { value: "west", text_value: "Western European Summer Time (WEST)", "Western European Summer Time (WEST)" }
                    SelectItem { value: "cat", text_value: "Central Africa Time (CAT)", "Central Africa Time (CAT)" }
                    SelectItem { value: "eat", text_value: "East Africa Time (EAT)", "East Africa Time (EAT)" }
                }
                SelectSeparator {}
                SelectGroup {
                    SelectLabel { "Asia" }
                    SelectItem { value: "msk", text_value: "Moscow Time (MSK)", "Moscow Time (MSK)" }
                    SelectItem { value: "ist", text_value: "India Standard Time (IST)", "India Standard Time (IST)" }
                    SelectItem { value: "cst_china", text_value: "China Standard Time (CST)", "China Standard Time (CST)" }
                    SelectItem { value: "jst", text_value: "Japan Standard Time (JST)", "Japan Standard Time (JST)" }
                    SelectItem { value: "kst", text_value: "Korea Standard Time (KST)", "Korea Standard Time (KST)" }
                    SelectItem { value: "wib", text_value: "Indonesia Western Time (WIB)", "Indonesia Western Time (WIB)" }
                }
            }
        }
    }
}
