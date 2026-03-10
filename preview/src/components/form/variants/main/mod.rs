use crate::components::button::component::{Button, ButtonVariant};
use crate::components::form::component::*;
use crate::components::input::component::Input;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut username_error = use_signal(|| None::<String>);
    let mut email_error = use_signal(|| None::<String>);

    rsx! {
        Form {
            on_submit: move |_| {
                // Simple validation demo
                let mut valid = true;
                // In a real app you'd read the form values here
                if username_error().is_some() {
                    valid = false;
                }
                if email_error().is_some() {
                    valid = false;
                }
                if valid {
                    tracing::info!("Form submitted successfully!");
                }
            },
            style: "display: flex; flex-direction: column; gap: 1.5rem; max-width: 28rem;",

            FormField { name: "username", error: username_error(),
                FormItem {
                    FormLabel { "Username" }
                    FormControl {
                        Input {
                            placeholder: "shadcn",
                            oninput: move |e: FormEvent| {
                                let val = e.value();
                                if val.len() < 2 {
                                    username_error.set(Some("Username must be at least 2 characters.".into()));
                                } else {
                                    username_error.set(None);
                                }
                            },
                        }
                    }
                    FormDescription { "This is your public display name." }
                    FormMessage {}
                }
            }

            FormField { name: "email", error: email_error(),
                FormItem {
                    FormLabel { "Email" }
                    FormControl {
                        Input {
                            r#type: "email",
                            placeholder: "you@example.com",
                            oninput: move |e: FormEvent| {
                                let val = e.value();
                                if !val.is_empty() && !val.contains('@') {
                                    email_error.set(Some("Please enter a valid email address.".into()));
                                } else {
                                    email_error.set(None);
                                }
                            },
                        }
                    }
                    FormDescription { "We'll never share your email." }
                    FormMessage {}
                }
            }

            Button { variant: ButtonVariant::Default, r#type: "submit",
                "Submit"
            }
        }
    }
}
