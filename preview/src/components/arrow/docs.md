`Arrow` renders a small SVG triangle suitable for pointing at the anchor of a floating element (popover, tooltip, dropdown). It's used internally by every overlay component in this library, but you can drop one into your own positioned content too.

The SVG has `viewBox="0 0 30 10"` and `preserveAspectRatio="none"`, so width and height are independent and the triangle scales to fill whatever box you give it. Style the fill, stroke, or transform via CSS.

```rust
use dioxus::prelude::*;
use dioxus_primitives::arrow::Arrow;

#[component]
fn TooltipTip() -> Element {
    rsx! {
        Arrow {
            width: 12.0,
            height: 6.0,
            style: "fill: var(--popover-bg);",
        }
    }
}
```

The default points downward (apex at the bottom). For other directions, apply a CSS `transform: rotate(...)` — Radix follows the same approach.

### Dioxus deviation

Upstream Radix supports `asChild` + custom SVG children to swap the path entirely. Our port renders the triangle path via `dangerous_inner_html` to work around a Dioxus rendering bug where conditional SVG child elements emit `<!--placeholder-->` instead of real DOM nodes. Custom children are not currently supported — style with CSS instead.
