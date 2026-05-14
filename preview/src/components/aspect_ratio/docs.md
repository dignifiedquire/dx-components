`AspectRatio` keeps its child at a fixed width-to-height proportion regardless of the available width. Useful for embedded images, video thumbnails, and card previews where the layout should reserve a known shape before the media loads.

The component uses the classic `padding-bottom: 100% / ratio` technique so the box is computed during layout, not after image load — no content-shift when the image arrives.

```rust
use dioxus::prelude::*;
use dioxus_components::aspect_ratio::AspectRatio;

#[component]
fn HeroImage() -> Element {
    rsx! {
        div { class: "w-full max-w-sm",
            AspectRatio { ratio: 16.0 / 9.0, class: "rounded-lg bg-muted",
                img {
                    src: "/hero.jpg",
                    alt: "Hero photo",
                    class: "h-full w-full rounded-lg object-cover",
                }
            }
        }
    }
}
```

Tip: pass `h-full w-full object-cover` (or `style: "width: 100%; height: 100%; object-fit: cover"`) to your `<img>` child so it fills the ratio box without distortion.
