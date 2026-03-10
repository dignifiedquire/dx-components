use crate::components::button::component::{Button, ButtonVariant};
use crate::components::dropdown_menu::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut position = use_signal(|| Some("bottom".to_string()));

    rsx! {
        DropdownMenu {
            DropdownMenuTrigger {
                Button { variant: ButtonVariant::Outline, "Open" }
            }
            DropdownMenuContent {
                DropdownMenuLabel { "Panel Position" }
                DropdownMenuSeparator {}
                DropdownMenuRadioGroup {
                    value: position(),
                    on_value_change: move |v: String| position.set(Some(v)),
                    DropdownMenuRadioItem { value: "top", "Top" }
                    DropdownMenuRadioItem { value: "bottom", "Bottom" }
                    DropdownMenuRadioItem { value: "right", "Right" }
                }
            }
        }
    }
}
