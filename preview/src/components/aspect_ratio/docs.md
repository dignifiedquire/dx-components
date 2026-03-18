```rust
use dioxus_components::aspect_ratio::AspectRatio;
```

```rust
rsx! {
    AspectRatio { ratio: 16.0 / 9.0,
        img { src: "...", alt: "Image", class: "rounded-md object-cover" }
    }
}
```
