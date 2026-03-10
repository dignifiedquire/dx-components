use crate::components::button::component::{Button, ButtonVariant};
use crate::components::dialog::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Dialog {
            DialogTrigger {
                Button { variant: ButtonVariant::Outline, "Terms & Conditions" }
            }
            DialogOverlay {}
            DialogContent {
                DialogHeader {
                    DialogTitle { "Terms of Service" }
                    DialogDescription { "Please read our terms of service carefully." }
                }
                div { class: "max-h-[60vh] overflow-y-auto pr-2 text-sm leading-relaxed text-muted-foreground",
                    p { class: "mb-4",
                        "These Terms of Service govern your use of the website and the services provided. By accessing or using our services, you agree to be bound by these terms."
                    }
                    h4 { class: "mb-2 font-medium text-foreground", "1. Use of Services" }
                    p { class: "mb-4",
                        "You agree to use the services only for lawful purposes and in accordance with these Terms. You must not use the services in any way that could damage, disable, overburden, or impair the services."
                    }
                    h4 { class: "mb-2 font-medium text-foreground", "2. User Accounts" }
                    p { class: "mb-4",
                        "When you create an account with us, you must provide accurate and complete information. You are responsible for maintaining the confidentiality of your account credentials."
                    }
                    h4 { class: "mb-2 font-medium text-foreground", "3. Intellectual Property" }
                    p { class: "mb-4",
                        "The services and all associated content, features, and functionality are owned by us and are protected by intellectual property laws."
                    }
                    h4 { class: "mb-2 font-medium text-foreground", "4. Privacy" }
                    p { class: "mb-4",
                        "Your use of the services is also governed by our Privacy Policy. By using our services, you consent to the collection and use of information as described in the Privacy Policy."
                    }
                    h4 { class: "mb-2 font-medium text-foreground", "5. Termination" }
                    p {
                        "We may terminate or suspend your access to the services at any time, without prior notice, for any reason. Upon termination, your right to use the services will immediately cease."
                    }
                }
                DialogFooter {
                    DialogClose {
                        Button { "I Accept" }
                    }
                }
            }
        }
    }
}
