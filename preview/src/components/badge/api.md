### Badge

A small pill-shaped label. Renders as a `<span>` by default. shadcn-only component (no Radix Primitives equivalent).

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `variant` | `BadgeVariant` | `Default` | Colour treatment. See enum below. |
| `class` | `Option<String>` | — | Forwarded to the root element. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the root element (extends `GlobalAttributes`). |
| `children` | `Element` | — | Content to render inside the badge — typically text, optionally mixed with icons. |

#### Data attributes

| Attribute | Values |
| --- | --- |
| `[data-slot]` | `"badge"` |
| `[data-variant]` | `"default"` \| `"secondary"` \| `"destructive"` \| `"outline"` \| `"ghost"` \| `"link"` |

### BadgeVariant

```rust
pub enum BadgeVariant {
    Default,      // primary colour fill (default)
    Secondary,    // secondary surface
    Destructive,  // destructive red
    Outline,      // border-only, no fill
    Ghost,        // text-only, accent hover
    Link,         // underlined text, no chrome
}
```
