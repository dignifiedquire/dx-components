use crate::components::dialog::component::*;
use crate::components::label::component::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Dialog {
            DialogTrigger { "Edit Profile" }
            DialogOverlay {}
            DialogContent {
                DialogHeader {
                    DialogTitle { "Edit profile" }
                    DialogDescription { "Make changes to your profile here. Click save when you're done." }
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
                DialogFooter {
                    DialogClose { "Save changes" }
                }
            }
        }
    }
}
