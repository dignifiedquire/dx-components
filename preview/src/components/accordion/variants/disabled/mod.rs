use crate::components::accordion::component::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Accordion { class: "w-full", disabled: true,
            AccordionItem { value: "item-1", index: 0,
                AccordionTrigger { "Product Information" }
                AccordionContent { class: "flex flex-col gap-4 text-balance",
                    p {
                        "Our flagship product combines cutting-edge technology with sleek design. Built with premium materials, it offers unparalleled performance and reliability."
                    }
                }
            }
            AccordionItem { value: "item-2", index: 1,
                AccordionTrigger { "Shipping Details" }
                AccordionContent { class: "flex flex-col gap-4 text-balance",
                    p {
                        "We offer worldwide shipping through trusted courier partners. Standard delivery takes 3-5 business days, while express shipping ensures delivery within 1-2 business days."
                    }
                }
            }
        }
    }
}
