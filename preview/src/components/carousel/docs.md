A carousel with motion and swipe built using Embla Carousel.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_components::carousel::*;

rsx! {
    Carousel { total_slides: 5,
        CarouselContent {
            for i in 0..5 {
                CarouselItem { "Slide {i + 1}" }
            }
        }
        CarouselPrevious {}
        CarouselNext {}
    }
};
```

## Props

### Carousel

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `total_slides` | `usize` | `0` | Total number of slides |
| `orientation` | `CarouselOrientation` | `Horizontal` | Layout direction |
| `initial_index` | `usize` | `0` | Starting slide index |
| `on_slide_change` | `Callback<usize>` | - | Called when active slide changes |
