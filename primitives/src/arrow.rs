//! Arrow primitive — matches `@radix-ui/react-arrow`.
//!
//! Renders an SVG triangle for use with floating elements (popover, tooltip,
//! dropdown, etc.). The arrow points downward by default.
//!
//! ## Dioxus Limitation
//!
//! Upstream supports custom children to replace the default triangle path.
//! Dioxus currently has a rendering bug where SVG child elements inside
//! conditional blocks (or interpolated from a standalone `rsx!` call) produce
//! `<!--placeholder-->` nodes instead of actual SVG elements. As a workaround,
//! the default triangle is rendered via `dangerous_inner_html`. Custom children
//! are not supported; use CSS `fill`, `stroke`, etc. to style the arrow.

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
    rsx! {
        svg {
            "data-slot": "arrow",
            width: "{props.width}",
            height: "{props.height}",
            view_box: "0 0 30 10",
            "preserveAspectRatio": "none",
            dangerous_inner_html: r#"<path d="M0,0 L30,0 L15,10 Z"></path>"#,
            class: props.class,
            ..props.attributes,
        }
    }
}

/// Upstream alias.
///
/// `const Root = Arrow;`
pub use Arrow as Root;
