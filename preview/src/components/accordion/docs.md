A vertically stacked set of interactive headings that each reveal a section of content.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_primitives::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};

rsx! {
    Accordion {
        AccordionItem { index: 0,
            AccordionTrigger { "Is it accessible?" }
            AccordionContent { "Yes. It adheres to the WAI-ARIA design pattern." }
        }
    }
};
```

## Props

### Accordion

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `allow_multiple_open` | `ReadSignal<bool>` | `false` | Allow multiple items open at once |
| `disabled` | `ReadSignal<bool>` | `false` | Disable the entire accordion |
| `collapsible` | `ReadSignal<bool>` | `true` | Allow all items to be collapsed |
| `horizontal` | `ReadSignal<bool>` | `false` | Use horizontal layout |

### AccordionItem

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `index` | `usize` | required | Position index within the accordion |
| `disabled` | `ReadSignal<bool>` | `false` | Disable this specific item |
| `default_open` | `bool` | `false` | Open this item by default |
| `on_change` | `Callback<bool>` | - | Called when open state changes |
