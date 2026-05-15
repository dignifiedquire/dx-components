use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_lucide::{IconArrowLeft, IconEllipsis};

// Mirrors shadcn's button-group-demo. Upstream's last group nests a
// DropdownMenu behind the "More Options" trigger; that demo is deferred
// to the `dropdown` follow-up (DropdownMenu is not yet audited), so here
// it is a plain icon button — the ButtonGroup layout is identical.
#[component]
pub fn Demo() -> Element {
    rsx! {
        ButtonGroup {
            ButtonGroup { class: "hidden sm:flex",
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Icon,
                    aria_label: "Go Back",
                    IconArrowLeft {}
                }
            }
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, "Archive" }
                Button { variant: ButtonVariant::Outline, "Report" }
            }
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, "Snooze" }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Icon,
                    aria_label: "More Options",
                    IconEllipsis {}
                }
            }
        }
    }
}
