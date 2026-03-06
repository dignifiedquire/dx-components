use dioxus::prelude::*;

use crate::components;
use crate::Route;

/// Right "On This Page" table of contents matching shadcn's docs-toc.
#[component]
pub(crate) fn TableOfContents() -> Element {
    let route: Route = router().current();
    let demo = match &route {
        Route::ComponentPage { name } => components::DEMOS.iter().find(|d| d.name == name.as_str()),
        _ => None,
    };

    let Some(demo) = demo else {
        return rsx! {};
    };

    let extra_variants = if demo.variants.len() > 1 {
        &demo.variants[1..]
    } else {
        &[]
    };

    rsx! {
        aside {
            class: "sticky top-14 z-30 ml-auto hidden h-[90svh] w-56 flex-col gap-4 overflow-hidden pb-8 xl:flex",
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
            }
        }
    }
}
