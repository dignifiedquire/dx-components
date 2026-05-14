use crate::components::accordion::component::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger, Orientation,
};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Accordion {
            class: "flex w-full",
            orientation: Orientation::Horizontal,
            default_value: vec!["item-1".to_string()],
            collapsible: true,
            AccordionItem { value: "item-1",
                AccordionTrigger { "Product" }
                AccordionContent { class: "flex flex-col gap-4 text-balance",
                    p { "Our flagship product combines cutting-edge technology with sleek design." }
                }
            }
            AccordionItem { value: "item-2",
                AccordionTrigger { "Shipping" }
                AccordionContent { class: "flex flex-col gap-4 text-balance",
                    p { "We offer worldwide shipping through trusted courier partners." }
                }
            }
            AccordionItem { value: "item-3",
                AccordionTrigger { "Returns" }
                AccordionContent { class: "flex flex-col gap-4 text-balance",
                    p { "We stand behind our products with a comprehensive 30-day return policy." }
                }
            }
        }
    }
}
