use dioxus::prelude::*;
use dioxus_primitives::dialog::*;
use dioxus_primitives::label::Label;

#[component]
pub fn Demo() -> Element {
    rsx! {
        DialogRoot {
            DialogTrigger {
                class: "inline-flex items-center justify-center gap-2 rounded-md border border-input bg-background px-4 py-2 text-sm font-medium shadow-xs hover:bg-accent hover:text-accent-foreground",
                "Edit Profile"
            }
            DialogOverlay {
                class: "fixed inset-0 z-50 bg-black/50 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0",
            }
            DialogContent {
                class: "fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border bg-background p-6 shadow-lg sm:max-w-lg",
                div { class: "flex flex-col gap-1.5",
                    DialogTitle { class: "text-lg font-semibold leading-none", "Edit profile" }
                    DialogDescription { class: "text-sm text-muted-foreground", "Make changes to your profile here. Click save when you're done." }
                }
                div { class: "grid gap-4 py-4",
                    div { class: "grid grid-cols-4 items-center gap-4",
                        Label { html_for: "name", class: "text-right", "Name" }
                        input {
                            id: "name",
                            class: "input col-span-3",
                            value: "Pedro Duarte",
                        }
                    }
                    div { class: "grid grid-cols-4 items-center gap-4",
                        Label { html_for: "username", class: "text-right", "Username" }
                        input {
                            id: "username",
                            class: "input col-span-3",
                            value: "@peduarte",
                        }
                    }
                }
                div { class: "flex justify-end",
                    DialogClose {
                        class: "inline-flex items-center justify-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-xs hover:bg-primary/90",
                        "Save changes"
                    }
                }
            }
        }
    }
}
