A `ButtonGroup` visually connects a set of related buttons (and inputs) into a single unit. It strips the inner border radii / borders so the children read as one control, and switches to a stacked layout with `orientation`.

```rust
use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::button_group::{ButtonGroup, ButtonGroupSeparator};

#[component]
fn Toolbar() -> Element {
    rsx! {
        ButtonGroup {
            Button { variant: ButtonVariant::Secondary, "Copy" }
            ButtonGroupSeparator {}
            Button { variant: ButtonVariant::Secondary, "Paste" }
        }
    }
}
```

Compose it with `ButtonGroupText` for non-interactive labels and `ButtonGroupSeparator` for a divider. Nest `ButtonGroup`s to add a gap between sub-groups. shadcn-only component (no Radix Primitives equivalent).
