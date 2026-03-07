//! Toggle primitive — matches `@radix-ui/react-toggle`.
//!
//! A two-state button that can be toggled on or off.

use crate::use_controlled;
use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Backward-compat types (used by toggle_group.rs, will be removed when
// toggle_group is rewritten in Phase 2)
// ---------------------------------------------------------------------------

/// Visual variant of a toggle — kept for backward compat with ToggleGroup.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ToggleVariant {
    /// Default variant.
    #[default]
    Default,
    /// Outline variant.
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

/// Size variant of a toggle — kept for backward compat with ToggleGroup.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ToggleSize {
    /// Default size.
    #[default]
    Default,
    /// Small.
    Sm,
    /// Large.
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

/// Props for [`Toggle`].
#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    /// The controlled pressed state.
    #[props(default)]
    pub pressed: ReadSignal<Option<bool>>,

    /// The default pressed state when uncontrolled.
    #[props(default)]
    pub default_pressed: bool,

    /// Whether the toggle is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Callback fired when the pressed state changes.
    #[props(default)]
    pub on_pressed_change: Callback<bool>,

    // Backward-compat props (used by toggle_group.rs, removed when it's rewritten)
    #[allow(missing_docs)]
    #[props(default)]
    pub variant: ToggleVariant,
    #[allow(missing_docs)]
    #[props(default)]
    pub size: ToggleSize,
    #[allow(missing_docs)]
    #[props(default)]
    pub onmounted: Callback<Event<MountedData>>,
    #[allow(missing_docs)]
    #[props(default)]
    pub onfocus: Callback<Event<FocusData>>,
    #[allow(missing_docs)]
    #[props(default)]
    pub onkeydown: Callback<Event<KeyboardData>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A two-state toggle button.
///
/// Matches Radix's `Toggle`. Renders a `<button>` with `aria-pressed` and
/// `data-state` (on/off).
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::toggle::Toggle;
/// rsx! {
///     Toggle { "Bold" }
/// };
/// ```
#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let (pressed, set_pressed) = use_controlled(
        props.pressed,
        props.default_pressed,
        props.on_pressed_change,
    );

    let disabled = props.disabled;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "toggle",
            "data-state": if pressed() { "on" } else { "off" },
            "data-disabled": if disabled { "" },
            aria_pressed: pressed,
            disabled: disabled,
            class: props.class,

            onmounted: props.onmounted,
            onfocus: props.onfocus,
            onkeydown: props.onkeydown,

            onclick: move |_| {
                if !disabled {
                    set_pressed.call(!pressed());
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}
