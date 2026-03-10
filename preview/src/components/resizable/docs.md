Accessible resizable panel groups and layouts.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_components::resizable::*;

rsx! {
    ResizablePanelGroup {
        ResizablePanel { default_size: 50.0, "One" }
        ResizableHandle {}
        ResizablePanel { default_size: 50.0, "Two" }
    }
};
```

## Props

### ResizablePanelGroup

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `orientation` | `Orientation` | `Horizontal` | Layout direction |
| `on_layout_change` | `Option<Callback<Vec<f64>>>` | `None` | Called when layout changes |

### ResizablePanel

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `default_size` | `f64` | `50.0` | Default size as percentage (0-100) |
| `min_size` | `f64` | `0.0` | Minimum size percentage |
| `max_size` | `f64` | `100.0` | Maximum size percentage |

### ResizableHandle

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `disabled` | `bool` | `false` | Whether the handle is disabled |
| `with_handle` | `bool` | `false` | Show a visible grip icon |
