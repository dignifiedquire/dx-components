use crate::components::button::component::{Button, ButtonVariant};
use crate::components::dropdown_menu::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut show_status_bar = use_signal(|| true);
    let mut show_activity_bar = use_signal(|| false);
    let mut show_panel = use_signal(|| false);

    rsx! {
        DropdownMenu {
            DropdownMenuTrigger {
                Button { variant: ButtonVariant::Outline, "Open" }
            }
            DropdownMenuContent {
                DropdownMenuLabel { "Appearance" }
                DropdownMenuSeparator {}
                DropdownMenuCheckboxItem {
                    checked: show_status_bar(),
                    on_checked_change: move |v| show_status_bar.set(v),
                    "Status Bar"
                }
                DropdownMenuCheckboxItem {
                    checked: show_activity_bar(),
                    on_checked_change: move |v| show_activity_bar.set(v),
                    "Activity Bar"
                }
                DropdownMenuCheckboxItem {
                    checked: show_panel(),
                    on_checked_change: move |v| show_panel.set(v),
                    "Panel"
                }
            }
        }
    }
}
