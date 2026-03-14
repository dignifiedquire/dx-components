use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(String::new);

    rsx! {
        InputOTP {
            max_length: 6,
            value: value(),
            on_change: move |v: String| value.set(v),
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
