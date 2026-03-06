use dioxus::prelude::*;

use crate::components;
use crate::Route;

/// Prev/Next navigation matching shadcn's pagination buttons.
#[component]
pub(crate) fn PrevNextNav(current_name: &'static str) -> Element {
    let mut sorted: Vec<_> = components::DEMOS.iter().collect();
    sorted.sort_by_key(|d| d.name);

    let current_idx = sorted.iter().position(|d| d.name == current_name);
    let Some(idx) = current_idx else {
        return rsx! {};
    };

    let prev = if idx > 0 { Some(sorted[idx - 1]) } else { None };
    let next = if idx + 1 < sorted.len() {
        Some(sorted[idx + 1])
    } else {
        None
    };

    rsx! {
        div { class: "flex h-16 w-full items-center gap-2 mt-8",
            if let Some(prev) = prev {
                Link {
                    to: Route::component(prev.name),
                    class: "inline-flex items-center gap-1.5 rounded-md border border-border bg-secondary px-3 py-1.5 text-sm font-medium text-secondary-foreground no-underline hover:bg-accent transition-colors",
                    "\u{2190} "
                    span { class: "capitalize", {prev.name.replace("_", " ")} }
                }
            }
            div { class: "flex-1" }
            if let Some(next) = next {
                Link {
                    to: Route::component(next.name),
                    class: "inline-flex items-center gap-1.5 rounded-md border border-border bg-secondary px-3 py-1.5 text-sm font-medium text-secondary-foreground no-underline hover:bg-accent transition-colors",
                    span { class: "capitalize", {next.name.replace("_", " ")} }
                    " \u{2192}"
                }
            }
        }
    }
}
