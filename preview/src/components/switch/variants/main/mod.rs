use dioxus::prelude::*;
use dioxus_primitives::switch::{Switch, SwitchThumb};

#[component]
pub fn Demo() -> Element {
    let mut checked = use_signal(|| false);
    rsx! {
        Switch {
            style: "width: 2rem; height: 1.15rem; border-radius: 9999px; background: var(--muted); border: none; padding: 0; cursor: pointer; position: relative;",
            checked: checked(),
            aria_label: "Switch Demo",
            on_checked_change: move |new_checked| {
                checked.set(new_checked);
            },
            SwitchThumb {
                style: "display: block; width: 1rem; height: 1rem; border-radius: 50%; background: var(--foreground); transform: translateX(1px);",
            }
        }
    }
}
