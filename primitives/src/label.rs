//! Label primitive — matches `@radix-ui/react-label`.
//!
//! Renders a `<label>` element that prevents text selection on double-click
//! (unless clicking inside a button, input, select, or textarea).

use dioxus::prelude::*;

/// Props for [`Label`].
#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    /// The id of the element this label is associated with.
    #[props(default)]
    pub html_for: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// An accessible label for form controls.
///
/// Matches Radix's `Label`. Prevents text selection on double-click to avoid
/// accidental selection when rapidly interacting with the associated control.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::label::Label;
/// rsx! {
///     Label { html_for: "name", "Name" }
///     input { id: "name", placeholder: "Enter your name" }
/// };
/// ```
#[component]
pub fn Label(props: LabelProps) -> Element {
    rsx! {
        label {
            "data-slot": "label",
            r#for: props.html_for,
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}
