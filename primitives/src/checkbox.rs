//! Checkbox primitive — matches `@radix-ui/react-checkbox`.
//!
//! A tri-state checkbox control (checked/unchecked/indeterminate).

use crate::use_controlled;
use dioxus::prelude::*;
use std::ops::Not;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// The state of a checkbox.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckedState {
    /// The checkbox is checked.
    Checked,
    /// The checkbox is in an indeterminate state.
    Indeterminate,
    /// The checkbox is unchecked.
    Unchecked,
}

impl CheckedState {
    /// Returns the ARIA `aria-checked` value.
    pub fn to_aria_checked(self) -> &'static str {
        match self {
            Self::Checked => "true",
            Self::Indeterminate => "mixed",
            Self::Unchecked => "false",
        }
    }

    /// Returns the `data-state` value.
    pub fn to_data_state(self) -> &'static str {
        match self {
            Self::Checked => "checked",
            Self::Indeterminate => "indeterminate",
            Self::Unchecked => "unchecked",
        }
    }

    /// Returns `true` if checked or indeterminate.
    pub fn is_checked(self) -> bool {
        !matches!(self, Self::Unchecked)
    }
}

impl From<CheckedState> for bool {
    fn from(value: CheckedState) -> Self {
        value.is_checked()
    }
}

impl Not for CheckedState {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Unchecked => Self::Checked,
            _ => Self::Unchecked,
        }
    }
}

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct CheckboxCtx {
    checked: Memo<CheckedState>,
    disabled: bool,
}

// ---------------------------------------------------------------------------
// Checkbox
// ---------------------------------------------------------------------------

/// Props for [`Checkbox`].
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    /// The controlled checked state.
    #[props(default)]
    pub checked: ReadSignal<Option<CheckedState>>,

    /// The default checked state when uncontrolled.
    #[props(default = CheckedState::Unchecked)]
    pub default_checked: CheckedState,

    /// Whether the checkbox is required in a form.
    #[props(default)]
    pub required: bool,

    /// Whether the checkbox is disabled.
    #[props(default)]
    pub disabled: bool,

    /// The name for form submission.
    #[props(default)]
    pub name: Option<String>,

    /// The value for form submission. Defaults to `"on"`.
    #[props(default = "on".to_string())]
    pub value: String,

    /// Callback fired when the checked state changes.
    #[props(default)]
    pub on_checked_change: Callback<CheckedState>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (typically a [`CheckboxIndicator`]).
    pub children: Element,
}

/// A tri-state checkbox control.
///
/// Matches Radix's `Checkbox`. Renders a `<button>` with `role="checkbox"`,
/// `aria-checked` (true/false/mixed), and `data-state`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator};
/// rsx! {
///     Checkbox {
///         CheckboxIndicator { "✓" }
///     }
/// };
/// ```
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let (checked, set_checked) = use_controlled(
        props.checked,
        props.default_checked,
        props.on_checked_change,
    );

    let disabled = props.disabled;

    use_context_provider(|| CheckboxCtx { checked, disabled });

    rsx! {
        button {
            r#type: "button",
            role: "checkbox",
            "data-slot": "checkbox",
            "data-state": checked().to_data_state(),
            "data-disabled": if disabled { "" },
            aria_checked: checked().to_aria_checked(),
            aria_required: if props.required { "true" },
            disabled: disabled,
            value: props.value.clone(),
            class: props.class,

            onclick: move |_| {
                set_checked.call(!checked());
            },

            // Checkboxes don't activate on Enter (WAI-ARIA)
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
            checked: checked().is_checked(),
            disabled: disabled,
            required: props.required,
            style: "position: absolute; pointer-events: none; opacity: 0; margin: 0; transform: translateX(-100%);",
        }
    }
}

// ---------------------------------------------------------------------------
// CheckboxIndicator
// ---------------------------------------------------------------------------

/// Props for [`CheckboxIndicator`].
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxIndicatorProps {
    /// When `true`, the indicator is always mounted regardless of checked state.
    #[props(default)]
    pub force_mount: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (typically a check icon or text).
    pub children: Element,
}

/// Visual indicator for a [`Checkbox`].
///
/// Only renders children when the checkbox is checked or indeterminate
/// (unless `force_mount` is true). Must be inside a [`Checkbox`].
#[component]
pub fn CheckboxIndicator(props: CheckboxIndicatorProps) -> Element {
    let ctx: CheckboxCtx = use_context();
    let checked = (ctx.checked)();
    let should_render = props.force_mount || checked.is_checked();

    rsx! {
        span {
            "data-slot": "checkbox-indicator",
            "data-state": checked.to_data_state(),
            "data-disabled": if ctx.disabled { "" },
            style: "pointer-events: none;",
            class: props.class,
            ..props.attributes,

            if should_render {
                {props.children}
            }
        }
    }
}
