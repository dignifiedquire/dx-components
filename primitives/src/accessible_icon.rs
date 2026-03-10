//! Accessible icon — matches `@radix-ui/react-accessible-icon`.
//!
//! Wraps an icon element (e.g. SVG) and adds `aria-hidden="true"` to it,
//! plus a [`VisuallyHidden`] label for screen readers.
//!
//! In Radix React this clones `aria-hidden` onto the child via `cloneElement`.
//! In Dioxus we wrap the children in a `<span aria-hidden="true">` and add the
//! visually-hidden label as a sibling — achieving the same accessibility result.

use crate::visually_hidden::VisuallyHidden;
use dioxus::prelude::*;

/// Props for [`AccessibleIcon`].
#[derive(Props, Clone, PartialEq)]
pub struct AccessibleIconProps {
    /// The accessible label announced to screen readers.
    /// Similar to `alt` text on `<img>` elements.
    pub label: String,

    /// The icon element (typically an SVG). It will be wrapped in a
    /// `<span aria-hidden="true">` so sighted users see it but screen
    /// readers skip it.
    pub children: Element,
}

/// Makes an icon accessible by hiding it from screen readers and providing
/// a visually-hidden text label instead.
///
/// Matches `@radix-ui/react-accessible-icon`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::accessible_icon::AccessibleIcon;
/// rsx! {
///     AccessibleIcon { label: "Close",
///         svg {
///             view_box: "0 0 24 24",
///             width: "24",
///             height: "24",
///             path { d: "M18 6L6 18M6 6l12 12" }
///         }
///     }
/// };
/// ```
#[component]
pub fn AccessibleIcon(props: AccessibleIconProps) -> Element {
    rsx! {
        span {
            "data-slot": "accessible-icon",
            // The icon itself is decorative — screen readers should skip it.
            // Radix also sets `focusable="false"` on the SVG child, but in Dioxus
            // we can't modify children props. `aria-hidden="true"` is sufficient.
            span {
                aria_hidden: "true",
                style: "display: inline-flex;",
                {props.children}
            }
            // The label is visible only to screen readers.
            VisuallyHidden { {props.label} }
        }
    }
}
