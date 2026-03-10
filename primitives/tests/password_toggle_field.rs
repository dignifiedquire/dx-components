//! SSR snapshot tests for the PasswordToggleField primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::password_toggle_field::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn root_renders_container() {
    fn App() -> Element {
        rsx! {
            PasswordToggleField {
                PasswordToggleFieldInput {}
                PasswordToggleFieldToggle {
                    "Toggle"
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"password-toggle-field\""),
        "has root data-slot: {html}"
    );
    assert!(
        html.contains("data-visible=\"false\""),
        "defaults to hidden: {html}"
    );
}

#[test]
fn input_renders_as_password_by_default() {
    fn App() -> Element {
        rsx! {
            PasswordToggleField {
                PasswordToggleFieldInput { placeholder: "Enter password" }
                PasswordToggleFieldToggle { "Toggle" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"password-toggle-field-input\""),
        "has input data-slot: {html}"
    );
    assert!(
        html.contains("type=\"password\""),
        "input type is password: {html}"
    );
    assert!(
        html.contains("placeholder=\"Enter password\""),
        "has placeholder: {html}"
    );
    assert!(
        html.contains("autocomplete=\"current-password\""),
        "has autocomplete: {html}"
    );
    assert!(
        html.contains("spellcheck=\"false\""),
        "has spellcheck false: {html}"
    );
}

#[test]
fn input_renders_as_text_when_visible() {
    fn App() -> Element {
        rsx! {
            PasswordToggleField { default_visible: true,
                PasswordToggleFieldInput {}
                PasswordToggleFieldToggle { "Toggle" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("type=\"text\""),
        "input type is text when visible: {html}"
    );
    assert!(
        html.contains("data-visible=\"true\""),
        "root shows visible: {html}"
    );
}

#[test]
fn toggle_renders_with_aria_label() {
    fn App() -> Element {
        rsx! {
            PasswordToggleField {
                PasswordToggleFieldInput {}
                PasswordToggleFieldToggle {
                    "Toggle"
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"password-toggle-field-toggle\""),
        "has toggle data-slot: {html}"
    );
    assert!(
        html.contains("type=\"button\""),
        "toggle type is button: {html}"
    );
    assert!(
        html.contains("aria-label=\"Show password\""),
        "has aria-label: {html}"
    );
}

#[test]
fn toggle_aria_label_when_visible() {
    fn App() -> Element {
        rsx! {
            PasswordToggleField { default_visible: true,
                PasswordToggleFieldInput {}
                PasswordToggleFieldToggle {
                    "Toggle"
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("aria-label=\"Hide password\""),
        "has hide aria-label: {html}"
    );
}

#[test]
fn icon_renders_hidden_by_default() {
    fn App() -> Element {
        rsx! {
            PasswordToggleField {
                PasswordToggleFieldInput {}
                PasswordToggleFieldToggle {
                    PasswordToggleFieldIcon {
                        visible: rsx! { span { "EYE" } },
                        hidden: rsx! { span { "EYE-OFF" } },
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"password-toggle-field-icon\""),
        "has icon data-slot: {html}"
    );
    assert!(html.contains("EYE-OFF"), "shows hidden icon: {html}");
    assert!(!html.contains("EYE</span>"), "hides visible icon: {html}");
}

#[test]
fn icon_renders_visible_when_shown() {
    fn App() -> Element {
        rsx! {
            PasswordToggleField { default_visible: true,
                PasswordToggleFieldInput {}
                PasswordToggleFieldToggle {
                    PasswordToggleFieldIcon {
                        visible: rsx! { span { "EYE" } },
                        hidden: rsx! { span { "EYE-OFF" } },
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains(">EYE<"), "shows visible icon: {html}");
    assert!(!html.contains("EYE-OFF"), "hides hidden icon: {html}");
}

#[test]
fn icon_has_aria_hidden() {
    fn App() -> Element {
        rsx! {
            PasswordToggleField {
                PasswordToggleFieldInput {}
                PasswordToggleFieldToggle {
                    PasswordToggleFieldIcon {
                        visible: rsx! { "V" },
                        hidden: rsx! { "H" },
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("aria-hidden=\"true\""),
        "icon has aria-hidden: {html}"
    );
}

#[test]
fn input_disabled_state() {
    fn App() -> Element {
        rsx! {
            PasswordToggleField {
                PasswordToggleFieldInput { disabled: true }
                PasswordToggleFieldToggle { "Toggle" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("disabled=true"), "input is disabled: {html}");
}

#[test]
fn full_composition() {
    fn App() -> Element {
        rsx! {
            PasswordToggleField {
                PasswordToggleFieldInput {
                    placeholder: "Password",
                    name: "password",
                }
                PasswordToggleFieldToggle {
                    PasswordToggleFieldIcon {
                        visible: rsx! { span { "👁" } },
                        hidden: rsx! { span { "👁‍🗨" } },
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"password-toggle-field\""),
        "root: {html}"
    );
    assert!(
        html.contains("data-slot=\"password-toggle-field-input\""),
        "input: {html}"
    );
    assert!(
        html.contains("data-slot=\"password-toggle-field-toggle\""),
        "toggle: {html}"
    );
    assert!(
        html.contains("data-slot=\"password-toggle-field-icon\""),
        "icon: {html}"
    );
    assert!(html.contains("name=\"password\""), "has name: {html}");
}
