use dioxus::prelude::*;
use dx_icons_tabler::{IconMoon, IconSun};

use crate::theme;

#[component]
pub(crate) fn DarkModeToggle() -> Element {
    rsx! {
        button {
            class: "dark-mode-toggle dark-mode-only",
            onclick: move |_| {
                theme::set_theme(false);
            },
            r#type: "button",
            aria_label: "Enable light mode",
            IconMoon { size: 20 }
        }
        button {
            class: "dark-mode-toggle light-mode-only",
            onclick: move |_| {
                theme::set_theme(true);
            },
            r#type: "button",
            aria_label: "Enable dark mode",
            IconSun { size: 20 }
        }
    }
}
