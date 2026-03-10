use crate::components::label::component::Label;
use crate::components::switch::component::Switch;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut checked = use_signal(|| false);
    rsx! {
        div { class: "flex items-center gap-2",
            Switch {
                id: "airplane-mode",
                checked: checked(),
                on_checked_change: move |new_checked| {
                    checked.set(new_checked);
                },
            }
            Label { html_for: "airplane-mode", "Airplane Mode" }
        }
    }
}
