//! Focus guards — matches `@radix-ui/react-focus-guards`.
//!
//! Renders a pair of invisible sentinel `<span>` elements that catch
//! `focusin`/`focusout` events consistently. These are placed at the edges
//! of the component subtree to ensure focus trapping and scope management
//! work reliably across browsers.

use dioxus::prelude::*;

/// Props for [`FocusGuards`].
#[derive(Props, Clone, PartialEq)]
pub struct FocusGuardsProps {
    /// Children wrapped by the focus guards.
    pub children: Element,
}

/// Renders focus guard sentinel spans around its children.
///
/// Matches Radix's `FocusGuards` component. Places invisible, focusable
/// `<span>` elements before and after the children to reliably catch
/// focus events at the boundaries.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::focus_guards::FocusGuards;
/// rsx! {
///     FocusGuards {
///         // Content that needs focus boundary guards
///         div { "Focusable content" }
///     }
/// };
/// ```
#[component]
pub fn FocusGuards(props: FocusGuardsProps) -> Element {
    rsx! {
        FocusGuard {}
        {props.children}
        FocusGuard {}
    }
}

/// A single invisible focus sentinel span.
#[component]
fn FocusGuard() -> Element {
    rsx! {
        span {
            "data-radix-focus-guard": "",
            tabindex: "0",
            style: "outline: none; opacity: 0; position: fixed; pointer-events: none;",
        }
    }
}
