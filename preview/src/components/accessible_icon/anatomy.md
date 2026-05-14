Import the component.

```rust
use dioxus::prelude::*;
use dioxus_primitives::accessible_icon::AccessibleIcon;

#[component]
fn Demo() -> Element {
    rsx! {
        AccessibleIcon { label: "...",
            // your icon here
        }
    }
}
```
