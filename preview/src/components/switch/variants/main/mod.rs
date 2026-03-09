use crate::components::switch::component::Switch;
use dioxus::prelude::*;

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
        }
    }
}
