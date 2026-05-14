//! Accessible icon — matches `@radix-ui/react-accessible-icon`.
//!
//! Wraps an icon element (e.g. an `svg`) and renders a visually-hidden
//! label alongside it so screen readers announce a meaningful name.
//!
//! ## Divergence from upstream
//!
//! Radix injects `aria-hidden="true"` and `focusable="false"` directly
//! onto the SVG child via `React.cloneElement`. Dioxus VNodes are
//! immutable, so we instead wrap the icon in a `<span aria-hidden="true">`.
//! This adds one extra DOM node compared to Radix, but the screen-reader
//! behaviour is identical: the icon graphic is hidden from assistive
//! technology and the visually-hidden label is announced.
//!
//! `focusable="false"` is omitted because SVG elements are not focusable
//! by default in modern browsers — the attribute is an IE-era workaround.

use crate::visually_hidden::VisuallyHidden;
use dioxus::prelude::*;

/// Props for [`AccessibleIcon`].
#[derive(Props, Clone, PartialEq)]
pub struct AccessibleIconProps {
    /// The accessible label announced to screen readers. Similar to
    /// `alt` text on `<img>` elements.
    pub label: String,

    /// The icon element (typically an `svg`). Wrapped in a
    /// `<span aria-hidden="true">` so sighted users see it but screen
    /// readers skip it. See module-level docs for why this wrapper is
    /// needed.
    pub children: Element,
}

/// Makes an icon accessible by hiding it from screen readers and
/// announcing a visually-hidden label instead.
///
/// Matches `@radix-ui/react-accessible-icon`. Returns a fragment of two
/// sibling elements (icon wrapper + visually-hidden label) — no outer
/// wrapper element.
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
            aria_hidden: "true",
            style: "display: inline-flex;",
            {props.children}
        }
        VisuallyHidden { {props.label} }
    }
}

/// Upstream alias.
///
/// `const Root = AccessibleIcon;`
pub use AccessibleIcon as Root;
