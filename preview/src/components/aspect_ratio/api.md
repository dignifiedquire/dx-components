### AspectRatio

Contains the content you want to constrain to a given ratio. Renders an outer relatively-positioned wrapper that uses the classic `padding-bottom: 100% / ratio` technique to enforce the dimension, with an inner element holding your children stretched to fill via `position: absolute; inset: 0`. Also exported as `Root`.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `ratio` | `f64` | `1.0` | Desired width-to-height ratio. Pass values like `16.0 / 9.0` for widescreen, `4.0 / 3.0` for standard, `1.0` for square. |
| `class` | `Option<String>` | — | Forwarded to the inner content `<div>` (where consumers typically apply their styling, e.g. `rounded-lg bg-muted`). |
| `attributes` | `Vec<Attribute>` | — | Spread onto the inner content `<div>` (extends `GlobalAttributes`). |
| `children` | `Element` | — | Content to render inside the ratio box. Children fill the inner element absolutely — apply `width: 100%; height: 100%; object-fit: cover` (or the equivalent Tailwind classes `h-full w-full object-cover`) on an `<img>` child to avoid distortion. |

#### Data attributes

| Attribute | Where | Description |
| --- | --- | --- |
| `[data-radix-aspect-ratio-wrapper]` | outer `<div>` | Marker for the positioning wrapper. |
| `[data-slot="aspect-ratio"]` | inner content `<div>` | Marker for the content slot. |
