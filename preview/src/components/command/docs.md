Fast, composable, unstyled command menu.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_components::command::*;

rsx! {
    Command {
        CommandInput { placeholder: "Type a command or search..." }
        CommandList {
            CommandEmpty { "No results found." }
            CommandGroup { heading: "Suggestions",
                CommandItem { value: "calendar", "Calendar" }
                CommandItem { value: "search", "Search" }
            }
        }
    }
};
```

## Props

### Command

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `on_select` | `Callback<String>` | - | Called when an item is selected |

### CommandItem

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | required | Unique item identifier |
| `disabled` | `bool` | `false` | Whether the item is disabled |
