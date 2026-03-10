A drawer component for Dioxus.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_components::drawer::*;

rsx! {
    Drawer {
        DrawerTrigger { "Open" }
        DrawerOverlay {}
        DrawerContent {
            DrawerHeader {
                DrawerTitle { "Are you absolutely sure?" }
                DrawerDescription { "This action cannot be undone." }
            }
            DrawerFooter {
                DrawerClose { "Cancel" }
            }
        }
    }
};
```

## Props

### Drawer

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `ReadSignal<Option<bool>>` | `None` | Controlled open state |
| `default_open` | `bool` | `false` | Whether to start open |
| `on_open_change` | `Callback<bool>` | - | Called when open state changes |

### DrawerContent

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `direction` | `DrawerDirection` | `Bottom` | Slide direction (Top, Right, Bottom, Left) |
| `force_mount` | `bool` | `false` | Force mount even when closed |
