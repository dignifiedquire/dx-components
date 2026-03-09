//! Styled separator matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::separator` primitive with
//! Tailwind classes — matching the shadcn/ui separator component 1:1.

use dioxus::prelude::*;
pub use dioxus_primitives::direction::Orientation;
use dioxus_primitives::separator as primitives;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Separator (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`Separator`] component.
#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    /// The orientation of the separator. Defaults to horizontal (matching shadcn).
    #[props(default = Orientation::Horizontal)]
    pub orientation: Orientation,

    /// Whether the separator is purely decorative (role="none" instead of "separator").
    #[props(default = true)]
    pub decorative: bool,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the separator element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the separator.
    #[props(default)]
    pub children: Element,
}

/// Styled Separator — matches shadcn exactly.
#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let class = tw_merge!(
        "shrink-0 bg-border data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px",
        props.class,
    );

    rsx! {
        primitives::Separator {
            orientation: props.orientation,
            decorative: props.decorative,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
