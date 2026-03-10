//! SSR snapshot tests for the styled form component.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::form::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn form_item_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Form {
                FormField { name: "test",
                    FormItem {
                        div { "content" }
                    }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("grid"), "has grid class: {html}");
    assert!(html.contains("gap-2"), "has gap-2 class: {html}");
}

#[test]
fn form_label_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Form {
                FormField { name: "test",
                    FormItem {
                        FormLabel { "Test" }
                    }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("data-[error=true]:text-destructive"),
        "has error class: {html}"
    );
}

#[test]
fn form_description_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Form {
                FormField { name: "test",
                    FormItem {
                        FormDescription { "Help text" }
                    }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("text-sm"), "has text-sm class: {html}");
    assert!(
        html.contains("text-muted-foreground"),
        "has text-muted-foreground class: {html}"
    );
}

#[test]
fn form_message_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Form {
                FormField { name: "test", error: "Error".to_string(),
                    FormItem {
                        FormMessage {}
                    }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("text-sm"), "has text-sm class: {html}");
    assert!(
        html.contains("text-destructive"),
        "has text-destructive class: {html}"
    );
    assert!(html.contains("Error"), "has error text: {html}");
}

#[test]
fn form_item_custom_class_merged() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Form {
                FormField { name: "test",
                    FormItem { class: "my-custom",
                        div { "content" }
                    }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("my-custom"), "has custom class: {html}");
    assert!(html.contains("grid"), "has base class: {html}");
}

#[test]
fn full_styled_form_composition() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Form {
                FormField { name: "email", error: "Required".to_string(),
                    FormItem {
                        FormLabel { "Email" }
                        FormControl {
                            input { r#type: "email" }
                        }
                        FormDescription { "Your email address." }
                        FormMessage {}
                    }
                }
                FormSubmit { "Send" }
            }
        }
    }

    let html = render(TestApp);

    // All slots present
    assert!(html.contains("data-slot=\"form\""), "form slot: {html}");
    assert!(
        html.contains("data-slot=\"form-field\""),
        "field slot: {html}"
    );
    assert!(
        html.contains("data-slot=\"form-item\""),
        "item slot: {html}"
    );
    assert!(
        html.contains("data-slot=\"form-label\""),
        "label slot: {html}"
    );
    assert!(
        html.contains("data-slot=\"form-control\""),
        "control slot: {html}"
    );
    assert!(
        html.contains("data-slot=\"form-description\""),
        "description slot: {html}"
    );
    assert!(
        html.contains("data-slot=\"form-message\""),
        "message slot: {html}"
    );
    assert!(
        html.contains("data-slot=\"form-submit\""),
        "submit slot: {html}"
    );

    // Styled classes
    assert!(html.contains("grid"), "item grid: {html}");
    assert!(
        html.contains("text-muted-foreground"),
        "description class: {html}"
    );
    assert!(html.contains("text-destructive"), "message class: {html}");
}
