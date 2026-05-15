use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_lucide::IconGitBranch;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm,
            IconGitBranch {}
            " New Branch"
        }
    }
}
