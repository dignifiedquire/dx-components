//! SSR snapshot tests for the form primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::form::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

// ---------------------------------------------------------------------------
// Form (root)
// ---------------------------------------------------------------------------

#[test]
fn form_renders_with_novalidate() {
    fn App() -> Element {
        rsx! {
            Form {
                div { "content" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("<form"), "has form element: {html}");
    assert!(html.contains("novalidate=true"), "has novalidate: {html}");
    assert!(html.contains("data-slot=\"form\""), "has data-slot: {html}");
}

#[test]
fn form_renders_with_class() {
    fn App() -> Element {
        rsx! {
            Form { class: "my-form",
                div { "content" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("class=\"my-form\""), "has class: {html}");
}

// ---------------------------------------------------------------------------
// FormField
// ---------------------------------------------------------------------------

#[test]
fn form_field_renders_valid_state() {
    fn App() -> Element {
        rsx! {
            Form {
                FormField { name: "email",
                    div { "field content" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"form-field\""),
        "has data-slot: {html}"
    );
    assert!(
        html.contains("data-valid=\"true\""),
        "has data-valid: {html}"
    );
}

#[test]
fn form_field_renders_invalid_state() {
    fn App() -> Element {
        rsx! {
            Form {
                FormField { name: "email", error: "Required".to_string(),
                    div { "field content" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-invalid=\"true\""),
        "has data-invalid: {html}"
    );
    assert!(
        !html.contains("data-valid=\"true\""),
        "no data-valid when invalid: {html}"
    );
}

// ---------------------------------------------------------------------------
// FormItem
// ---------------------------------------------------------------------------

#[test]
fn form_item_renders_container() {
    fn App() -> Element {
        rsx! {
            Form {
                FormField { name: "test",
                    FormItem {
                        div { "item content" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"form-item\""),
        "has data-slot: {html}"
    );
}

// ---------------------------------------------------------------------------
// FormLabel
// ---------------------------------------------------------------------------

#[test]
fn form_label_wires_for_attribute() {
    fn App() -> Element {
        rsx! {
            Form {
                FormField { name: "email",
                    FormItem {
                        FormLabel { "Email" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"form-label\""),
        "has data-slot: {html}"
    );
    assert!(html.contains("<label"), "renders label element: {html}");
    assert!(html.contains("for=\""), "has for attribute: {html}");
    assert!(html.contains("Email"), "has label text: {html}");
}

#[test]
fn form_label_shows_error_state() {
    fn App() -> Element {
        rsx! {
            Form {
                FormField { name: "email", error: "Required".to_string(),
                    FormItem {
                        FormLabel { "Email" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-error=\"true\""),
        "has data-error when invalid: {html}"
    );
}

#[test]
fn form_label_no_error_state_when_valid() {
    fn App() -> Element {
        rsx! {
            Form {
                FormField { name: "email",
                    FormItem {
                        FormLabel { "Email" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains("data-error=\"true\""),
        "no data-error when valid: {html}"
    );
}

// ---------------------------------------------------------------------------
// FormControl
// ---------------------------------------------------------------------------

#[test]
fn form_control_sets_aria_attributes() {
    fn App() -> Element {
        rsx! {
            Form {
                FormField { name: "email",
                    FormItem {
                        FormControl {
                            input { r#type: "email" }
                        }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"form-control\""),
        "has data-slot: {html}"
    );
    assert!(html.contains("id=\""), "has id: {html}");
    assert!(
        html.contains("aria-describedby=\""),
        "has aria-describedby: {html}"
    );
    assert!(
        html.contains("aria-invalid=false"),
        "aria-invalid is false when valid: {html}"
    );
}

#[test]
fn form_control_aria_invalid_when_error() {
    fn App() -> Element {
        rsx! {
            Form {
                FormField { name: "email", error: "Required".to_string(),
                    FormItem {
                        FormControl {
                            input { r#type: "email" }
                        }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("aria-invalid=true"),
        "aria-invalid is true when error: {html}"
    );
}

// ---------------------------------------------------------------------------
// FormDescription
// ---------------------------------------------------------------------------

#[test]
fn form_description_renders_with_id() {
    fn App() -> Element {
        rsx! {
            Form {
                FormField { name: "email",
                    FormItem {
                        FormDescription { "We'll never share your email." }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"form-description\""),
        "has data-slot: {html}"
    );
    assert!(html.contains("id=\""), "has id: {html}");
    assert!(
        html.contains("never share your email"),
        "has description text: {html}"
    );
}

// ---------------------------------------------------------------------------
// FormMessage
// ---------------------------------------------------------------------------

#[test]
fn form_message_renders_error_text() {
    fn App() -> Element {
        rsx! {
            Form {
                FormField { name: "email", error: "Email is required".to_string(),
                    FormItem {
                        FormMessage {}
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"form-message\""),
        "has data-slot: {html}"
    );
    assert!(html.contains("role=\"alert\""), "has role=alert: {html}");
    assert!(html.contains("Email is required"), "has error text: {html}");
}

#[test]
fn form_message_hidden_when_no_error() {
    fn App() -> Element {
        rsx! {
            Form {
                FormField { name: "email",
                    FormItem {
                        FormMessage {}
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains("data-slot=\"form-message\""),
        "message not rendered when no error: {html}"
    );
}

#[test]
fn form_message_shows_custom_children() {
    fn App() -> Element {
        rsx! {
            Form {
                FormField { name: "email", error: "Error text".to_string(),
                    FormItem {
                        FormMessage {
                            "Custom message"
                        }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("Custom message"),
        "has custom children: {html}"
    );
}

// ---------------------------------------------------------------------------
// FormSubmit
// ---------------------------------------------------------------------------

#[test]
fn form_submit_renders_button() {
    fn App() -> Element {
        rsx! {
            Form {
                FormSubmit { "Submit" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"form-submit\""),
        "has data-slot: {html}"
    );
    assert!(html.contains("type=\"submit\""), "has type=submit: {html}");
    assert!(html.contains("Submit"), "has button text: {html}");
}

#[test]
fn form_submit_disabled() {
    fn App() -> Element {
        rsx! {
            Form {
                FormSubmit { disabled: true, "Submit" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("disabled=true"), "has disabled: {html}");
}

// ---------------------------------------------------------------------------
// Full composition
// ---------------------------------------------------------------------------

#[test]
fn full_form_composition() {
    fn App() -> Element {
        rsx! {
            Form {
                FormField { name: "username", error: "Too short".to_string(),
                    FormItem {
                        FormLabel { "Username" }
                        FormControl {
                            input { r#type: "text" }
                        }
                        FormDescription { "Your display name." }
                        FormMessage {}
                    }
                }
                FormSubmit { "Create account" }
            }
        }
    }

    let html = render(App);

    // All data-slots present
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

    // Error state
    assert!(
        html.contains("data-invalid=\"true\""),
        "invalid state: {html}"
    );
    assert!(html.contains("data-error=\"true\""), "error state: {html}");
    assert!(html.contains("aria-invalid=true"), "aria-invalid: {html}");
    assert!(html.contains("Too short"), "error message: {html}");

    // Content
    assert!(html.contains("Username"), "label text: {html}");
    assert!(
        html.contains("Your display name."),
        "description text: {html}"
    );
    assert!(html.contains("Create account"), "submit text: {html}");
}
