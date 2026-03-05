//! Defines the [`Button`] component with Tailwind-based styling and variants.

use dioxus::prelude::*;
use tailwind_fuse::*;

/// Visual variant of the [`Button`] component.
#[derive(Debug, PartialEq, TwVariant)]
pub enum ButtonVariant {
    /// Default filled button with primary color.
    #[tw(
        default,
        class = "bg-primary text-primary-foreground hover:bg-primary/90"
    )]
    Default,

    /// Destructive action button (e.g. delete).
    #[tw(
        class = "bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:bg-destructive/60 dark:focus-visible:ring-destructive/40"
    )]
    Destructive,

    /// Bordered button with transparent background.
    #[tw(
        class = "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:border-input dark:bg-input/30 dark:hover:bg-input/50"
    )]
    Outline,

    /// Muted button with secondary color.
    #[tw(class = "bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,

    /// Borderless transparent button, visible on hover.
    #[tw(class = "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50")]
    Ghost,

    /// Styled as a hyperlink.
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
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
#[derive(Debug, PartialEq, TwVariant)]
pub enum ButtonSize {
    /// Default size.
    #[tw(default, class = "h-9 px-4 py-2 has-[>svg]:px-3")]
    Default,

    /// Small button.
    #[tw(class = "h-8 gap-1.5 rounded-md px-3 has-[>svg]:px-2.5")]
    Sm,

    /// Large button.
    #[tw(class = "h-10 rounded-md px-6 has-[>svg]:px-4")]
    Lg,

    /// Square icon-only button.
    #[tw(class = "size-9")]
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
    #[props(default = ButtonVariant::Default)]
    pub variant: ButtonVariant,

    /// Size of the button.
    #[props(default = ButtonSize::Default)]
    pub size: ButtonSize,

    /// Whether the button is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Click event handler.
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// Additional Tailwind classes to apply. Conflicts with base/variant classes
    /// are resolved in favor of this override.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply to the button element.
    #[props(extends = button, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the button component.
    pub children: Element,
}

/// # Button
///
/// A button component with variant and size support, styled with Tailwind utility classes.
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
///
///         Button { class: "w-full", "Full Width" }
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

    let class = tw_merge!(
        "inline-flex shrink-0 items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-all cursor-pointer outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        props.variant,
        props.size,
        props.class,
    );

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
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}
