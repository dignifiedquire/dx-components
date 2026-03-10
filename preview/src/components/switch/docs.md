A control that allows the user to toggle between a checked and not checked state.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_components::switch::*;

let mut checked = use_signal(|| false);

rsx! {
    Switch {
        checked: checked(),
        on_checked_change: move |v| checked.set(v),
    }
};
```

The styled Switch composes the thumb internally — you only need to render the `Switch` component.

## Props

### Switch

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `checked` | `ReadSignal<Option<bool>>` | `None` | Controlled checked state |
| `default_checked` | `bool` | `false` | Default checked state |
| `disabled` | `bool` | `false` | Whether the switch is disabled |
| `on_checked_change` | `Callback<bool>` | - | Called when checked state changes |
| `size` | `SwitchSize` | `Default` | Size variant (`Default`, `Sm`) |
