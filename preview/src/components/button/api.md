### Button

Renders a native `<button type="button">` with shadcn styling. shadcn-only component (no Radix Primitives equivalent).

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `variant` | `ButtonVariant` | `Default` | Colour treatment. See enum below. |
| `size` | `ButtonSize` | `Default` | Height + padding preset. See enum below. |
| `disabled` | `bool` | `false` | Disables the button — sets the `disabled` attribute and applies the `pointer-events-none opacity-50` Tailwind classes. |
| `class` | `Option<String>` | — | Forwarded to the `<button>` element. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the `<button>` element (extends `GlobalAttributes`). |
| `children` | `Element` | — | Content — typically text, optionally mixed with leading or trailing icons. |

#### Data attributes

| Attribute | Values |
| --- | --- |
| `[data-slot]` | `"button"` |

### ButtonVariant

```rust
pub enum ButtonVariant {
    Default,      // primary filled
    Destructive,  // destructive red
    Outline,      // border-only
    Secondary,    // muted surface
    Ghost,        // text-only with accent hover
    Link,         // text with underline-offset
}
```

### ButtonSize

```rust
pub enum ButtonSize {
    Default,  // h-9 px-4 py-2
    Sm,       // h-8 px-3
    Lg,       // h-10 px-6
    Icon,     // size-9 (square)
}
```
