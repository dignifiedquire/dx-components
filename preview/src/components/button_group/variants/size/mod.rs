use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_lucide::IconPlus;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex flex-col items-start gap-8",
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Small" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Button" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Group" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::IconSm,
                    IconPlus {}
                }
            }
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, "Default" }
                Button { variant: ButtonVariant::Outline, "Button" }
                Button { variant: ButtonVariant::Outline, "Group" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                    IconPlus {}
                }
            }
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Lg, "Large" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Lg, "Button" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Lg, "Group" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::IconLg,
                    IconPlus {}
                }
            }
        }
    }
}
