use dioxus::prelude::*;
use dioxus_primitives::button::{Button, ButtonVariant};
use dioxus_primitives::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
use dioxus_primitives::label::Label;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Button {
            variant: ButtonVariant::Outline,
            onclick: move |_| open.set(true),
            "Edit Profile"
        }
        DialogRoot { open: open(), on_open_change: move |v| open.set(v),
            DialogContent {
                div { class: "flex flex-col gap-1.5",
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
                div { class: "flex justify-end",
                    Button {
                        onclick: move |_| open.set(false),
                        "Save changes"
                    }
                }
            }
        }
    }
}
