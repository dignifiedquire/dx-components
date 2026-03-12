Displays a list of options for the user to pick from, triggered by a button.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_components::select::*;

rsx! {
    Select {
        placeholder: "Select a fruit...",
        SelectTrigger {
            SelectValue {}
        }
        SelectContent {
            SelectGroup {
                SelectLabel { "Fruits" }
                SelectItem {
                    value: "apple",
                    text_value: "Apple",
                    "Apple"
                }
                SelectItem {
                    value: "banana",
                    text_value: "Banana",
                    "Banana"
                }
            }
            SelectSeparator {}
            SelectGroup {
                SelectLabel { "Vegetables" }
                SelectItem {
                    value: "carrot",
                    text_value: "Carrot",
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
| `on_value_change` | `Callback<String>` | - | Called when selected value changes |

### SelectItem

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `&str` | - | The value of the item |
| `text_value` | `&str` | value | Text for typeahead search |
| `disabled` | `bool` | `false` | Whether the item is disabled |
