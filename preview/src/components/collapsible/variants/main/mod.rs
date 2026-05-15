use dioxus::prelude::*;
use dioxus_components::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use dx_icons_lucide::IconChevronsUpDown;

// shadcn `Button variant="ghost" size="icon"` + the demo's `size-8`
// override, flattened to a class string — our `CollapsibleTrigger` is
// itself the `<button>` (no `asChild`), so we can't nest a `Button`.
const TRIGGER_BUTTON_CLASS: &str = "inline-flex shrink-0 items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-all cursor-pointer outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50 size-8";

#[component]
pub fn Demo() -> Element {
    rsx! {
        Collapsible { class: "flex w-[350px] flex-col gap-2",
            div { class: "flex items-center justify-between gap-4 px-4",
                // shadcn uses <h4> here purely for `text-sm font-semibold`
                // styling; the isolated preview has no h1–h3 ancestor, so a
                // standalone heading trips axe `heading-order`. A span keeps
                // the exact visual without the invalid heading.
                span { class: "text-sm font-semibold", "@peduarte starred 3 repositories" }
                CollapsibleTrigger { class: TRIGGER_BUTTON_CLASS,
                    IconChevronsUpDown {}
                    span { class: "sr-only", "Toggle" }
                }
            }
            div { class: "rounded-md border px-4 py-2 font-mono text-sm",
                "@radix-ui/primitives"
            }
            CollapsibleContent { class: "flex flex-col gap-2",
                div { class: "rounded-md border px-4 py-2 font-mono text-sm",
                    "@radix-ui/colors"
                }
                div { class: "rounded-md border px-4 py-2 font-mono text-sm",
                    "@stitches/react"
                }
            }
        }
    }
}
