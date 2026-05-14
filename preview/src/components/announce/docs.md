`Announce` is a thin wrapper that mirrors its children into a global ARIA live region on `document.body`, so screen readers reliably announce content changes that aren't otherwise visible to assistive technology. It's a line-by-line port of `@radix-ui/react-announce`.

Use it when you need to surface dynamic status text (e.g. "Saved", "Network error") without rendering anything visually intrusive. The component renders its children inline at its location in the tree, and a parallel copy lives inside a `<div role="status" aria-live="polite|assertive">` at the document root — only the live region copy is announced.

```rust
use dioxus::prelude::*;
use dioxus_primitives::announce::{Announce, AnnounceType};

#[component]
fn SaveStatus(saved: ReadSignal<bool>) -> Element {
    rsx! {
        // Visible UI
        if saved() {
            span { "Saved!" }
        }

        // Live-region announcement — invisible, announced by screen readers.
        Announce {
            r#type: if saved() { AnnounceType::Polite } else { AnnounceType::Off },
            if saved() { "Document saved" }
        }
    }
}
```

`Announce` creates at most two unique live regions per page by default (one polite, one assertive). Pass `region_identifier` to scope an announcement to its own dedicated region — useful when nested announcers would otherwise interfere with one another.
