//! Defines the [`Checkbox`] component and its subcomponents, which manage checkbox inputs with controlled state.

use crate::{use_controlled, use_unique_id};
use dioxus::{document::eval, prelude::*};
use std::ops::Not;
use tailwind_fuse::*;

/// The state of a [`Checkbox`] component.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckboxState {
    /// The checkbox is checked.
    Checked,
    /// The checkbox is in an indeterminate state, meaning it is neither checked nor unchecked.
    Indeterminate,
    /// The checkbox is unchecked.
    Unchecked,
}

impl CheckboxState {
    fn to_aria_checked(self) -> &'static str {
        match self {
            CheckboxState::Checked => "true",
            CheckboxState::Indeterminate => "mixed",
            CheckboxState::Unchecked => "false",
        }
    }

    fn to_data_state(self) -> &'static str {
        match self {
            CheckboxState::Checked => "checked",
            CheckboxState::Indeterminate => "indeterminate",
            CheckboxState::Unchecked => "unchecked",
        }
    }
}

impl From<CheckboxState> for bool {
    fn from(value: CheckboxState) -> Self {
        !matches!(value, CheckboxState::Unchecked)
    }
}

impl Not for CheckboxState {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Unchecked => Self::Checked,
            _ => Self::Unchecked,
        }
    }
}

#[derive(Clone, Copy)]
struct CheckboxCtx {
    checked: Memo<CheckboxState>,
    disabled: ReadSignal<bool>,
}

/// The props for the [`Checkbox`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    /// The controlled state of the checkbox.
    pub checked: ReadSignal<Option<CheckboxState>>,

    /// The default state of the checkbox when it is not controlled.
    #[props(default = CheckboxState::Unchecked)]
    pub default_checked: CheckboxState,

    /// Whether the checkbox is required in a form.
    #[props(default)]
    pub required: ReadSignal<bool>,

    /// Whether the checkbox is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The name of the checkbox, used in forms.
    #[props(default)]
    pub name: ReadSignal<String>,

    /// The value of the checkbox, which can be used in forms.
    #[props(default = ReadSignal::new(Signal::new(String::from("on"))))]
    pub value: ReadSignal<String>,

    /// Callback that is called when the checked state changes.
    #[props(default)]
    pub on_checked_change: Callback<CheckboxState>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply to the checkbox element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the checkbox.
    pub children: Element,
}

/// # Checkbox
///
/// The `Checkbox` component is a controlled checkbox input that allows users to toggle a state. It can be used in forms or standalone.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Checkbox {
///             name: "tos-check",
///             aria_label: "Demo Checkbox",
///             CheckboxIndicator {
///                 "✅"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Checkbox`] component defines the following data attributes you can use to control styling:
/// - `data-state`: The state of the checkbox. Possible values are `checked`, `indeterminate`, or `unchecked`.
/// - `data-disabled`: Indicates if the checkbox is disabled. values are `true` or `false`.
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let (checked, set_checked) = use_controlled(
        props.checked,
        props.default_checked,
        props.on_checked_change,
    );

    let class = tw_merge!(
        "peer size-4 shrink-0 rounded-[4px] border border-input shadow-xs transition-shadow outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:border-primary data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground dark:bg-input/30",
        props.class,
    );

    use_context_provider(|| CheckboxCtx {
        checked,
        disabled: props.disabled,
    });

    rsx! {
        button {
            type: "button",
            value: props.value,
            role: "checkbox",
            aria_checked: checked().to_aria_checked(),
            aria_required: props.required,
            disabled: props.disabled,
            "data-slot": "checkbox",
            "data-state": checked().to_data_state(),
            "data-disabled": props.disabled,
            class: class,

            onclick: move |_| {
                let new_checked = !checked();
                set_checked.call(new_checked);
            },

            // Aria says only spacebar can change state of checkboxes.
            onkeydown: move |e| {
                if e.key() == Key::Enter {
                    e.prevent_default();
                }
            },

            ..props.attributes,
            {props.children}
        }
        BubbleInput {
            checked: checked,
            default_checked: props.default_checked,

            required: props.required,
            name: props.name,
            value: props.value,
            disabled: props.disabled,
        }
    }
}

/// # CheckboxIndicator
///
/// The indicator for the [`Checkbox`] component, which visually represents the checkbox state. The
/// children will only be rendered when the checkbox is checked.
///
/// This must be used inside a [`Checkbox`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Checkbox {
///             name: "tos-check",
///             aria_label: "Demo Checkbox",
///             CheckboxIndicator {
///                 "✅"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`CheckboxIndicator`] component defines the following data attributes you can use to control styling:
/// - `data-state`: The state of the checkbox. Possible values are `checked`, `indeterminate`, or `unchecked`.
/// - `data-disabled`: Indicates if the checkbox is disabled. values are `true` or `false`.
#[component]
pub fn CheckboxIndicator(
    #[props(default)] class: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx: CheckboxCtx = use_context();
    let checked = (ctx.checked)();

    let class = tw_merge!(
        "grid place-content-center text-current transition-none",
        class,
    );

    rsx! {
        span {
            "data-slot": "checkbox-indicator",
            "data-state": checked.to_data_state(),
            "data-disabled": ctx.disabled,
            class: class,
            ..attributes,

            if checked.into() {
                {children}
            }
        }
    }
}

#[component]
fn BubbleInput(
    checked: ReadSignal<CheckboxState>,
    default_checked: CheckboxState,
    #[props(extends = input)] attributes: Vec<Attribute>,
) -> Element {
    let id = use_unique_id();

    // Update the actual input state to match our virtual state.
    use_effect(move || {
        let checked = checked();
        let js = eval(
            r#"
            let id = await dioxus.recv();
            let action = await dioxus.recv();
            let input = document.getElementById(id);

            switch(action) {
                case "checked":
                    input.checked = true;
                    input.indeterminate = false;
                    break;
                case "indeterminate":
                    input.indeterminate = true;
                    input.checked = true;
                    break;
                case "unchecked":
                    input.checked = false;
                    input.indeterminate = false;
                    break;
            }
            "#,
        );

        let _ = js.send(id());
        let _ = js.send(checked.to_data_state());
    });

    rsx! {
        input {
            id,
            type: "checkbox",
            aria_hidden: "true",
            tabindex: "-1",
            position: "absolute",
            pointer_events: "none",
            opacity: "0",
            margin: "0",
            transform: "translateX(-100%)",

            // Default checked
            checked: default_checked != CheckboxState::Unchecked,

            ..attributes,
        }
    }
}
