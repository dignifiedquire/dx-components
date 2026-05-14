### Root

Contains the icon to make accessible. Also re-exported as `AccessibleIcon`.

| Prop       | Type      | Default | Description                                                                                                                                              |
| ---------- | --------- | ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `label`    | `String`  | —       | **Required.** The accessible label for the icon. Visually hidden but announced to screen readers, similar to `alt` text for `<img>` tags.                |
| `children` | `Element` | —       | The icon element (typically an `svg`). It is wrapped in a `<span aria-hidden="true">` so sighted users see it but assistive technology skips it.        |
