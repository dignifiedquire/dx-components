use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(String::new);

    rsx! {
        InputOTP {
            max_length: 4,
            value: value(),
            on_change: move |v: String| value.set(v),
            InputOTPGroup { class: "gap-2",
                InputOTPSlot { index: 0, class: "rounded-md border" }
                InputOTPSlot { index: 1, class: "rounded-md border" }
                InputOTPSlot { index: 2, class: "rounded-md border" }
                InputOTPSlot { index: 3, class: "rounded-md border" }
            }
        }
    }
}
