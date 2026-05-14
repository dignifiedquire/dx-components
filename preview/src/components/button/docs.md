A button surfaces a primary or secondary action. Use `variant` to pick a colour treatment (`Default`, `Secondary`, `Destructive`, `Outline`, `Ghost`, `Link`) and `size` for height/padding (`Default`, `Sm`, `Lg`, `Icon`). The component is keyboard-accessible and focusable by default — it renders a native `<button>` element.

```rust
use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonSize, ButtonVariant};

#[component]
fn SaveBar() -> Element {
    rsx! {
        div { class: "flex gap-2",
            Button { "Save" }
            Button { variant: ButtonVariant::Outline, "Cancel" }
            Button { variant: ButtonVariant::Destructive, "Delete" }
        }
    }
}
```

Mix in any icon (e.g. from `dx_icons_lucide`) as a child — the styled layer applies `[&_svg]:size-4` and `gap-2` so icons sit at a consistent scale alongside text. Use `ButtonSize::Icon` for square icon-only buttons.
