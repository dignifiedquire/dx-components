//! Styled toggle matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::toggle` primitive with
//! Tailwind classes — matching shadcn's toggle component 1:1.

use dioxus::prelude::*;
use dioxus_primitives::toggle as primitives;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Toggle variant/size enums (from shadcn toggle.tsx CVA)
// ---------------------------------------------------------------------------

/// Visual variant of a toggle — matches shadcn's `toggleVariants`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

/// Size of a toggle — matches shadcn's `toggleVariants`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ToggleSize {
    #[default]
    Default,
    Sm,
    Lg,
}

// ---------------------------------------------------------------------------
// Toggle (styled)
// ---------------------------------------------------------------------------

/// Base toggle classes (from shadcn toggle.tsx CVA).
const TOGGLE_BASE: &str = "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-[color,box-shadow] outline-none hover:bg-muted hover:text-muted-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 data-[state=on]:bg-accent data-[state=on]:text-accent-foreground dark:aria-invalid:ring-destructive/40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";

/// Props for the styled [`Toggle`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    /// The controlled pressed state.
    #[props(default)]
    pub pressed: ReadSignal<Option<bool>>,

    /// The default pressed state when uncontrolled.
    #[props(default)]
    pub default_pressed: bool,

    /// Whether the toggle is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Callback fired when the pressed state changes.
    #[props(default)]
    pub on_pressed_change: Callback<bool>,

    /// Visual variant.
    #[props(default)]
    pub variant: ToggleVariant,

    /// Size.
    #[props(default)]
    pub size: ToggleSize,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled Toggle — matches shadcn exactly with CVA variants.
#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let variant_class = match props.variant {
        ToggleVariant::Default => "bg-transparent",
        ToggleVariant::Outline => {
            "border border-input bg-transparent shadow-xs hover:bg-accent hover:text-accent-foreground"
        }
    };

    let size_class = match props.size {
        ToggleSize::Default => "h-9 min-w-9 px-2",
        ToggleSize::Sm => "h-8 min-w-8 px-1.5",
        ToggleSize::Lg => "h-10 min-w-10 px-2.5",
    };

    let class = tw_merge!(TOGGLE_BASE, variant_class, size_class, props.class,);

    rsx! {
        primitives::Toggle {
            pressed: props.pressed,
            default_pressed: props.default_pressed,
            disabled: props.disabled,
            on_pressed_change: props.on_pressed_change,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
