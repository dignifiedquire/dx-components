use dioxus::prelude::*;
use dx_icons_lucide::{IconGithub, IconMenu};

use crate::components;
use crate::components::sheet::component::*;
use crate::ui::dark_mode::DarkModeToggle;
use crate::ui::language_select::LanguageSelect;
use crate::Route;

#[component]
pub(crate) fn Navbar() -> Element {
    let mut mobile_open = use_signal(|| Option::<bool>::None);

    rsx! {
        header {
            class: "sticky top-0 z-50 w-full h-14 border-b border-border bg-background backdrop-blur",
            div { class: "flex h-full items-center px-6",
                // Mobile hamburger
                button {
                    r#type: "button",
                    class: "lg:hidden inline-flex items-center justify-center rounded-md p-2 mr-2 text-muted-foreground hover:text-foreground hover:bg-accent transition-colors",
                    aria_label: "Toggle navigation",
                    onclick: move |_| mobile_open.set(Some(true)),
                    IconMenu { size: 20 }
                }
                // Brand
                Link { to: Route::home(), class: "flex items-center gap-2 mr-6 no-underline",
                    img {
                        src: asset!("/assets/dioxus_color.svg"),
                        alt: "Dioxus Logo",
                        width: "28",
                        height: "28",
                    }
                    span { class: "hidden sm:inline-block text-sm font-bold text-foreground",
                        "Dioxus UI"
                    }
                }
                // Center nav
                nav { class: "hidden md:flex items-center gap-6 text-sm",
                    Link {
                        to: Route::component("button"),
                        class: "text-muted-foreground hover:text-foreground no-underline transition-colors",
                        "Components"
                    }
                }
                // Spacer
                div { class: "flex-1" }
                // Right actions
                div { class: "flex items-center gap-2",
                    Link {
                        to: "https://github.com/DioxusLabs/components",
                        class: "inline-flex items-center justify-center rounded-md p-2 text-muted-foreground hover:text-foreground hover:bg-accent transition-colors",
                        aria_label: "GitHub",
                        IconGithub { size: 20 }
                    }
                    DarkModeToggle {}
                    LanguageSelect {}
                }
            }
        }
        // Mobile sidebar sheet
        MobileSidebar { open: mobile_open }
    }
}

#[component]
fn MobileSidebar(open: Signal<Option<bool>>) -> Element {
    let route: Route = router().current();
    let current_name = match &route {
        Route::ComponentPage { name } => Some(name.as_str()),
        _ => None,
    };

    let mut sorted_demos: Vec<_> = components::DEMOS.iter().collect();
    sorted_demos.sort_by_key(|d| d.name);

    rsx! {
        Sheet { open,
            SheetContent { side: SheetSide::Left,
                SheetHeader {
                    SheetTitle { "Navigation" }
                }
                nav { class: "flex flex-col gap-0.5 mt-4 overflow-y-auto",
                    h4 { class: "mb-1 rounded-md px-2 py-1 text-sm font-semibold text-foreground",
                        "Components"
                    }
                    for demo in sorted_demos {
                        {
                            let is_active = current_name == Some(demo.name);
                            let display_name = demo.name.replace("_", " ");
                            rsx! {
                                Link {
                                    to: Route::component(demo.name),
                                    onclick: move |_| open.set(Some(false)),
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
