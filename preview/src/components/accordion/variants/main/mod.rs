use dioxus::prelude::*;
use dioxus_primitives::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};

#[component]
pub fn Demo() -> Element {
    rsx! {
        Accordion { allow_multiple_open: false, horizontal: false,
            for i in 0..4 {
                AccordionItem { index: i,
                    AccordionTrigger { "the quick brown fox" }
                    AccordionContent {
                        div { class: "pb-4",
                            p { class: "p-0",
                                "lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum"
                            }
                        }
                    }
                }
            }
        }
    }
}
