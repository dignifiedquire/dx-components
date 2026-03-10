Displays rich content in a portal, triggered by a button.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_components::popover::*;

rsx! {
    Popover {
        PopoverTrigger {
            button { "Open popover" }
        }
        PopoverContent {
            side: ContentSide::Bottom,
            align: ContentAlign::Center,
            p { "The content of the popover." }
            PopoverClose {
                button { "Close" }
            }
        }
    }
};
```

## Props

### PopoverContent

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `side` | `ContentSide` | `Bottom` | Which side to display (`Top`, `Right`, `Bottom`, `Left`) |
| `align` | `ContentAlign` | `Center` | Alignment relative to trigger (`Start`, `Center`, `End`) |
| `force_mount` | `bool` | `false` | Force content to stay mounted |
