//! SSR snapshot tests for the styled InputOTP component.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::input_otp::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn input_otp_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            InputOTP { max_length: 6,
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("has-disabled:opacity-50"),
        "root has container class: {html}"
    );
}

#[test]
fn input_otp_group_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            InputOTP { max_length: 4,
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                }
            }
        }
    }

    let html = render(TestApp);
    // The group div should have flex items-center from styled layer
    assert!(
        html.contains("flex items-center"),
        "group has flex class: {html}"
    );
}

#[test]
fn input_otp_slot_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            InputOTP { max_length: 4,
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("border-input"),
        "slot has border-input class: {html}"
    );
    assert!(
        html.contains("first:rounded-l-md"),
        "slot has rounded class: {html}"
    );
}

#[test]
fn full_styled_otp_composition() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            InputOTP { max_length: 6, value: "12",
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

    let html = render(TestApp);
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
    assert!(
        html.contains("data-filled=\"true\""),
        "has filled slot: {html}"
    );
}
