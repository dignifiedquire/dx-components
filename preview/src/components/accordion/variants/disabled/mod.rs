use dioxus::prelude::*;
use dioxus_primitives::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};

#[component]
pub fn Demo() -> Element {
    rsx! {
        Accordion { disabled: true,
            AccordionItem { index: 0,
                AccordionTrigger { "Is it accessible?" }
                AccordionContent {
                    div { class: "pb-4",
                        p { class: "text-sm text-muted-foreground",
                            "Yes. It adheres to the WAI-ARIA design pattern."
                        }
                    }
                }
            }
            AccordionItem { index: 1,
                AccordionTrigger { "Is it styled?" }
                AccordionContent {
                    div { class: "pb-4",
                        p { class: "text-sm text-muted-foreground",
                            "Yes. It comes with default styles that match the other components' aesthetic."
                        }
                    }
                }
            }
        }
    }
}
