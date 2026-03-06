use crate::components::accordion::component::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Accordion { class: "w-full",
            AccordionItem { value: "item-1", index: 0, default_open: true,
                AccordionTrigger { "Product Information" }
                AccordionContent { class: "flex flex-col gap-4 text-balance",
                    p {
                        "Our flagship product combines cutting-edge technology with sleek design. Built with premium materials, it offers unparalleled performance and reliability."
                    }
                    p {
                        "Key features include advanced processing capabilities, and an intuitive user interface designed for both beginners and experts."
                    }
                }
            }
            AccordionItem { value: "item-2", index: 1,
                AccordionTrigger { "Shipping Details" }
                AccordionContent { class: "flex flex-col gap-4 text-balance",
                    p {
                        "We offer worldwide shipping through trusted courier partners. Standard delivery takes 3-5 business days, while express shipping ensures delivery within 1-2 business days."
                    }
                    p {
                        "All orders are carefully packaged and fully insured. Track your shipment in real-time through our dedicated tracking portal."
                    }
                }
            }
            AccordionItem { value: "item-3", index: 2,
                AccordionTrigger { "Return Policy" }
                AccordionContent { class: "flex flex-col gap-4 text-balance",
                    p {
                        "We stand behind our products with a comprehensive 30-day return policy. If you're not completely satisfied, simply return the item in its original condition."
                    }
                    p {
                        "Our hassle-free return process includes free return shipping and full refunds processed within 48 hours of receiving the returned item."
                    }
                }
            }
        }
    }
}
