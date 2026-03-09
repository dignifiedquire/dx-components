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

/// Styled Button — matches shadcn exactly:
/// - Base: flex layout, focus-visible ring, disabled opacity
/// - Variant: primary, destructive, outline, secondary, ghost, link
/// - Size: default, sm, lg, icon
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let variant_class = match props.variant {
        ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
        ButtonVariant::Destructive => {
            "bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:bg-destructive/60 dark:focus-visible:ring-destructive/40"
        }
        ButtonVariant::Outline => {
            "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:border-input dark:bg-input/30 dark:hover:bg-input/50"
        }
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ButtonVariant::Ghost => {
            "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50"
        }
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
    };

    let size_class = match props.size {
        ButtonSize::Default => "h-9 px-4 py-2 has-[>svg]:px-3",
        ButtonSize::Sm => "h-8 gap-1.5 rounded-md px-3 has-[>svg]:px-2.5",
        ButtonSize::Lg => "h-10 rounded-md px-6 has-[>svg]:px-4",
        ButtonSize::Icon => "size-9",
    };

    let class = tw_merge!(
        "inline-flex shrink-0 items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-all cursor-pointer outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
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
