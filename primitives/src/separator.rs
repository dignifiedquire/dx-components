//! Separator primitive — matches `@radix-ui/react-separator`.
//!
//! Renders a visual or semantic divider. When `decorative` is false (default),
//! it renders with `role="separator"` for assistive technologies.

use crate::direction::Orientation;
use dioxus::prelude::*;

/// Props for [`Separator`].
#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    /// Orientation of the separator. Defaults to `Horizontal`.
    #[props(default = Orientation::Horizontal)]
    pub orientation: Orientation,

    /// Whether the separator is purely decorative. When `true`, it is removed
    /// from the accessibility tree (`role="none"`). Defaults to `false`.
    #[props(default)]
    pub decorative: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A visual or semantic divider between content sections.
///
/// Matches Radix's `Separator`. Sets `role="separator"` by default, or
/// `role="none"` when `decorative` is true.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::separator::Separator;
/// # use dioxus_primitives::direction::Orientation;
/// rsx! {
///     "Above"
///     Separator {}
///     "Below"
///
///     Separator { orientation: Orientation::Vertical }
/// };
/// ```
#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let orientation = props.orientation;

    // `aria-orientation` defaults to `horizontal` per WAI-ARIA, so we only
    // set it explicitly when vertical.
    let aria_orientation = match orientation {
        Orientation::Vertical => Some("vertical"),
        Orientation::Horizontal => None,
    };

    rsx! {
        div {
            "data-slot": "separator",
            "data-orientation": orientation.as_str(),
            role: if props.decorative { "none" } else { "separator" },
            aria_orientation: if !props.decorative { aria_orientation },
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}
