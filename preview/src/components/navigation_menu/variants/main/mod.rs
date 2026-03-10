use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        NavigationMenu {
            NavigationMenuList {
                NavigationMenuItem {
                    NavigationMenuTrigger { "Getting Started" }
                    NavigationMenuContent {
                        ul { class: "grid gap-3 p-4 md:w-[400px] lg:w-[500px] lg:grid-cols-[.75fr_1fr]",
                            li { class: "row-span-3",
                                NavigationMenuLink { href: "#",
                                    div { class: "flex h-full w-full select-none flex-col justify-end rounded-md bg-gradient-to-b from-muted/50 to-muted p-6 no-underline outline-none focus:shadow-md",
                                        div { class: "mb-2 mt-4 text-lg font-medium", "shadcn/ui" }
                                        p { class: "text-sm leading-tight text-muted-foreground",
                                            "Beautifully designed components built with Radix UI and Tailwind CSS."
                                        }
                                    }
                                }
                            }
                            li {
                                NavigationMenuLink { href: "#",
                                    div { class: "block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground",
                                        div { class: "text-sm font-medium leading-none", "Introduction" }
                                        p { class: "line-clamp-2 text-sm leading-snug text-muted-foreground",
                                            "Re-usable components built using Radix UI and Tailwind CSS."
                                        }
                                    }
                                }
                            }
                            li {
                                NavigationMenuLink { href: "#",
                                    div { class: "block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground",
                                        div { class: "text-sm font-medium leading-none", "Installation" }
                                        p { class: "line-clamp-2 text-sm leading-snug text-muted-foreground",
                                            "How to install dependencies and structure your app."
                                        }
                                    }
                                }
                            }
                            li {
                                NavigationMenuLink { href: "#",
                                    div { class: "block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground",
                                        div { class: "text-sm font-medium leading-none", "Typography" }
                                        p { class: "line-clamp-2 text-sm leading-snug text-muted-foreground",
                                            "Styles for headings, paragraphs, lists... etc."
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                NavigationMenuItem {
                    NavigationMenuTrigger { "Components" }
                    NavigationMenuContent {
                        ul { class: "grid w-[400px] gap-3 p-4 md:w-[500px] md:grid-cols-2 lg:w-[600px]",
                            li {
                                NavigationMenuLink { href: "#",
                                    div { class: "block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground",
                                        div { class: "text-sm font-medium leading-none", "Alert Dialog" }
                                        p { class: "line-clamp-2 text-sm leading-snug text-muted-foreground",
                                            "A modal dialog that interrupts the user with important content."
                                        }
                                    }
                                }
                            }
                            li {
                                NavigationMenuLink { href: "#",
                                    div { class: "block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground",
                                        div { class: "text-sm font-medium leading-none", "Hover Card" }
                                        p { class: "line-clamp-2 text-sm leading-snug text-muted-foreground",
                                            "For sighted users to preview content available behind a link."
                                        }
                                    }
                                }
                            }
                            li {
                                NavigationMenuLink { href: "#",
                                    div { class: "block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground",
                                        div { class: "text-sm font-medium leading-none", "Progress" }
                                        p { class: "line-clamp-2 text-sm leading-snug text-muted-foreground",
                                            "Displays an indicator showing the completion progress of a task."
                                        }
                                    }
                                }
                            }
                            li {
                                NavigationMenuLink { href: "#",
                                    div { class: "block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground",
                                        div { class: "text-sm font-medium leading-none", "Tooltip" }
                                        p { class: "line-clamp-2 text-sm leading-snug text-muted-foreground",
                                            "A popup that displays information related to an element."
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                NavigationMenuItem {
                    NavigationMenuLink { href: "#", class: "{NAVIGATION_MENU_TRIGGER_STYLE}",
                        "Documentation"
                    }
                }
            }
            NavigationMenuViewport {}
        }
    }
}
