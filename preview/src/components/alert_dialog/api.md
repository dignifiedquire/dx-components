### AlertDialogRoot

Contains all the parts of an alert dialog. Always modal — there is no `modal` prop (matches Radix, which forces `modal: true`). Also exported as `AlertDialog` from the styled layer.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `open` | `ReadSignal<Option<bool>>` | `None` | Controlled open state. Pair with `on_open_change`. When `None`, the dialog is uncontrolled. |
| `default_open` | `bool` | `false` | Initial open state when uncontrolled. |
| `on_open_change` | `Callback<bool>` | — | Fires whenever the open state changes (user interaction, ESC, programmatic close, etc.). |

### AlertDialogTrigger

A button that opens the dialog. Renders as a native `<button type="button">` with `aria-haspopup="dialog"` and `aria-expanded` reflecting the current state.

#### Data attributes

| Attribute | Values |
| --- | --- |
| `[data-state]` | `"open"` \| `"closed"` |

### AlertDialogOverlay

A layer that covers the inert portion of the view when the dialog is open. **Unlike `DialogOverlay`, clicking the overlay does NOT close the alert dialog** — the user must use an explicit `AlertDialogCancel` or `AlertDialogAction` button. This matches Radix's `onPointerDownOutside.preventDefault()` behaviour.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `id` | `ReadSignal<Option<String>>` | auto-generated | DOM `id` for the overlay. |
| `force_mount` | `bool` | `false` | Force the overlay to mount even when closed. Retained for API parity — `Presence` handles unmount on close by default. |

#### Data attributes

| Attribute | Values |
| --- | --- |
| `[data-state]` | `"open"` \| `"closed"` |

### AlertDialogContent

Contains content to be rendered when the dialog is open. Renders as a native `<dialog>` element with `role="alertdialog"` and `aria-modal="true"`, opened via `showModal()`. Native ESC closes the dialog (matches Radix's `onEscapeKeyDown` default). **Backdrop click does NOT close** — explicit `AlertDialogCancel` or `AlertDialogAction` required.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `id` | `ReadSignal<Option<String>>` | auto-generated | DOM `id` for the content. |
| `force_mount` | `bool` | `false` | Retained for API parity — the native `<dialog>` is always in the DOM and toggled via the `open` attribute. |

#### Data attributes

| Attribute | Values |
| --- | --- |
| `[data-state]` | `"open"` \| `"closed"` |

### AlertDialogTitle

An accessible name announced when the dialog opens. Wires `aria-labelledby` on the content automatically. Renders as `<h2>`.

### AlertDialogDescription

An accessible description announced when the dialog opens. Wires `aria-describedby` on the content automatically. Renders as `<p>`.

### AlertDialogCancel

A button that closes the dialog. Should be visually distinguished from `AlertDialogAction`. Renders as a native `<button type="button">` that calls the close path on click.

### AlertDialogAction

A button that closes the dialog. Should be visually distinguished from `AlertDialogCancel` (e.g. destructive styling). Renders as a native `<button type="button">` that calls the close path on click.

### AlertDialogFooter

Layout helper that arranges `AlertDialogCancel` and `AlertDialogAction` side by side (and stacks them on small screens in the styled layer). Not part of upstream Radix — added in the styled layer to mirror shadcn's API.
