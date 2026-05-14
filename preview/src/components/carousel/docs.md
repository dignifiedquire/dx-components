A carousel slides through a series of items with prev/next navigation. Driven by a `total_slides` counter and an internal `current_index` signal; the buttons are automatically disabled at the boundaries. CSS `transition-transform` on `CarouselContent` provides the slide animation (~300 ms).

`CarouselItem` defaults to `basis-full` (one item visible at a time). Pass Tailwind classes like `basis-1/2` or `lg:basis-1/3` to show multiple items at once — see the "Multiple" example. Use `class: "-ml-1"` on `CarouselContent` plus `pl-1` on each `CarouselItem` to tighten spacing — see "Spacing".

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

Our primitive is hand-written, not an embla wrapper, so some features are not yet available:

- **Pointer drag / touch swipe** — embla provides physics-based dragging; our carousel only advances via the prev/next buttons.
- **Plugins** (auto-play, fade, etc.) — embla exposes a plugin API; we don't.
- **API exposure** — embla returns an api ref the consumer can scroll programmatically; we don't.
- **Loop, alignment, slidesToScroll** — embla `opts` aren't implemented.

These are tracked as follow-ups; the basic carousel (slides + prev/next + boundary disable + transition) is functional today.
