use super::super::component::*;
use crate::components::button::{Button, ButtonVariant};
use crate::components::input::Input;
use crate::components::label::Label;
use dioxus::prelude::*;

// Mirrors shadcn's card-rtl under `dir="rtl"`. Upstream wires a live
// language switcher (Arabic/Hebrew dictionaries via site-only
// `@/components/language-selector`) which is app infra, not part of the
// component — labels are static Arabic here (documented deviation).
// Note `ms-auto` (logical) replaces the LTR demo's `ml-auto`.
#[component]
pub fn Demo() -> Element {
    rsx! {
        Card { class: "w-full max-w-sm", dir: "rtl",
            CardHeader {
                CardTitle { "تسجيل الدخول إلى حسابك" }
                CardDescription { "أدخل بريدك الإلكتروني أدناه لتسجيل الدخول إلى حسابك" }
                CardAction {
                    Button { variant: ButtonVariant::Link, "إنشاء حساب" }
                }
            }
            CardContent {
                form {
                    div { class: "flex flex-col gap-6",
                        div { class: "grid gap-2",
                            Label { html_for: "email-rtl", "البريد الإلكتروني" }
                            Input {
                                id: "email-rtl",
                                r#type: "email",
                                placeholder: "m@example.com",
                                required: "true",
                            }
                        }
                        div { class: "grid gap-2",
                            div { class: "flex items-center",
                                Label { html_for: "password-rtl", "كلمة المرور" }
                                a {
                                    href: "#",
                                    class: "ms-auto inline-block text-sm underline-offset-4 hover:underline",
                                    "نسيت كلمة المرور؟"
                                }
                            }
                            Input { id: "password-rtl", r#type: "password", required: "true" }
                        }
                    }
                }
            }
            CardFooter { class: "flex-col gap-2",
                Button { r#type: "submit", class: "w-full", "تسجيل الدخول" }
                Button { variant: ButtonVariant::Outline, class: "w-full", "تسجيل الدخول باستخدام Google" }
            }
        }
    }
}
