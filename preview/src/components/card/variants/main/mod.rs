use super::super::component::*;
use crate::components::button::{Button, ButtonVariant};
use crate::components::input::Input;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Card { class: "w-full max-w-sm",
            CardHeader {
                CardTitle { "Login to your account" }
                CardDescription { "Enter your email below to login to your account" }
                CardAction {
                    Button { variant: ButtonVariant::Link, "Sign Up" }
                }
            }
            CardContent {
                form {
                    div { class: "flex flex-col gap-6",
                        div { class: "grid gap-2",
                            Label { html_for: "email", "Email" }
                            Input {
                                id: "email",
                                r#type: "email",
                                placeholder: "m@example.com",
                                required: "true",
                            }
                        }
                        div { class: "grid gap-2",
                            div { class: "flex items-center",
                                Label { html_for: "password", "Password" }
                                a {
                                    href: "#",
                                    class: "ml-auto inline-block text-sm underline-offset-4 hover:underline",
                                    "Forgot your password?"
                                }
                            }
                            Input { id: "password", r#type: "password", required: "true" }
                        }
                    }
                }
            }
            CardFooter { class: "flex-col gap-2",
                Button { r#type: "submit", class: "w-full", "Login" }
                Button { variant: ButtonVariant::Outline, class: "w-full", "Login with Google" }
            }
        }
    }
}
