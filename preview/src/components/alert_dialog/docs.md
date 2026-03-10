A modal dialog that interrupts the user with important content and expects a response.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_components::alert_dialog::*;

rsx! {
    AlertDialog {
        AlertDialogTrigger {
            button { "Show Dialog" }
        }
        AlertDialogOverlay {}
        AlertDialogContent {
            AlertDialogHeader {
                AlertDialogTitle { "Are you absolutely sure?" }
                AlertDialogDescription {
                    "This action cannot be undone."
                }
            }
            AlertDialogFooter {
                AlertDialogCancel { "Cancel" }
                AlertDialogAction { "Continue" }
            }
        }
    }
};
```

## Props

### AlertDialog

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `Option<Signal<bool>>` | `None` | Controlled open state |
| `default_open` | `bool` | `false` | Default open state |
| `on_open_change` | `Callback<bool>` | - | Called when open state changes |

### AlertDialogContent

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `force_mount` | `bool` | `false` | Force content to stay mounted |
| `show_close` | `bool` | `false` | Show an X close button |
