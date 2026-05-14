use crate::components::alert_dialog::component::*;
use crate::components::button::component::{Button, ButtonVariant};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        AlertDialog {
            AlertDialogTrigger {
                Button { variant: ButtonVariant::Outline, "Show Dialog" }
            }
            AlertDialogOverlay {}
            AlertDialogContent {
                AlertDialogHeader {
                    AlertDialogTitle { "Are you absolutely sure?" }
                    AlertDialogDescription {
                        "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                    }
                }
                AlertDialogFooter {
                    AlertDialogCancel { "Cancel" }
                    AlertDialogAction { "Continue" }
                }
            }
        }
    }
}
