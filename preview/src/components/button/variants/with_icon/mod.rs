use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_lucide::IconMail;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Button {
            IconMail { size: 16 }
            "Login with Email"
        }
    }
}
