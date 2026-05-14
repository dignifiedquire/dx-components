An avatar displays a user's profile picture or, when the image is missing or still loading, a fallback (typically initials). The `Avatar` root coordinates between `AvatarImage` and `AvatarFallback` so only one shows at a time and you don't see the fallback flash before a fast-loading image arrives.

Pair `AvatarImage` with `AvatarFallback` containing two-letter initials. Use `delay_ms` on the fallback if you don't want it to appear at all for images that load quickly.

```rust
use dioxus::prelude::*;
use dioxus_components::avatar::{Avatar, AvatarFallback, AvatarImage};

#[component]
fn UserAvatar() -> Element {
    rsx! {
        Avatar {
            AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
            AvatarFallback { "CN" }
        }
    }
}
```

Avatars work well inside [Tooltip](/docs/components/tooltip) for "who's online" overlays and in stacked groups with negative margin (`-space-x-2`) for collaborator displays — see the "Stacked group" example below.
