use super::super::component::*;
use crate::components::button::component::Button;
use crate::components::label::component::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tabs {
            default_value: "account".to_string(),
            class: "flex w-full max-w-sm flex-col gap-2",
            TabsList {
                TabsTrigger { value: "account".to_string(), "Account" }
                TabsTrigger { value: "password".to_string(), "Password" }
            }
            TabsContent { value: "account".to_string(),
                div { class: "rounded-lg border bg-card p-6 text-card-foreground shadow-sm",
                    div { class: "flex flex-col gap-1.5",
                        h3 { class: "text-lg font-semibold leading-none tracking-tight", "Account" }
                        p { class: "text-sm text-muted-foreground", "Make changes to your account here. Click save when you're done." }
                    }
                    div { class: "grid gap-4 py-4",
                        div { class: "grid gap-2",
                            Label { html_for: "tab-name", "Name" }
                            input { id: "tab-name", class: "input", value: "Pedro Duarte" }
                        }
                        div { class: "grid gap-2",
                            Label { html_for: "tab-username", "Username" }
                            input { id: "tab-username", class: "input", value: "@peduarte" }
                        }
                    }
                    div { class: "flex pt-2",
                        Button { "Save changes" }
                    }
                }
            }
            TabsContent { value: "password".to_string(),
                div { class: "rounded-lg border bg-card p-6 text-card-foreground shadow-sm",
                    div { class: "flex flex-col gap-1.5",
                        h3 { class: "text-lg font-semibold leading-none tracking-tight", "Password" }
                        p { class: "text-sm text-muted-foreground", "Change your password here. After saving, you'll be logged out." }
                    }
                    div { class: "grid gap-4 py-4",
                        div { class: "grid gap-2",
                            Label { html_for: "tab-current", "Current password" }
                            input { id: "tab-current", class: "input", r#type: "password" }
                        }
                        div { class: "grid gap-2",
                            Label { html_for: "tab-new", "New password" }
                            input { id: "tab-new", class: "input", r#type: "password" }
                        }
                    }
                    div { class: "flex pt-2",
                        Button { "Save password" }
                    }
                }
            }
        }
    }
}
