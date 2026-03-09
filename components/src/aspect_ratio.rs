//! Styled aspect ratio matching shadcn/ui.
//!
//! Thin passthrough to the unstyled `dioxus_primitives::aspect_ratio` primitive.
//! shadcn adds only a `data-slot` attribute (which the primitive already provides).

use dioxus::prelude::*;
use dioxus_primitives::aspect_ratio as primitives;

// ---------------------------------------------------------------------------
// AspectRatio (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`AspectRatio`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AspectRatioProps {
    /// Aspect ratio as width / height (e.g. `16.0 / 9.0`).
    #[props(default = 1.0)]
    pub ratio: f64,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the aspect ratio container.
    pub children: Element,
}

/// Styled AspectRatio — thin passthrough matching shadcn.
#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
    rsx! {
        primitives::AspectRatio {
            ratio: props.ratio,
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
