use crate::components::accordion::component::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger, Orientation,
};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    // Horizontal accordions need different border + sizing than the
    // vertical-default shadcn styling. We override the styled
    // `AccordionItem`'s `border-b` with `border-r last:border-r-0` and
    // give each item equal width via `flex-1` so the row of triggers
    // sits side by side.
    rsx! {
        Accordion {
            class: "flex w-full rounded-md border",
            orientation: Orientation::Horizontal,
            default_value: vec!["item-1".to_string()],
            collapsible: true,
            AccordionItem {
                value: "item-1",
                class: "flex-1 border-r last:border-r-0 border-b-0 px-4",
                AccordionTrigger { "Product" }
                AccordionContent { class: "flex flex-col gap-4 text-balance pb-4",
                    p { "Our flagship product combines cutting-edge technology with sleek design." }
                }
            }
            AccordionItem {
                value: "item-2",
                class: "flex-1 border-r last:border-r-0 border-b-0 px-4",
                AccordionTrigger { "Shipping" }
                AccordionContent { class: "flex flex-col gap-4 text-balance pb-4",
                    p { "We offer worldwide shipping through trusted courier partners." }
                }
            }
            AccordionItem {
                value: "item-3",
                class: "flex-1 border-r last:border-r-0 border-b-0 px-4",
                AccordionTrigger { "Returns" }
                AccordionContent { class: "flex flex-col gap-4 text-balance pb-4",
                    p { "We stand behind our products with a comprehensive 30-day return policy." }
                }
            }
        }
    }
}
