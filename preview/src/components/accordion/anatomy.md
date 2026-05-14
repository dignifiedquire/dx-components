Import all parts and piece them together.

```rust
use dioxus::prelude::*;
use dioxus_primitives::accordion::{
    Accordion, AccordionContent, AccordionHeader, AccordionItem, AccordionTrigger,
};

#[component]
fn Demo() -> Element {
    rsx! {
        Accordion {
            AccordionItem { value: "item-1",
                AccordionHeader {
                    AccordionTrigger {}
                }
                AccordionContent {}
            }
        }
    }
}
```
