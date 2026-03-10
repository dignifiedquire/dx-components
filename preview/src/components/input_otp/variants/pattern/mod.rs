use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        InputOTP { max_length: 6, pattern: "^[0-9]*$",
            InputOTPGroup {
                InputOTPSlot { index: 0 }
                InputOTPSlot { index: 1 }
                InputOTPSlot { index: 2 }
                InputOTPSlot { index: 3 }
                InputOTPSlot { index: 4 }
                InputOTPSlot { index: 5 }
            }
        }
    }
}
