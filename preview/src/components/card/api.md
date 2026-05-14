### Card

The root container for card content. Renders a `<div data-slot="card">`. shadcn-only component (no Radix Primitives equivalent).

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `size` | `CardSize` | `Default` | `Default` or `Sm`. The `Sm` variant tightens gaps and paddings and propagates the size to nested parts via `group-data-[size=sm]/card:` selectors. |
| `class` | `Option<String>` | — | Forwarded to the root element. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the root element. |
| `children` | `Element` | — | Sub-component composition — typically `CardHeader`, `CardContent`, `CardFooter`. |

#### Data attributes

| Attribute | Values |
| --- | --- |
| `[data-slot]` | `"card"` |
| `[data-size]` | `"default"` \| `"sm"` |

### CardHeader

Container for `CardTitle`, `CardDescription`, and an optional `CardAction`. Automatically switches to a two-column grid layout when a `CardAction` child is present (via `has-data-[slot=card-action]:grid-cols-[1fr_auto]`).

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `class` | `Option<String>` | — | Forwarded to the element. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the element. |
| `children` | `Element` | — | |

### CardTitle

Main heading. Renders as a `<div>` styled with `text-base leading-snug font-medium` (drops to `text-sm` when the parent card has `size: Sm`).

### CardDescription

Supporting text under the title. Styled `text-sm text-muted-foreground`.

### CardAction

Slot for header-level actions (e.g. a "Sign Up" link). Positioned in the top-right corner via grid placement (`col-start-2 row-span-2 row-start-1 self-start justify-self-end`).

### CardContent

The main body. Adds horizontal padding only (the card root supplies vertical padding).

### CardFooter

Footer slot. Renders with `flex items-center rounded-b-xl border-t bg-muted/50 p-4`. The card root sets `has-data-[slot=card-footer]:pb-0` so the footer sits flush against the bottom.

### CardSize

```rust
pub enum CardSize {
    Default,  // gap-4, py-4, px-4
    Sm,       // gap-3, py-3, px-3 — also drops title to text-sm
}
```
