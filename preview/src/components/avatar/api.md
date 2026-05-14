### Avatar

Contains all the parts of an avatar. Also exported as `Root`. The styled layer wraps the root in a circular `flex` container sized via the `size` prop.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `size` | `AvatarSize` | `Default` (32 px) | One of `Sm` (24 px), `Default` (32 px), `Lg` (40 px). Sets `data-size` on the root for downstream sizing. *(styled layer only — the primitive has no `size`)* |
| `class` | `Option<String>` | — | Forwarded to the root element. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the root element (extends `GlobalAttributes`). |
| `children` | `Element` | — | Typically an `AvatarImage` followed by an `AvatarFallback`. |

#### Data attributes

| Attribute | Values |
| --- | --- |
| `[data-slot]` | `"avatar"` |
| `[data-size]` | `"sm"` \| `"default"` \| `"lg"` *(styled layer only)* |

### AvatarImage

The image to render. By default it only renders when it has loaded successfully — the `Avatar` swaps to the fallback while loading or on error. Use `on_loading_status_change` if you need to drive your own UI from the load state.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `src` | `String` | — | **Required.** The image source URL. |
| `alt` | `Option<String>` | — | Alt text for the image. |
| `on_loading_status_change` | `Option<Callback<ImageLoadingStatus>>` | — | Fires with the current load status — useful for telemetry or custom rendering. |
| `class` | `Option<String>` | — | Forwarded to the `<img>` element. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the `<img>` element. |

### AvatarFallback

Rendered when the image hasn't loaded — i.e. while loading, or after an error. Pass `delay_ms` to defer rendering so it only shows up for users on slower connections.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `delay_ms` | `Option<u64>` | `None` | Milliseconds to wait before rendering the fallback. Defers display so fast-loading images never show the fallback. |
| `class` | `Option<String>` | — | Forwarded to the fallback element. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the fallback element. |
| `children` | `Element` | — | Content to display when the image is unavailable — typically two-letter initials or a placeholder icon. |

### ImageLoadingStatus

```rust
pub enum ImageLoadingStatus {
    Idle,
    Loading,
    Loaded,
    Error,
}
```

### AvatarSize

```rust
pub enum AvatarSize {
    Sm,       // 24 px
    Default,  // 32 px (default)
    Lg,       // 40 px
}
```
