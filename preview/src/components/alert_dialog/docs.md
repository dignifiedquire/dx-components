A modal dialog that interrupts the user with important content and expects a response. Unlike the regular `Dialog`, the alert dialog cannot be dismissed by clicking the backdrop or pressing the X close button — the user **must** acknowledge the message via either `AlertDialogCancel` or `AlertDialogAction`. This matches the [WAI-ARIA `alertdialog` pattern](https://www.w3.org/WAI/ARIA/apg/patterns/alertdialog).

The content renders as a native `<dialog role="alertdialog">` opened with `showModal()`, so focus trap, inert siblings, and ESC handling come from the browser. See the [Accessibility](#accessibility) section for keyboard interactions.

```rust
use dioxus::prelude::*;
use dioxus_components::alert_dialog::*;
use dioxus_components::button::{Button, ButtonVariant};

#[component]
fn ConfirmDelete() -> Element {
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
```
