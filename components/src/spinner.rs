//! Styled spinner matching shadcn/ui.
//!
//! Pure HTML + Tailwind component using the Lucide Loader2 icon.
//! No primitive dependency.

use dioxus::prelude::*;
use tailwind_fuse::*;

/// The props for the styled [`Spinner`] component.
#[derive(Props, Clone, PartialEq)]
pub struct SpinnerProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled Spinner — matches shadcn exactly.
///
/// Renders a spinning loader SVG (Lucide Loader2 icon) with `role="status"`
/// and `aria-label="Loading"` for accessibility.
#[component]
pub fn Spinner(props: SpinnerProps) -> Element {
    let class = tw_merge!("size-4 animate-spin", props.class);

    rsx! {
        svg {
            role: "status",
            "aria-label": "Loading",
            "data-slot": "spinner",
            class: class,
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            ..props.attributes,
            // Lucide loader-circle (Loader2) path
            path { d: "M21 12a9 9 0 1 1-6.219-8.56" }
        }
    }
}
