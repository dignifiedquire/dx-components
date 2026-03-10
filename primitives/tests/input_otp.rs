//! SSR snapshot tests for the InputOTP primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::input_otp::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

// ---------------------------------------------------------------------------
// InputOTP (root)
// ---------------------------------------------------------------------------

#[test]
fn input_otp_renders_container() {
    fn App() -> Element {
        rsx! {
            InputOTP { max_length: 6,
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"input-otp\""),
        "has root data-slot: {html}"
    );
}

#[test]
fn input_otp_renders_hidden_input() {
    fn App() -> Element {
        rsx! {
            InputOTP { max_length: 4,
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"input-otp-input\""),
        "has hidden input: {html}"
    );
    assert!(
        html.contains("inputmode=\"numeric\""),
        "has inputmode numeric: {html}"
    );
    assert!(
        html.contains("autocomplete=\"one-time-code\""),
        "has autocomplete: {html}"
    );
    assert!(html.contains("maxlength=4"), "has maxlength: {html}");
}

#[test]
fn input_otp_disabled_state() {
    fn App() -> Element {
        rsx! {
            InputOTP { max_length: 4, disabled: true,
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-disabled=\"true\""),
        "has data-disabled: {html}"
    );
    assert!(html.contains("disabled=true"), "input is disabled: {html}");
}

// ---------------------------------------------------------------------------
// InputOTPGroup
// ---------------------------------------------------------------------------

#[test]
fn input_otp_group_renders() {
    fn App() -> Element {
        rsx! {
            InputOTP { max_length: 4,
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                    InputOTPSlot { index: 1 }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"input-otp-group\""),
        "has group data-slot: {html}"
    );
}

// ---------------------------------------------------------------------------
// InputOTPSlot
// ---------------------------------------------------------------------------

#[test]
fn input_otp_slot_renders_empty() {
    fn App() -> Element {
        rsx! {
            InputOTP { max_length: 4,
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"input-otp-slot\""),
        "has slot data-slot: {html}"
    );
}

#[test]
fn input_otp_slot_shows_character() {
    fn App() -> Element {
        rsx! {
            InputOTP { max_length: 4, value: "12",
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                    InputOTPSlot { index: 1 }
                    InputOTPSlot { index: 2 }
                }
            }
        }
    }

    let html = render(App);
    // Slots 0 and 1 should have data-filled
    assert!(
        html.contains("data-filled=\"true\""),
        "filled slots present: {html}"
    );
}

// ---------------------------------------------------------------------------
// InputOTPSeparator
// ---------------------------------------------------------------------------

#[test]
fn input_otp_separator_renders() {
    fn App() -> Element {
        rsx! {
            InputOTP { max_length: 6,
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                }
                InputOTPSeparator {}
                InputOTPGroup {
                    InputOTPSlot { index: 1 }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"input-otp-separator\""),
        "has separator data-slot: {html}"
    );
    assert!(
        html.contains("role=\"separator\""),
        "has separator role: {html}"
    );
    // Default minus icon
    assert!(html.contains("<svg"), "has default minus icon: {html}");
}

#[test]
fn input_otp_separator_custom_children() {
    fn App() -> Element {
        rsx! {
            InputOTP { max_length: 6,
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                }
                InputOTPSeparator {
                    span { "-" }
                }
                InputOTPGroup {
                    InputOTPSlot { index: 1 }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("<span>-</span>"),
        "has custom separator: {html}"
    );
}

// ---------------------------------------------------------------------------
// Full composition
// ---------------------------------------------------------------------------

#[test]
fn full_otp_composition() {
    fn App() -> Element {
        rsx! {
            InputOTP { max_length: 6, value: "123",
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                    InputOTPSlot { index: 1 }
                    InputOTPSlot { index: 2 }
                }
                InputOTPSeparator {}
                InputOTPGroup {
                    InputOTPSlot { index: 3 }
                    InputOTPSlot { index: 4 }
                    InputOTPSlot { index: 5 }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("data-slot=\"input-otp\""), "has root: {html}");
    assert!(
        html.contains("data-slot=\"input-otp-group\""),
        "has group: {html}"
    );
    assert!(
        html.contains("data-slot=\"input-otp-slot\""),
        "has slot: {html}"
    );
    assert!(
        html.contains("data-slot=\"input-otp-separator\""),
        "has separator: {html}"
    );
    assert!(html.contains("maxlength=6"), "has maxlength: {html}");
}

// ---------------------------------------------------------------------------
// Pattern matching
// ---------------------------------------------------------------------------

#[test]
fn pattern_matcher_digits_only() {
    use dioxus_primitives::input_otp::matches_char_pattern;

    assert!(matches_char_pattern("[0-9]", '5'));
    assert!(matches_char_pattern("[0-9]", '0'));
    assert!(matches_char_pattern("[0-9]", '9'));
    assert!(!matches_char_pattern("[0-9]", 'a'));
    assert!(!matches_char_pattern("[0-9]", 'Z'));
}

#[test]
fn pattern_matcher_alphanumeric() {
    use dioxus_primitives::input_otp::matches_char_pattern;

    assert!(matches_char_pattern("[a-zA-Z0-9]", 'a'));
    assert!(matches_char_pattern("[a-zA-Z0-9]", 'Z'));
    assert!(matches_char_pattern("[a-zA-Z0-9]", '5'));
    assert!(!matches_char_pattern("[a-zA-Z0-9]", '!'));
    assert!(!matches_char_pattern("[a-zA-Z0-9]", ' '));
}
