//! AspectRatio primitive — matches `@radix-ui/react-aspect-ratio`.
//!
//! Maintains a specified aspect ratio for its children using the
//! padding-bottom technique.

use dioxus::prelude::*;

/// Props for [`AspectRatio`].
#[derive(Props, Clone, PartialEq)]
pub struct AspectRatioProps {
    /// The desired aspect ratio (width / height). Defaults to `1.0` (square).
    #[props(default = 1.0)]
    pub ratio: f64,

    /// Additional CSS classes applied to the inner content element.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes applied to the inner content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children rendered inside the aspect-ratio container.
    pub children: Element,
}

/// A container that maintains a specific aspect ratio for its children.
///
/// Matches Radix's `AspectRatio`. Uses the padding-bottom trick to enforce
/// the ratio, with children absolutely positioned to fill the space.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::aspect_ratio::AspectRatio;
/// rsx! {
///     AspectRatio { ratio: 16.0 / 9.0,
///         img { src: "photo.jpg", style: "width: 100%; height: 100%; object-fit: cover;" }
///     }
/// };
/// ```
#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
    let padding_bottom = 100.0 / props.ratio;

    rsx! {
        div {
            "data-slot": "aspect-ratio",
            "data-radix-aspect-ratio-wrapper": "",
            style: "position: relative; width: 100%; padding-bottom: {padding_bottom}%;",
            div {
                style: "position: absolute; top: 0; right: 0; bottom: 0; left: 0;",
                class: props.class,
                ..props.attributes,
                {props.children}
            }
        }
    }
}
