//! Defines the [`Label`] component with Tailwind-based styling.

use dioxus::prelude::*;
use tailwind_fuse::*;

/// The props for the [`Label`] component.
#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    /// The id of the element that this label is associated with.
    pub html_for: ReadSignal<String>,

    /// Additional Tailwind classes to apply. Conflicts with base classes
    /// are resolved in favor of this override.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply to the label element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the label element.
    pub children: Element,
}

/// # Label
///
/// The `Label` component is used to create a label for form elements. It must be associated with an element using the [`LabelProps::html_for`] attribute.
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::label::Label;
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Label {
///             html_for: "name",
///             "Name"
///         }
///
///         input {
///             id: "name",
///             placeholder: "Enter your name",
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Label`] component defines the following data attributes for external styling:
/// - `data-slot`: Always `"label"`.
#[component]
pub fn Label(props: LabelProps) -> Element {
    let class = tw_merge!(
        "flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50",
        props.class,
    );

    rsx! {
        label {
            "data-slot": "label",
            r#for: props.html_for,
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}
