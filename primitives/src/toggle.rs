//! Defines the [`Toggle`] component for creating toggle buttons with Tailwind-based styling.

use crate::use_controlled;
use dioxus::prelude::*;
use tailwind_fuse::*;

/// Visual variant of the [`Toggle`] component.
#[derive(Debug, PartialEq, TwVariant)]
pub enum ToggleVariant {
    /// Transparent background toggle.
    #[tw(default, class = "bg-transparent")]
    Default,

    /// Bordered toggle with shadow.
    #[tw(
        class = "border border-input bg-transparent shadow-xs hover:bg-accent hover:text-accent-foreground"
    )]
    Outline,
}

impl ToggleVariant {
    /// Returns the value for the `data-variant` attribute.
    pub fn as_data_attr(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Outline => "outline",
        }
    }
}

/// Size variant for the [`Toggle`] component.
#[derive(Debug, PartialEq, TwVariant)]
pub enum ToggleSize {
    /// Default size.
    #[tw(default, class = "h-9 min-w-9 px-2")]
    Default,

    /// Small toggle.
    #[tw(class = "h-8 min-w-8 px-1.5")]
    Sm,

    /// Large toggle.
    #[tw(class = "h-10 min-w-10 px-2.5")]
    Lg,
}

impl ToggleSize {
    /// Returns the value for the `data-size` attribute.
    pub fn as_data_attr(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
            Self::Lg => "lg",
        }
    }
}

/// The props for the [`Toggle`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    /// Visual variant of the toggle.
    #[props(default = ToggleVariant::Default)]
    pub variant: ToggleVariant,

    /// Size of the toggle.
    #[props(default = ToggleSize::Default)]
    pub size: ToggleSize,

    /// The controlled pressed state of the toggle.
    pub pressed: ReadSignal<Option<bool>>,

    /// The default pressed state when uncontrolled.
    #[props(default)]
    pub default_pressed: bool,

    /// Whether the toggle is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Callback fired when the pressed state changes.
    #[props(default)]
    pub on_pressed_change: Callback<bool>,

    // https://github.com/DioxusLabs/dioxus/issues/2467
    /// Callback fired when the toggle is mounted.
    #[props(default)]
    pub onmounted: Callback<Event<MountedData>>,
    /// Callback fired when the toggle receives focus.
    #[props(default)]
    pub onfocus: Callback<Event<FocusData>>,
    /// Callback fired when a key is pressed on the toggle.
    #[props(default)]
    pub onkeydown: Callback<Event<KeyboardData>>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply to the toggle element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the toggle component.
    pub children: Element,
}

/// # Toggle
///
/// The `Toggle` component is a button that can be on or off.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::toggle::{Toggle, ToggleVariant, ToggleSize};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Toggle { em { "B" } }
///
///         Toggle { variant: ToggleVariant::Outline, "Outline" }
///
///         Toggle { size: ToggleSize::Sm, "Small" }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Toggle`] component defines the following data attributes for external styling:
/// - `data-slot`: Always `"toggle"`.
/// - `data-variant`: The current variant (e.g. `"default"`, `"outline"`).
/// - `data-size`: The current size (e.g. `"default"`, `"sm"`, `"lg"`).
/// - `data-state`: Indicates the state of the toggle. Values are `on` or `off`.
/// - `data-disabled`: Indicates if the toggle is disabled.
#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let (pressed, set_pressed) = use_controlled(
        props.pressed,
        props.default_pressed,
        props.on_pressed_change,
    );

    let variant_attr = props.variant.as_data_attr();
    let size_attr = props.size.as_data_attr();

    let class = tw_merge!(
        "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-[color,box-shadow] outline-none hover:bg-muted hover:text-muted-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 data-[state=on]:bg-accent data-[state=on]:text-accent-foreground [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        props.variant,
        props.size,
        props.class,
    );

    rsx! {
        button {
            onmounted: props.onmounted,
            onfocus: props.onfocus,
            onkeydown: props.onkeydown,

            r#type: "button",
            disabled: props.disabled,
            aria_pressed: pressed,
            "data-slot": "toggle",
            "data-variant": variant_attr,
            "data-size": size_attr,
            "data-state": if pressed() { "on" } else { "off" },
            "data-disabled": props.disabled,
            class: class,

            onclick: move |_| {
                let new_pressed = !pressed();
                set_pressed.call(new_pressed);
            },

            ..props.attributes,
            {props.children}
        }
    }
}
