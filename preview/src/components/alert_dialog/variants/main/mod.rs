use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        AlertDialogRoot {
            AlertDialogTrigger {
                class: "inline-flex items-center justify-center gap-2 rounded-md border border-input bg-background px-4 py-2 text-sm font-medium shadow-xs hover:bg-accent hover:text-accent-foreground",
                "Show Alert Dialog"
            }
            AlertDialogOverlay {
                class: "fixed inset-0 z-50 bg-black/50 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0",
            }
            AlertDialogContent {
                class: "fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border bg-background p-6 shadow-lg sm:max-w-lg",
                AlertDialogTitle { class: "text-lg font-semibold leading-none", "Delete item" }
                AlertDialogDescription { class: "text-sm text-muted-foreground", "Are you sure you want to delete this item? This action cannot be undone." }
                AlertDialogFooter {
                    class: "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end",
                    AlertDialogCancel {
                        class: "inline-flex items-center justify-center gap-2 rounded-md border border-input bg-background px-4 py-2 text-sm font-medium shadow-xs hover:bg-accent hover:text-accent-foreground",
                        "Cancel"
                    }
                    AlertDialogAction {
                        class: "inline-flex items-center justify-center gap-2 rounded-md bg-destructive px-4 py-2 text-sm font-medium text-destructive-foreground shadow-xs hover:bg-destructive/90",
                        "Delete"
                    }
                }
            }
        }
    }
}
