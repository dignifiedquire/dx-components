use super::super::component::*;
use dioxus::prelude::*;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumIter, strum::Display)]
enum Fruit {
    Apple,
    Banana,
    Orange,
    Strawberry,
    Watermelon,
}

impl Fruit {
    const fn emoji(&self) -> &'static str {
        match self {
            Fruit::Apple => "🍎",
            Fruit::Banana => "🍌",
            Fruit::Orange => "🍊",
            Fruit::Strawberry => "🍓",
            Fruit::Watermelon => "🍉",
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let fruits = Fruit::iter().map(|f| {
        rsx! {
            SelectItem { value: "{f}", text_value: "{f}",
                {format!("{} {f}", f.emoji())}
            }
        }
    });

    rsx! {
        Select { placeholder: "Select a fruit...",
            SelectTrigger { aria_label: "Select Trigger", width: "12rem", SelectValue {} }
            SelectContent { aria_label: "Select Demo",
                SelectGroup {
                    SelectLabel { "Fruits" }
                    {fruits}
                }
                SelectGroup {
                    SelectLabel { "Other" }
                    SelectItem {
                        value: "",
                        text_value: "Other",
                        "Other"
                    }
                }
            }
        }
    }
}
