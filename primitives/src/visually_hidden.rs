//! Visually hidden content — matches `@radix-ui/react-visually-hidden`.
//!
//! Renders a `<span>` that is invisible to sighted users but accessible to
//! screen readers. Uses Bootstrap's visually-hidden CSS technique.

use dioxus::prelude::*;

/// CSS styles that hide content visually while keeping it accessible.
///
/// Matches Radix's `VISUALLY_HIDDEN_STYLES`.
const VISUALLY_HIDDEN_STYLE: &str = "\
    position: absolute; \
    border: 0; \
    width: 1px; \
    height: 1px; \
    padding: 0; \
    margin: -1px; \
    overflow: hidden; \
    clip: rect(0, 0, 0, 0); \
    white-space: nowrap; \
    word-wrap: normal";

/// Props for [`VisuallyHidden`].
#[derive(Props, Clone, PartialEq)]
pub struct VisuallyHiddenProps {
    /// Additional inline styles appended after the visually-hidden base styles.
    #[props(default)]
    pub style: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (screen-reader content).
    pub children: Element,
}

/// Hides content visually while keeping it accessible to screen readers.
///
/// Matches Radix's `VisuallyHidden` component.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::visually_hidden::VisuallyHidden;
/// rsx! {
///     VisuallyHidden { "Only screen readers see this" }
/// };
/// ```
#[component]
pub fn VisuallyHidden(props: VisuallyHiddenProps) -> Element {
    let style = match &props.style {
        Some(extra) => format!("{VISUALLY_HIDDEN_STYLE}; {extra}"),
        None => VISUALLY_HIDDEN_STYLE.to_string(),
    };

    rsx! {
        span {
            "data-slot": "visually-hidden",
            style: "{style}",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}
