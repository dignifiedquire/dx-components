### Accordion

Contains all the parts of an accordion. Also exported as `Root`.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `type` | `AccordionType` | `Single` | `Single` allows one item open at a time; `Multiple` allows any number open simultaneously. |
| `value` | `ReadSignal<Option<Vec<String>>>` | `None` | Controlled value(s) of the expanded item(s). For `Single`, the `Vec` contains zero or one entry. Pair with `on_value_change` for controlled mode. |
| `default_value` | `Vec<String>` | `vec![]` | Initial expanded item(s) when uncontrolled. |
| `on_value_change` | `Callback<Vec<String>>` | — | Fires whenever the expanded set changes. |
| `collapsible` | `ReadSignal<bool>` | `false` | When `type` is `Single`, allows clicking the open trigger to close it (otherwise it stays open). Always effectively `true` for `Multiple`. |
| `disabled` | `ReadSignal<bool>` | `false` | Prevents the user from interacting with the accordion and all its items. |
| `dir` | `ReadSignal<Direction>` | `Ltr` | Reading direction. Affects `ArrowLeft` / `ArrowRight` keyboard navigation when horizontal. |
| `orientation` | `ReadSignal<Orientation>` | `Vertical` | Layout orientation of the accordion. |

#### Data attributes

| Attribute | Values |
| --- | --- |
| `[data-orientation]` | `"vertical"` \| `"horizontal"` |
| `[data-disabled]` | Present when `disabled` is `true` |

### AccordionItem

Contains all the parts of a collapsible section.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `value` | `String` | — | **Required.** A unique value identifying the item. |
| `disabled` | `ReadSignal<bool>` | `false` | Prevents the user from interacting with this specific item. |

#### Data attributes

| Attribute | Values |
| --- | --- |
| `[data-state]` | `"open"` \| `"closed"` |
| `[data-disabled]` | Present when the item is disabled |
| `[data-orientation]` | `"vertical"` \| `"horizontal"` |

### AccordionHeader

Wraps an `AccordionTrigger`. Renders as a heading element — defaults to `h3` for semantic structure.

#### Data attributes

| Attribute | Values |
| --- | --- |
| `[data-state]` | `"open"` \| `"closed"` |
| `[data-disabled]` | Present when the item is disabled |
| `[data-orientation]` | `"vertical"` \| `"horizontal"` |

### AccordionTrigger

Toggles the collapsed state of its associated item. Should be nested inside an `AccordionHeader`.

#### Data attributes

| Attribute | Values |
| --- | --- |
| `[data-state]` | `"open"` \| `"closed"` |
| `[data-disabled]` | Present when the item is disabled |
| `[data-orientation]` | `"vertical"` \| `"horizontal"` |

### AccordionContent

Contains the collapsible content for an item.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `force_mount` | `bool` | `false` | Force the content to mount even when closed. Useful when driving open/close animations from an external animation library that needs the DOM present. |

#### Data attributes

| Attribute | Values |
| --- | --- |
| `[data-state]` | `"open"` \| `"closed"` |
| `[data-disabled]` | Present when the item is disabled |
| `[data-orientation]` | `"vertical"` \| `"horizontal"` |

#### CSS custom properties

The content element exposes the same `--radix-*` CSS variables Radix does so existing CSS animation patterns work without changes.

| Property | Description |
| --- | --- |
| `--radix-accordion-content-width` | The width of the content when it opens/closes. |
| `--radix-accordion-content-height` | The height of the content when it opens/closes. |
