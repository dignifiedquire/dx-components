//! Arrow primitive — matches `@radix-ui/react-arrow`.
//!
//! Renders an SVG triangle for use with floating elements (popover, tooltip,
//! dropdown, etc.). The arrow points downward by default.

use dioxus::prelude::*;

/// Props for [`Arrow`].
#[derive(Props, Clone, PartialEq)]
pub struct ArrowProps {
    /// Width of the arrow in pixels. Defaults to 10.
    #[props(default = 10.0)]
    pub width: f64,

    /// Height of the arrow in pixels. Defaults to 5.
    #[props(default = 5.0)]
    pub height: f64,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes applied to the `<svg>` element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Optional children to replace the default polygon.
    #[props(default)]
    pub children: Element,
}

/// SVG arrow triangle for floating elements.
///
/// Matches Radix's `Arrow` component. Renders a downward-pointing triangle
/// inside a `<svg>` with `viewBox="0 0 30 10"` and `preserveAspectRatio="none"`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::arrow::Arrow;
/// rsx! {
///     Arrow { width: 12.0, height: 6.0 }
/// };
/// ```
#[component]
pub fn Arrow(props: ArrowProps) -> Element {
    let has_children = props.children.is_ok();

    rsx! {
        svg {
            "data-slot": "arrow",
            width: "{props.width}",
            height: "{props.height}",
            view_box: "0 0 30 10",
            "preserveAspectRatio": "none",
            class: props.class,
            ..props.attributes,

            if has_children {
                {props.children}
            } else {
                polygon { points: "0,0 30,0 15,10" }
            }
        }
    }
}
