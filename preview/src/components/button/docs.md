## Usage

```rust
use dioxus_primitives::button::{Button, ButtonVariant, ButtonSize};

rsx! {
    Button { "Click me" }
}
```

## Props

| Prop | Type | Default |
|------|------|---------|
| variant | `ButtonVariant` | `Default` |
| size | `ButtonSize` | `Default` |
| disabled | `bool` | `false` |
| class | `&str` | `""` |

## Variants

- **Default** — Primary filled button
- **Secondary** — Muted button with secondary color
- **Destructive** — For dangerous actions like delete
- **Outline** — Bordered with transparent background
- **Ghost** — Borderless, visible on hover
- **Link** — Styled as a hyperlink

## Sizes

- **Default** — Standard size (`h-9`)
- **Sm** — Small (`h-8`)
- **Lg** — Large (`h-10`)
- **Icon** — Square icon-only (`size-9`)
