use dioxus::prelude::*;

use crate::components;
use crate::Route;

/// Right "On This Page" table of contents matching shadcn's docs-toc.
#[component]
pub(crate) fn TableOfContents() -> Element {
    let route: Route = router().current();
    let comp_name = route.component_name();
    let meta =
        comp_name.and_then(|name| components::COMPONENT_LIST.iter().find(|d| d.name == name));

    let Some(meta) = meta else {
        return rsx! {};
    };

    let extra_variants = if meta.variants.len() > 1 {
        &meta.variants[1..]
    } else {
        &[]
    };

    rsx! {
        aside {
            class: "sticky top-14 z-30 ml-auto hidden h-[calc(100svh-3.5rem)] w-56 shrink-0 flex-col gap-4 pb-8 xl:flex",
            div {
                class: "flex flex-col gap-2 p-4 pt-6 text-sm overflow-y-auto",
                style: "scrollbar-width: none;",
                p { class: "sticky top-0 h-6 bg-background text-xs font-medium text-muted-foreground",
                    "On This Page"
                }
                a {
                    class: "text-[0.8rem] text-muted-foreground no-underline transition-colors hover:text-foreground",
                    href: "#preview",
                    "Preview"
                }
                a {
                    class: "text-[0.8rem] text-muted-foreground no-underline transition-colors hover:text-foreground",
                    href: "#installation",
                    "Installation"
                }
                a {
                    class: "text-[0.8rem] text-muted-foreground no-underline transition-colors hover:text-foreground",
                    href: "#usage",
                    "Usage"
                }
                if !extra_variants.is_empty() {
                    a {
                        class: "text-[0.8rem] text-muted-foreground no-underline transition-colors hover:text-foreground",
                        href: "#examples",
                        "Examples"
                    }
                    for variant in extra_variants {
                        a {
                            class: "pl-4 text-[0.8rem] text-muted-foreground no-underline transition-colors hover:text-foreground capitalize",
                            href: "#{variant.name}",
                            {variant.name.replace("_", " ")}
                        }
                    }
                }
                if !meta.api_docs.is_empty() {
                    a {
                        class: "text-[0.8rem] text-muted-foreground no-underline transition-colors hover:text-foreground",
                        href: "#api-reference",
                        "API Reference"
                    }
                }
            }
        }
    }
}
