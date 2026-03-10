Displays a menu to the user, triggered by a button.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_components::dropdown_menu::*;

rsx! {
    DropdownMenu {
        DropdownMenuTrigger {
            button { "Open" }
        }
        DropdownMenuContent {
            DropdownMenuLabel { "My Account" }
            DropdownMenuSeparator {}
            DropdownMenuItem {
                on_select: move |_| { /* handle click */ },
                "Profile"
            }
            DropdownMenuItem {
                on_select: move |_| { /* handle click */ },
                "Settings"
            }
            DropdownMenuSeparator {}
            DropdownMenuItem {
                disabled: true,
                "Disabled item"
            }
        }
    }
};
```

## Props

### DropdownMenuItem

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `disabled` | `bool` | `false` | Whether the item is disabled |
| `on_select` | `EventHandler<()>` | - | Called when the item is selected |

### DropdownMenuCheckboxItem

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `checked` | `ReadSignal<bool>` | - | Whether the item is checked |
| `on_checked_change` | `Callback<bool>` | - | Called when checked state changes |
| `disabled` | `bool` | `false` | Whether the item is disabled |

### DropdownMenuRadioItem

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | - | The value of this radio item |
| `disabled` | `bool` | `false` | Whether the item is disabled |
