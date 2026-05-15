//! Styled button matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::button` primitive with
//! Tailwind classes for variant and size styling — matching the
//! shadcn/ui button component.

use dioxus::prelude::*;
use dioxus_primitives::button as primitives;
pub use dioxus_primitives::button::{ButtonSize, ButtonVariant};
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Button (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`Button`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Visual variant of the button.
    #[props(default)]
    pub variant: ButtonVariant,

    /// Size of the button.
    #[props(default)]
    pub size: ButtonSize,

    /// Whether the button is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Click event handler.
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the button element.
    #[props(extends = button, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the button.
    pub children: Element,
}

/// Styled Button — matches shadcn radix-flavor exactly:
/// - Variant: default, outline, secondary, ghost, destructive, link
/// - Size: default, xs, sm, lg, icon, icon-xs, icon-sm, icon-lg
#[component]
pub fn Button(props: ButtonProps) -> Element {
    // Mirrors shadcn radix-flavor `examples/radix/ui/button.tsx` 1:1.
    let variant_class = match props.variant {
        ButtonVariant::Default => "bg-primary text-primary-foreground [a]:hover:bg-primary/80",
        ButtonVariant::Outline => {
            "border-border bg-background hover:bg-muted hover:text-foreground aria-expanded:bg-muted aria-expanded:text-foreground dark:border-input dark:bg-input/30 dark:hover:bg-input/50"
        }
        ButtonVariant::Secondary => {
            "bg-secondary text-secondary-foreground hover:bg-secondary/80 aria-expanded:bg-secondary aria-expanded:text-secondary-foreground"
        }
        ButtonVariant::Ghost => {
            "hover:bg-muted hover:text-foreground aria-expanded:bg-muted aria-expanded:text-foreground dark:hover:bg-muted/50"
        }
        ButtonVariant::Destructive => {
            "bg-destructive/10 text-destructive hover:bg-destructive/20 focus-visible:border-destructive/40 focus-visible:ring-destructive/20 dark:bg-destructive/20 dark:hover:bg-destructive/30 dark:focus-visible:ring-destructive/40"
        }
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
    };

    let size_class = match props.size {
        ButtonSize::Default => {
            "h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
        }
        ButtonSize::Xs => {
            "h-6 gap-1 rounded-[min(var(--radius-md),10px)] px-2 text-xs in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3"
        }
        ButtonSize::Sm => {
            "h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3.5"
        }
        ButtonSize::Lg => {
            "h-9 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-3 has-data-[icon=inline-start]:pl-3"
        }
        ButtonSize::Icon => "size-8",
        ButtonSize::IconXs => {
            "size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&_svg:not([class*='size-'])]:size-3"
        }
        ButtonSize::IconSm => {
            "size-7 rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg"
        }
        ButtonSize::IconLg => "size-9",
    };

    let class = tw_merge!(
        "group/button inline-flex shrink-0 items-center justify-center rounded-lg border border-transparent bg-clip-padding text-sm font-medium whitespace-nowrap transition-all outline-none select-none focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 active:not-aria-[haspopup]:translate-y-px disabled:pointer-events-none disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        variant_class,
        size_class,
        props.class,
    );

    rsx! {
        primitives::Button {
            variant: props.variant,
            size: props.size,
            disabled: props.disabled,
            onclick: props.onclick,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
