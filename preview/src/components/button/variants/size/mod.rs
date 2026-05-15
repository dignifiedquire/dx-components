use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_lucide::IconArrowUpRight;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex flex-col items-start gap-8 sm:flex-row",
            div { class: "flex items-start gap-2",
                Button { size: ButtonSize::Xs, variant: ButtonVariant::Outline, "Extra Small" }
                Button {
                    size: ButtonSize::IconXs,
                    variant: ButtonVariant::Outline,
                    aria_label: "Submit",
                    IconArrowUpRight {}
                }
            }
            div { class: "flex items-start gap-2",
                Button { size: ButtonSize::Sm, variant: ButtonVariant::Outline, "Small" }
                Button {
                    size: ButtonSize::IconSm,
                    variant: ButtonVariant::Outline,
                    aria_label: "Submit",
                    IconArrowUpRight {}
                }
            }
            div { class: "flex items-start gap-2",
                Button { variant: ButtonVariant::Outline, "Default" }
                Button {
                    size: ButtonSize::Icon,
                    variant: ButtonVariant::Outline,
                    aria_label: "Submit",
                    IconArrowUpRight {}
                }
            }
            div { class: "flex items-start gap-2",
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Lg, "Large" }
                Button {
                    size: ButtonSize::IconLg,
                    variant: ButtonVariant::Outline,
                    aria_label: "Submit",
                    IconArrowUpRight {}
                }
            }
        }
    }
}
