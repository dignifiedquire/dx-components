use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Select::<Option<String>> { placeholder: "Select a timezone...",
            SelectTrigger { aria_label: "Timezone", width: "17rem", SelectValue {} }
            SelectContent { aria_label: "Timezones",
                SelectGroup {
                    SelectLabel { "North America" }
                    SelectItem::<Option<String>> { index: 0usize, value: "est".to_string(), "Eastern Standard Time (EST)" }
                    SelectItem::<Option<String>> { index: 1usize, value: "cst".to_string(), "Central Standard Time (CST)" }
                    SelectItem::<Option<String>> { index: 2usize, value: "mst".to_string(), "Mountain Standard Time (MST)" }
                    SelectItem::<Option<String>> { index: 3usize, value: "pst".to_string(), "Pacific Standard Time (PST)" }
                    SelectItem::<Option<String>> { index: 4usize, value: "akst".to_string(), "Alaska Standard Time (AKST)" }
                    SelectItem::<Option<String>> { index: 5usize, value: "hst".to_string(), "Hawaii Standard Time (HST)" }
                }
                SelectSeparator {}
                SelectGroup {
                    SelectLabel { "Europe & Africa" }
                    SelectItem::<Option<String>> { index: 6usize, value: "gmt".to_string(), "Greenwich Mean Time (GMT)" }
                    SelectItem::<Option<String>> { index: 7usize, value: "cet".to_string(), "Central European Time (CET)" }
                    SelectItem::<Option<String>> { index: 8usize, value: "eet".to_string(), "Eastern European Time (EET)" }
                    SelectItem::<Option<String>> { index: 9usize, value: "west".to_string(), "Western European Summer Time (WEST)" }
                    SelectItem::<Option<String>> { index: 10usize, value: "cat".to_string(), "Central Africa Time (CAT)" }
                    SelectItem::<Option<String>> { index: 11usize, value: "eat".to_string(), "East Africa Time (EAT)" }
                }
                SelectSeparator {}
                SelectGroup {
                    SelectLabel { "Asia" }
                    SelectItem::<Option<String>> { index: 12usize, value: "msk".to_string(), "Moscow Time (MSK)" }
                    SelectItem::<Option<String>> { index: 13usize, value: "ist".to_string(), "India Standard Time (IST)" }
                    SelectItem::<Option<String>> { index: 14usize, value: "cst_china".to_string(), "China Standard Time (CST)" }
                    SelectItem::<Option<String>> { index: 15usize, value: "jst".to_string(), "Japan Standard Time (JST)" }
                    SelectItem::<Option<String>> { index: 16usize, value: "kst".to_string(), "Korea Standard Time (KST)" }
                    SelectItem::<Option<String>> { index: 17usize, value: "wib".to_string(), "Indonesia Western Time (WIB)" }
                }
            }
        }
    }
}
