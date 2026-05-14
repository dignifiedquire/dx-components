`Badge` displays a small, pill-shaped label — useful for status indicators, tag-style counts, and categorisation. Pass a `variant` to pick a colour treatment from `Default`, `Secondary`, `Destructive`, `Outline`, `Ghost`, or `Link`.

```rust
use dioxus::prelude::*;
use dioxus_components::badge::{Badge, BadgeVariant};

#[component]
fn StatusRow() -> Element {
    rsx! {
        div { class: "flex gap-2",
            Badge { "New" }
            Badge { variant: BadgeVariant::Secondary, "Beta" }
            Badge { variant: BadgeVariant::Destructive, "Deprecated" }
        }
    }
}
```

Badges accept any children, so you can mix in icons (e.g. lucide check-circle, bookmark, spinners) or text emphasis like counts. The styled layer applies `[&>svg]:size-3` so icons drop in at the right scale.
