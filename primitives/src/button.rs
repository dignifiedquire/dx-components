//! Defines the [`Button`] component — an unstyled button primitive with variant
//! and size data attributes for external styling.

use dioxus::prelude::*;

/// Visual variant of the [`Button`] component.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    /// Default filled button with primary color.
    #[default]
    Default,
    /// Destructive action button (e.g. delete).
    Destructive,
    /// Bordered button with transparent background.
    Outline,
    /// Muted button with secondary color.
    Secondary,
    /// Borderless transparent button, visible on hover.
    Ghost,
    /// Styled as a hyperlink.
    Link,
}

impl ButtonVariant {
    /// Returns the value for the `data-variant` attribute.
    pub fn as_data_attr(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
            Self::Outline => "outline",
            Self::Secondary => "secondary",
            Self::Ghost => "ghost",
            Self::Link => "link",
        }
    }
}

/// Size variant for the [`Button`] component.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ButtonSize {
    /// Default size.
    #[default]
    Default,
    /// Small button.
    Sm,
    /// Large button.
    Lg,
    /// Square icon-only button.
    Icon,
}

impl ButtonSize {
    /// Returns the value for the `data-size` attribute.
    pub fn as_data_attr(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
            Self::Lg => "lg",
            Self::Icon => "icon",
        }
    }
}

/// The props for the [`Button`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Visual variant of the button.
    #[props(default)]
    pub variant: ButtonVariant,

    /// Size of the button.
    #[props(default)]
    pub size: ButtonSize,

    /// Whether the button is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Click event handler.
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// Additional attributes to apply to the button element.
    #[props(extends = button, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the button component.
    pub children: Element,
}

/// # Button
///
/// An unstyled button component with variant and size data attributes.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::button::{Button, ButtonVariant, ButtonSize};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Button { "Click me" }
///
///         Button { variant: ButtonVariant::Destructive, "Delete" }
///
///         Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Small Outline" }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Button`] component defines the following data attributes for external styling:
/// - `data-slot`: Always `"button"`.
/// - `data-variant`: The current variant (e.g. `"default"`, `"destructive"`, `"outline"`).
/// - `data-size`: The current size (e.g. `"default"`, `"sm"`, `"lg"`, `"icon"`).
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let variant_attr = props.variant.as_data_attr();
    let size_attr = props.size.as_data_attr();

    rsx! {
        button {
            r#type: "button",
            disabled: props.disabled,
            onclick: move |event| {
                if let Some(f) = &props.onclick {
                    f.call(event);
                }
            },
            "data-slot": "button",
            "data-variant": variant_attr,
            "data-size": size_attr,
            ..props.attributes,
            {props.children}
        }
    }
}
