//! Dioxus equivalent of Radix's `Slot` / `asChild` pattern.
//!
//! In Radix, the `Slot` component merges a parent component's props onto a child element,
//! allowing the child to control the rendered element type while inheriting behavior (ARIA,
//! data attributes, event handlers) from the parent.
//!
//! In Dioxus, this is achieved via the `r#as` prop pattern:
//!
//! ```ignore
//! #[derive(Props, Clone, PartialEq)]
//! pub struct MyComponentProps {
//!     /// Render as a custom element (Radix `asChild` equivalent).
//!     #[props(default)]
//!     pub r#as: Option<Callback<Vec<Attribute>, Element>>,
//!
//!     #[props(extends = GlobalAttributes)]
//!     pub attributes: Vec<Attribute>,
//!
//!     pub children: Element,
//! }
//! ```
//!
//! The component builds its internal attributes, merges them with the user's attributes
//! via [`merge_attributes`], then either passes them to the `r#as` callback or renders
//! a default element.
//!
//! ## Merge rules (matching Radix's `mergeProps`)
//!
//! [`merge_attributes`] handles the merge with these rules:
//! - `class`: concatenated with spaces (both preserved)
//! - `style`: concatenated with `"; "` (both preserved)
//! - All other attributes: later list wins (user overrides component defaults)
//!
//! ## Event handlers
//!
//! Event handlers in Dioxus are separate from `extends = GlobalAttributes` (see
//! [dioxus#2467](https://github.com/DioxusLabs/dioxus/issues/2467)). Components that
//! need to expose event handlers to callers should declare them as explicit props
//! (e.g., `pub onkeydown: Callback<Event<KeyboardData>>`) and forward them onto the
//! element in the non-`r#as` path.
//!
//! When `r#as` is used, internal event handlers set via the `attributes!` macro
//! (like `onclick`) are included in the merged attribute list and will be applied
//! when the caller spreads `..attrs` onto their element.
//!
//! ## Usage
//!
//! ```ignore
//! use crate::{merge_attributes, slot::render_slot};
//! use dioxus_attributes::attributes;
//!
//! #[component]
//! fn MyButton(props: MyButtonProps) -> Element {
//!     let base = attributes!(button {
//!         "data-slot": "my-button",
//!         onclick: move |_| { /* internal handler */ },
//!     });
//!     let merged = merge_attributes(vec![base, props.attributes]);
//!
//!     render_slot(props.r#as, merged, props.children, |attrs, children| {
//!         rsx! { button { ..attrs, {children} } }
//!     })
//! }
//!
//! // Caller using default element:
//! rsx! { MyButton { "Click me" } }
//!
//! // Caller using custom element (asChild):
//! rsx! {
//!     MyButton {
//!         r#as: move |attrs| rsx! { a { href: "/home", ..attrs, "Go home" } },
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Renders either a default element or delegates to an `r#as` callback.
///
/// This is the Dioxus equivalent of Radix's `asChild` / `Slot` dispatch.
///
/// - If `r#as` is `Some`, calls the callback with the merged attributes.
///   The callback is responsible for rendering an element and spreading the attributes.
/// - If `r#as` is `None`, calls `default_render` with the attributes and children.
#[allow(dead_code)] // Will be used as components adopt the standardized pattern
pub(crate) fn render_slot(
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    attrs: Vec<Attribute>,
    children: Element,
    default_render: impl FnOnce(Vec<Attribute>, Element) -> Element,
) -> Element {
    if let Some(dynamic) = r#as {
        dynamic.call(attrs)
    } else {
        default_render(attrs, children)
    }
}
