//! Defines the [`Separator`] component for creating visual or semantic separators.

use dioxus::prelude::*;
use tailwind_fuse::*;

/// The props for the [`Separator`] component.
#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    /// Horizontal if true, vertical if false.
    #[props(default = true)]
    pub horizontal: bool,

    /// If the separator is decorative and should not be classified
    /// as a separator to the ARIA standard.
    #[props(default = true)]
    pub decorative: bool,

    /// Additional Tailwind classes to apply. Conflicts with base classes
    /// are resolved in favor of this override.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply to the separator element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the separator component.
    pub children: Element,
}

/// # Separator
///
/// The `Separator` component creates a visual or semantic divider between sections of content. If the divider
/// is purely decorative, it can be marked as such to avoid being classified as a separator by assistive technologies.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::separator::Separator;
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         "One thing"
///         Separator { horizontal: true, decorative: true }
///         "Another thing"
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Separator`] component defines the following data attributes for external styling:
/// - `data-slot`: Always `"separator"`.
/// - `data-orientation`: Indicates the orientation. Values are `horizontal` or `vertical`.
#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let orientation = match props.horizontal {
        true => "horizontal",
        false => "vertical",
    };

    let class = tw_merge!(
        "shrink-0 bg-border data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "separator",
            role: if !props.decorative { "separator" } else { "none" },
            aria_orientation: if !props.decorative { orientation },
            "data-orientation": orientation,
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}
