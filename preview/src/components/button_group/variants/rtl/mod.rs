use super::super::component::*;
use dioxus::prelude::*;
use dx_icons_lucide::{IconArrowLeft, IconEllipsis};

// Mirrors shadcn's button-group-rtl layout under `dir="rtl"`. Upstream
// wires a live language switcher (Arabic/Hebrew dictionaries via site-only
// `@/components/language-selector`) which is app infra, not part of the
// component — so labels are static here (documented deviation). The
// dropdown is deferred with the `dropdown` follow-up (see `main`).
#[component]
pub fn Demo() -> Element {
    rsx! {
        div { dir: "rtl",
            ButtonGroup {
                ButtonGroup { class: "hidden sm:flex",
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Icon,
                        aria_label: "Go Back",
                        IconArrowLeft { class: "rtl:rotate-180" }
                    }
                }
                ButtonGroup {
                    Button { variant: ButtonVariant::Outline, "أرشفة" }
                    Button { variant: ButtonVariant::Outline, "تقرير" }
                }
                ButtonGroup {
                    Button { variant: ButtonVariant::Outline, "تأجيل" }
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
}
