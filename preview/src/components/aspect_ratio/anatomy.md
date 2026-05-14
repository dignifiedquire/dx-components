Import the component.

```rust
use dioxus::prelude::*;
use dioxus_primitives::aspect_ratio::AspectRatio;

#[component]
fn Demo() -> Element {
    rsx! {
        AspectRatio { ratio: 16.0 / 9.0 }
    }
}
```
