use dioxus::prelude::*;

use crate::ui::navbar::Navbar;
use crate::ui::sidebar::Sidebar;
use crate::ui::toc::TableOfContents;
use crate::Route;

/// Three-column docs layout matching shadcn: left sidebar | main content | right TOC.
#[component]
pub(crate) fn DocsLayout() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/dx-components-theme.css"),
        }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        Navbar {}
        div { class: "flex flex-1 flex-col px-2",
            div { class: "min-h-min flex-1 items-start px-0 lg:grid lg:grid-cols-[18rem_minmax(0,1fr)]",
                Sidebar {}
                div { class: "flex min-w-0 flex-1 items-stretch",
                    div { class: "flex min-w-0 flex-1 flex-col",
                        div { class: "mx-auto flex w-full max-w-[40rem] min-w-0 flex-1 flex-col gap-6 px-4 py-6 lg:py-8 md:px-0",
                            Outlet::<Route> {}
                        }
                    }
                    TableOfContents {}
                }
            }
        }
    }
}
