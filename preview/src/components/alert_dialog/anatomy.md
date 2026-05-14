Import all parts and piece them together.

```rust
use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::{
    AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogOverlay, AlertDialogRoot, AlertDialogTitle, AlertDialogTrigger,
};

#[component]
fn Demo() -> Element {
    rsx! {
        AlertDialogRoot {
            AlertDialogTrigger {}
            AlertDialogOverlay {}
            AlertDialogContent {
                AlertDialogTitle {}
                AlertDialogDescription {}
                AlertDialogCancel {}
                AlertDialogAction {}
            }
        }
    }
}
```
