An accordion is a vertically stacked set of interactive headings that each reveal a section of content. By default only one item can be open at a time (`AccordionType::Single`); pass `AccordionType::Multiple` to allow any combination of items to be open simultaneously.

Trigger keyboard interaction follows the [WAI-ARIA accordion pattern](https://www.w3.org/WAI/ARIA/apg/patterns/accordion) — see the [Accessibility](#accessibility) section.

```rust
use dioxus::prelude::*;
use dioxus_primitives::accordion::{
    Accordion, AccordionContent, AccordionHeader, AccordionItem, AccordionTrigger,
};

#[component]
fn FAQ() -> Element {
    rsx! {
        Accordion { collapsible: true, default_value: vec!["q-1".to_string()],
            AccordionItem { value: "q-1",
                AccordionHeader {
                    AccordionTrigger { "Is it accessible?" }
                }
                AccordionContent { "Yes. It follows the WAI-ARIA accordion design pattern." }
            }
            AccordionItem { value: "q-2",
                AccordionHeader {
                    AccordionTrigger { "Is it animated?" }
                }
                AccordionContent {
                    "Yes — content height is exposed via the "
                    code { "--radix-accordion-content-height" }
                    " CSS custom property so you can animate "
                    code { "height" }
                    " on open/close."
                }
            }
        }
    }
}
```
