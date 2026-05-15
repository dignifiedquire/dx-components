use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_lucide::IconPlus;

#[component]
pub fn Demo() -> Element {
    rsx! {
        ButtonGroup {
            Button { variant: ButtonVariant::Secondary, "Button" }
            ButtonGroupSeparator {}
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Icon,
                IconPlus {}
            }
        }
    }
}
