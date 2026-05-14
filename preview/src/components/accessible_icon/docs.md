Wrap any icon element (typically an `svg`) in `AccessibleIcon` and pass a `label` describing what the icon means in context. The visual icon is hidden from assistive technology with `aria-hidden`, and the label is rendered into a `VisuallyHidden` span so screen readers announce it.

```rust
use dioxus::prelude::*;
use dioxus_primitives::accessible_icon::AccessibleIcon;

#[component]
fn CloseButton() -> Element {
    rsx! {
        button {
            class: "icon-button",
            AccessibleIcon { label: "Close",
                svg {
                    view_box: "0 0 24 24",
                    width: "24",
                    height: "24",
                    fill: "none",
                    stroke: "currentColor",
                    path { d: "M18 6L6 18M6 6l12 12" }
                }
            }
        }
    }
}
```
