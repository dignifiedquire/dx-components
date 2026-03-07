//! Switch primitive — matches `@radix-ui/react-switch`.
//!
//! A two-state toggle control. Renders a `<button>` with `role="switch"`.

use crate::use_controlled;
use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct SwitchCtx {
    checked: Memo<bool>,
    disabled: bool,
}

// ---------------------------------------------------------------------------
// Switch
// ---------------------------------------------------------------------------

/// Props for [`Switch`].
#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    /// The controlled checked state.
    #[props(default)]
    pub checked: ReadSignal<Option<bool>>,

    /// The default checked state when uncontrolled.
    #[props(default)]
    pub default_checked: bool,

    /// Whether the switch is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether the switch is required in a form.
    #[props(default)]
    pub required: bool,

    /// The name attribute for form submission.
    #[props(default)]
    pub name: Option<String>,

    /// The value attribute for form submission. Defaults to `"on"`.
    #[props(default = "on".to_string())]
    pub value: String,

    /// Callback fired when the checked state changes.
    #[props(default)]
    pub on_checked_change: Callback<bool>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (typically a [`SwitchThumb`]).
    pub children: Element,
}

/// A toggle switch control.
///
/// Matches Radix's `Switch`. Renders a `<button>` with `role="switch"`,
/// `aria-checked`, and `data-state` (checked/unchecked).
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::switch::{Switch, SwitchThumb};
/// rsx! {
///     Switch {
///         SwitchThumb {}
///     }
/// };
/// ```
#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let (checked, set_checked) = use_controlled(
        props.checked,
        props.default_checked,
        props.on_checked_change,
    );

    let disabled = props.disabled;

    use_context_provider(|| SwitchCtx { checked, disabled });

    rsx! {
        button {
            r#type: "button",
            role: "switch",
            "data-slot": "switch",
            "data-state": if checked() { "checked" } else { "unchecked" },
            "data-disabled": if disabled { "" },
            aria_checked: checked,
            aria_required: if props.required { "true" },
            disabled: disabled,
            value: props.value.clone(),
            class: props.class,

            onclick: move |_| {
                set_checked.call(!checked());
            },

            // Switches should only toggle on Space, not Enter
            onkeydown: move |e| {
                if e.key() == Key::Enter {
                    e.prevent_default();
                }
            },

            ..props.attributes,
            {props.children}
        }

        // Hidden input for form submission
        input {
            r#type: "checkbox",
            aria_hidden: true,
            tabindex: "-1",
            name: props.name.clone(),
            value: props.value.clone(),
            checked: checked,
            disabled: disabled,
            required: props.required,
            style: "position: absolute; pointer-events: none; opacity: 0; margin: 0; transform: translateX(-100%);",
        }
    }
}

// ---------------------------------------------------------------------------
// SwitchThumb
// ---------------------------------------------------------------------------

/// Props for [`SwitchThumb`].
#[derive(Props, Clone, PartialEq)]
pub struct SwitchThumbProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// The visual thumb indicator for a [`Switch`].
///
/// Must be used inside a [`Switch`] component. Inherits `data-state` and
/// `data-disabled` from the parent switch context.
#[component]
pub fn SwitchThumb(props: SwitchThumbProps) -> Element {
    let ctx: SwitchCtx = use_context();

    rsx! {
        span {
            "data-slot": "switch-thumb",
            "data-state": if (ctx.checked)() { "checked" } else { "unchecked" },
            "data-disabled": if ctx.disabled { "" },
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}
