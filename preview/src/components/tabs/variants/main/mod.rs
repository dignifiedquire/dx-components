use dioxus::prelude::*;
use dioxus_primitives::tabs::{TabContent, TabList, TabTrigger, Tabs};

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tabs {
            default_value: "tab1".to_string(),
            horizontal: true,
            max_width: "16rem",
            TabList {
                TabTrigger { value: "tab1".to_string(), index: 0usize, "Tab 1" }
                TabTrigger { value: "tab2".to_string(), index: 1usize, "Tab 2" }
                TabTrigger { value: "tab3".to_string(), index: 2usize, "Tab 3" }
            }
            TabContent { index: 0usize, value: "tab1".to_string(),
                div { class: "flex h-20 w-full items-center justify-center",
                    "Tab 1 Content"
                }
            }
            TabContent { index: 1usize, value: "tab2".to_string(),
                div { class: "flex h-20 w-full items-center justify-center",
                    "Tab 2 Content"
                }
            }
            TabContent { index: 2usize, value: "tab3".to_string(),
                div { class: "flex h-20 w-full items-center justify-center",
                    "Tab 3 Content"
                }
            }
        }
    }
}
