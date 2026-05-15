A carousel slides through a series of items with prev/next navigation. Driven by `total_slides` + `slides_per_view`; the buttons disable automatically at the boundaries and `CarouselContent` translates one slide-width per step (`100 / slides_per_view` percent). CSS `transition-transform` provides the ~300 ms animation. Pass `on_api` to receive a [`CarouselApi`] snapshot whenever the state changes (mirrors shadcn's `setApi`).

`CarouselItem` defaults to `basis-full` (one item per view). To show multiple items at once, set `slides_per_view` on `Carousel` AND a matching `basis-1/N` on each `CarouselItem` (e.g. `slides_per_view: 3` + `basis-1/3`) — see the "Multiple"/"Sizes"/"Spacing" examples. `slides_per_view` is required for the boundary + translate math because, unlike embla, we don't measure the DOM. Tighten the gap with `-ml-1` on `CarouselContent` + `pl-1` on each `CarouselItem` (the "Spacing" example).

```rust
use dioxus::prelude::*;
use dioxus_components::card::{Card, CardContent};
use dioxus_components::carousel::*;

#[component]
fn ImageCarousel() -> Element {
    rsx! {
        Carousel { total_slides: 5, class: "w-full max-w-xs",
            CarouselContent {
                for i in 0..5 {
                    CarouselItem {
                        div { class: "p-1",
                            Card {
                                CardContent { class: "flex aspect-square items-center justify-center p-6",
                                    span { class: "text-4xl font-semibold", "{i + 1}" }
                                }
                            }
                        }
                    }
                }
            }
            CarouselPrevious {}
            CarouselNext {}
        }
    }
}
```

### Known gaps vs shadcn (embla-backed)

shadcn wraps `embla-carousel-react`. Our primitive is hand-written, so the embla-only features are not yet available:

- **Pointer drag / touch swipe** — embla provides physics-based dragging; our carousel only advances via the prev/next buttons or arrow keys.
- **Plugins** (auto-play, fade, etc.) — embla exposes a plugin API; we don't.
- **`opts` config** — embla's `loop`, `align`, `slidesToScroll`, `dragFree`, `containScroll`.
- **Auto DOM measurement** — embla measures slide widths from the DOM; we require an explicit `slides_per_view` instead.

These are tracked as a follow-up. Everything else now matches shadcn's source: boundary disable that accounts for `slides_per_view`, the `setApi`-equivalent `on_api` callback, `aria-roledescription`, `sr-only` button labels, ArrowLeft/Right with `preventDefault`, and the outline-icon button styling.
