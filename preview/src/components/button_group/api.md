### ButtonGroup

Connects related buttons/inputs into one unit. shadcn-only component (no Radix Primitives equivalent). Renders a `<div role="group">`.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `orientation` | `Orientation` | `Horizontal` | `Horizontal` strips inner left/right radii + borders; `Vertical` stacks and strips top/bottom. |
| `class` | `Option<String>` | — | Forwarded to the root `<div>`. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the root (extends `GlobalAttributes`). |
| `children` | `Element` | — | Buttons, inputs, `ButtonGroupText`, `ButtonGroupSeparator`, or nested `ButtonGroup`s. |

#### Data attributes

| Attribute | Values |
| --- | --- |
| `[data-slot]` | `"button-group"` |
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |

### ButtonGroupText

Non-interactive label cell that visually matches the buttons. Renders a `<div data-slot="button-group-text">`.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `class` | `Option<String>` | — | Forwarded to the `<div>`. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the `<div>` (extends `GlobalAttributes`). |
| `children` | `Element` | — | Label content (text / icons). |

### ButtonGroupSeparator

A divider between group items. Renders a decorative `<div role="none" data-slot="button-group-separator">`.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `orientation` | `Orientation` | `Vertical` | Divider orientation. |
| `class` | `Option<String>` | — | Forwarded to the `<div>`. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the `<div>` (extends `GlobalAttributes`). |
