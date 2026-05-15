### Carousel

Root container. Sets up context (current index, total slides, orientation, `on_slide_change` callback) and renders a `<div data-slot="carousel">` with the prev/next buttons and content slot inside.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `total_slides` | `usize` | `1` | Total slide count. Must match the number of `CarouselItem`s rendered. |
| `slides_per_view` | `usize` | `1` | Slides visible per viewport. Set to match the `basis-1/N` on each `CarouselItem` (`2` for `basis-1/2`, `3` for `basis-1/3`, …). Drives the boundary detection and the per-step translate distance — required because, unlike embla, we don't measure the DOM. |
| `orientation` | `CarouselOrientation` | `Horizontal` | `Horizontal` or `Vertical`. Affects which axis the content translates on. |
| `initial_index` | `usize` | `0` | Starting slide index when uncontrolled. |
| `on_slide_change` | `Callback<usize>` | — | Fires whenever the active slide changes (via prev/next click or arrow keys). |
| `on_api` | `Callback<CarouselApi>` | — | Fires with a [`CarouselApi`] snapshot whenever state changes. Mirrors shadcn's `setApi`. |
| `class` | `Option<String>` | — | Forwarded to the root element. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the root element. |
| `children` | `Element` | — | `CarouselContent` + `CarouselPrevious` + `CarouselNext`. |

#### Data attributes

| Attribute | Values |
| --- | --- |
| `[data-slot]` | `"carousel"` |

(`data-orientation` is not emitted on the root — matching shadcn, orientation is read from context.)

### CarouselApi

```rust
pub struct CarouselApi {
    pub current_index: usize,
    pub total_slides: usize,
    pub can_scroll_prev: bool,
    pub can_scroll_next: bool,
}
```

Read-only state snapshot delivered to `on_api`. The subset of embla's `CarouselApi` that doesn't depend on DOM measurement.

### CarouselContent

Renders the moving track. The CSS transform that scrolls the slides lives here.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `class` | `Option<String>` | — | Forwarded to the element. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the element. |
| `children` | `Element` | — | One or more `CarouselItem`s. |

### CarouselItem

A single slide. Default basis is `basis-full` so one item is visible at a time. Override with Tailwind utilities (`basis-1/2`, `sm:basis-1/2`, `lg:basis-1/3`, …) to show multiple items per viewport.

### CarouselPrevious / CarouselNext

Buttons that move to the previous / next slide. Both automatically render as `disabled` when at the corresponding boundary (`can_scroll_prev` / `can_scroll_next`). Default content is a left/right chevron icon — pass children to override.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `class` | `Option<String>` | — | Forwarded to the `<button>` element. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the `<button>` element. |
| `children` | `Element` | (default chevron) | Custom button content. |

### CarouselOrientation

```rust
pub enum CarouselOrientation {
    Horizontal,  // default — content translates on X axis
    Vertical,    // content translates on Y axis
}
```

### CarouselCtx + use_carousel

```rust
pub struct CarouselCtx { /* … */ }
pub fn use_carousel() -> CarouselCtx;
```

Internal context (`use_carousel` retrieves it). Mostly useful when authoring custom carousel parts. The struct exposes `can_scroll_prev`, `can_scroll_next`, `total_slides`, and `orientation`.

### Known gaps vs shadcn / embla

See [Usage](#usage) above — pointer drag, plugins, api exposure, and the `opts` config object aren't yet implemented.
