Autocomplete input and command palette with filtering.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_components::combobox::*;

rsx! {
    Combobox { value: String::new(), on_value_change: |_| {},
        ComboboxInput { placeholder: "Search framework..." }
        ComboboxContent {
            ComboboxList {
                ComboboxEmpty { "No framework found." }
                ComboboxItem { value: "next", "Next.js" }
                ComboboxItem { value: "svelte", "SvelteKit" }
            }
        }
    }
};
```

## Props

### Combobox

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | `""` | Current selected value |
| `on_value_change` | `Callback<String>` | - | Called when value changes |
| `disabled` | `bool` | `false` | Whether the combobox is disabled |

### ComboboxItem

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | required | Unique item value |
| `disabled` | `bool` | `false` | Whether the item is disabled |
