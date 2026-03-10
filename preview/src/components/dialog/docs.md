A modal dialog that interrupts the user with important content and expects a response.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_components::dialog::*;

rsx! {
    Dialog {
        DialogTrigger {
            button { "Open" }
        }
        DialogOverlay {}
        DialogContent {
            DialogHeader {
                DialogTitle { "Edit profile" }
                DialogDescription {
                    "Make changes to your profile here."
                }
            }
            // Form content goes here
            DialogFooter {
                DialogClose {
                    button { "Save changes" }
                }
            }
        }
    }
};
```

## Props

### Dialog

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `Option<Signal<bool>>` | `None` | Controlled open state |
| `default_open` | `bool` | `false` | Default open state |
| `on_open_change` | `Callback<bool>` | - | Called when open state changes |

### DialogContent

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `force_mount` | `bool` | `false` | Force content to stay mounted |
| `show_close` | `bool` | `true` | Show the X close button |
