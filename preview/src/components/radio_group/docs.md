A set of checkable buttons where only one can be checked at a time.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_components::radio_group::*;

rsx! {
    RadioGroup {
        default_value: "comfortable".to_string(),
        on_value_change: move |value: String| {
            // Handle value change
        },
        RadioGroupItem { value: "default".to_string() }
        RadioGroupItem { value: "comfortable".to_string() }
        RadioGroupItem { value: "compact".to_string() }
    }
};
```

## Props

### RadioGroup

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `ReadSignal<Option<String>>` | `None` | Controlled selected value |
| `default_value` | `String` | `""` | Default selected value |
| `on_value_change` | `Callback<String>` | - | Called when selection changes |
| `disabled` | `bool` | `false` | Whether the group is disabled |
| `required` | `bool` | `false` | Whether a selection is required |
| `orientation` | `Orientation` | `Vertical` | Layout direction |

### RadioGroupItem

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | - | The value of this radio item |
| `disabled` | `bool` | `false` | Whether this item is disabled |
