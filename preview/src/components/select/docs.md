Displays a list of options for the user to pick from, triggered by a button.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_components::select::*;

rsx! {
    Select::<Option<String>> {
        placeholder: "Select a fruit...",
        SelectTrigger {
            SelectValue {}
        }
        SelectContent {
            SelectGroup {
                SelectLabel { "Fruits" }
                SelectItem::<Option<String>> {
                    index: 0,
                    value: "apple".to_string(),
                    "Apple"
                }
                SelectItem::<Option<String>> {
                    index: 1,
                    value: "banana".to_string(),
                    "Banana"
                }
            }
            SelectSeparator {}
            SelectGroup {
                SelectLabel { "Vegetables" }
                SelectItem::<Option<String>> {
                    index: 2,
                    value: "carrot".to_string(),
                    "Carrot"
                }
            }
        }
    }
};
```

## Props

### Select

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `placeholder` | `&str` | `""` | Placeholder text when no value selected |
| `on_value_change` | `Callback<T>` | - | Called when selected value changes |

### SelectItem

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `ReadSignal<T>` | - | The value of the item |
| `index` | `ReadSignal<usize>` | - | Item index for keyboard navigation |
| `disabled` | `bool` | `false` | Whether the item is disabled |
