use crate::components::alert_dialog::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        AlertDialog {
            AlertDialogTrigger { "Show Alert Dialog" }
            AlertDialogOverlay {}
            AlertDialogContent {
                AlertDialogTitle { "Delete item" }
                AlertDialogDescription { "Are you sure you want to delete this item? This action cannot be undone." }
                AlertDialogFooter {
                    AlertDialogCancel { "Cancel" }
                    AlertDialogAction { "Delete" }
                }
            }
        }
    }
}
