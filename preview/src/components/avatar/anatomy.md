Import all parts and piece them together.

```rust
use dioxus::prelude::*;
use dioxus_primitives::avatar::{Avatar, AvatarFallback, AvatarImage};

#[component]
fn Demo() -> Element {
    rsx! {
        Avatar {
            AvatarImage {}
            AvatarFallback {}
        }
    }
}
```
