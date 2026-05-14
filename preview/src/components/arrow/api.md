### Arrow

Renders a downward-pointing SVG triangle. Inside an overlay, position the arrow using a `PopperArrow` (which wraps `Arrow` with floating-ui-aware placement) rather than placing `Arrow` directly. Standalone use is also fine for static decorations.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `width` | `f64` | `10.0` | Arrow width in pixels. |
| `height` | `f64` | `5.0` | Arrow height in pixels. |
| `class` | `Option<String>` | — | Forwarded to the `<svg>` element. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the `<svg>` element (extends `GlobalAttributes`). |

#### Data attributes

| Attribute | Values |
| --- | --- |
| `[data-slot]` | `"arrow"` |

#### Output

The component emits:

```html
<svg data-slot="arrow"
     width="{width}" height="{height}"
     viewBox="0 0 30 10"
     preserveAspectRatio="none">
    <path d="M0,0 L30,0 L15,10 Z"></path>
</svg>
```
