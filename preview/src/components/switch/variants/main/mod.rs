use dioxus::prelude::*;
use dioxus_primitives::switch::{Switch, SwitchThumb};

#[component]
pub fn Demo() -> Element {
    let mut checked = use_signal(|| false);
    rsx! {
        Switch {
            checked: checked(),
            aria_label: "Switch Demo",
            on_checked_change: move |new_checked| {
                checked.set(new_checked);
            },
            SwitchThumb {}
        }
    }
}
