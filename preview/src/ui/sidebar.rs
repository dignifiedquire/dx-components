use dioxus::prelude::*;

use crate::components;
use crate::Route;

/// Left navigation sidebar matching shadcn's docs-sidebar.
#[component]
pub(crate) fn Sidebar() -> Element {
    let route: Route = router().current();
    let current_name = route.component_name();

    let mut sorted: Vec<_> = components::COMPONENT_LIST.iter().collect();
    sorted.sort_by_key(|d| d.name);

    rsx! {
        aside {
            class: "sticky top-14 z-30 hidden h-[calc(100svh-3.5rem)] shrink-0 bg-transparent lg:flex",
            div { class: "absolute top-12 right-2 bottom-0 hidden h-full w-px bg-gradient-to-b from-transparent via-border to-transparent lg:flex" }
            nav {
                class: "w-56 overflow-x-hidden overflow-y-auto px-2 py-6",
                style: "scrollbar-width: none;",
                h4 { class: "mb-1 rounded-md px-2 py-1 text-sm font-medium text-muted-foreground",
                    "Components"
                }
                div { class: "flex flex-col gap-0.5",
                    for meta in sorted {
                        {
                            let is_active = current_name == Some(meta.name);
                            let display_name = meta.name.replace("_", " ");
                            rsx! {
                                Link {
                                    to: Route::component(meta.name),
                                    class: if is_active {
                                        "block capitalize rounded-md px-2 py-1 text-[0.8rem] font-medium text-foreground bg-accent no-underline"
                                    } else {
                                        "block capitalize rounded-md px-2 py-1 text-[0.8rem] font-medium text-muted-foreground hover:text-foreground hover:bg-accent no-underline transition-colors"
                                    },
                                    "{display_name}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
