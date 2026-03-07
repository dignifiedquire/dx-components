//! Toggle primitive — matches `@radix-ui/react-toggle`.
//!
//! A two-state button that can be toggled on or off.

use crate::use_controlled;
use dioxus::prelude::*;

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
