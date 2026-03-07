use dioxus::prelude::*;
use dioxus_primitives::label::Label;
use dioxus_primitives::radio_group::{RadioGroup, RadioGroupIndicator, RadioGroupItem};

#[component]
pub fn Demo() -> Element {
    rsx! {
        RadioGroup {
            default_value: "comfortable".to_string(),
            class: "grid gap-3",
            RadioGroupItemWithLabel { value: "default".to_string(), label: "Default" }
            RadioGroupItemWithLabel { value: "comfortable".to_string(), label: "Comfortable" }
            RadioGroupItemWithLabel { value: "compact".to_string(), label: "Compact" }
        }
    }
}

#[component]
fn RadioGroupItemWithLabel(value: String, label: String) -> Element {
    let id = format!("radio-{value}");
    rsx! {
        div { class: "flex items-center gap-2",
            RadioGroupItem {
                value: value,
                id: id.clone(),
                class: "aspect-square size-4 shrink-0 rounded-full border border-input text-primary shadow-xs focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 aria-checked:bg-primary aria-checked:text-primary-foreground aria-checked:border-primary dark:bg-input/30",
                RadioGroupIndicator {
                    class: "flex items-center justify-center",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "8",
                        height: "8",
                        view_box: "0 0 8 8",
                        fill: "currentColor",
                        circle { cx: "4", cy: "4", r: "4" }
                    }
                }
            }
            Label { html_for: id, "{label}" }
        }
    }
}
