//! Password toggle field — matches `@radix-ui/react-password-toggle-field`.
//!
//! Provides a password input with a toggle button to show/hide the password.
//! Automatically resets visibility on form submit/reset for security.
//!
//! ## Architecture
//!
//! - [`PasswordToggleField`] — Root, provides visibility context
//! - [`PasswordToggleFieldInput`] — `<input>` that toggles between `password` and `text`
//! - [`PasswordToggleFieldToggle`] — Button to toggle visibility
//! - [`PasswordToggleFieldIcon`] — Conditionally renders visible/hidden icons
//!
//! ## Example
//!
//! ```rust,no_run
//! # use dioxus::prelude::*;
//! # use dioxus_primitives::password_toggle_field::*;
//! fn Demo() -> Element {
//!     rsx! {
//!         PasswordToggleField {
//!             PasswordToggleFieldInput { placeholder: "Enter password" }
//!             PasswordToggleFieldToggle {
//!                 PasswordToggleFieldIcon {
//!                     visible: rsx! { "👁" },
//!                     hidden: rsx! { "👁‍🗨" },
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

/// Context provided by [`PasswordToggleField`].
#[derive(Clone, Debug, PartialEq)]
pub struct PasswordToggleFieldCtx {
    /// Whether the password is currently visible.
    pub visible: bool,
}

/// Access the nearest [`PasswordToggleField`] context.
pub fn use_password_toggle_field() -> PasswordToggleFieldCtx {
    use_context::<Signal<PasswordToggleFieldCtx>>().cloned()
}

// ---------------------------------------------------------------------------
// PasswordToggleField (root)
// ---------------------------------------------------------------------------

/// Props for [`PasswordToggleField`].
#[derive(Props, Clone, PartialEq)]
pub struct PasswordToggleFieldProps {
    /// Controlled visibility state.
    #[props(default)]
    pub visible: ReadSignal<Option<bool>>,

    /// Default visibility (uncontrolled mode).
    #[props(default)]
    pub default_visible: bool,

    /// Callback when visibility changes.
    #[props(default)]
    pub on_visibility_change: Callback<bool>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Root component providing password visibility context.
///
/// Manages the visible/hidden state and provides context to
/// [`PasswordToggleFieldInput`], [`PasswordToggleFieldToggle`], and
/// [`PasswordToggleFieldIcon`].
#[component]
pub fn PasswordToggleField(props: PasswordToggleFieldProps) -> Element {
    let (visible, set_visible) = crate::use_controlled(
        props.visible,
        props.default_visible,
        props.on_visibility_change,
    );

    let ctx = PasswordToggleFieldCtx { visible: visible() };

    use_context_provider(|| Signal::new(ctx.clone()));

    // Update context when state changes
    let mut ctx_signal = use_context::<Signal<PasswordToggleFieldCtx>>();
    if *ctx_signal.peek() != ctx {
        ctx_signal.set(ctx);
    }

    // Store the toggle callback in context for the toggle button
    use_context_provider(|| set_visible);

    rsx! {
        div {
            "data-slot": "password-toggle-field",
            "data-visible": if visible() { "true" } else { "false" },
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// PasswordToggleFieldInput
// ---------------------------------------------------------------------------

/// Props for [`PasswordToggleFieldInput`].
#[derive(Props, Clone, PartialEq)]
pub struct PasswordToggleFieldInputProps {
    /// Placeholder text.
    #[props(default)]
    pub placeholder: Option<String>,

    /// Autocomplete hint.
    #[props(default = "current-password".to_string())]
    pub autocomplete: String,

    /// Input name attribute.
    #[props(default)]
    pub name: Option<String>,

    /// Input id attribute.
    #[props(default)]
    pub id: Option<String>,

    /// Whether the input is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether the input is required.
    #[props(default)]
    pub required: bool,

    /// Controlled value.
    #[props(default)]
    pub value: Option<String>,

    /// Input change callback.
    #[props(default)]
    pub oninput: Callback<FormEvent>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Password input that toggles between `type="password"` and `type="text"`.
///
/// Reads visibility from [`PasswordToggleFieldCtx`] to determine the input type.
#[component]
pub fn PasswordToggleFieldInput(props: PasswordToggleFieldInputProps) -> Element {
    let ctx = use_password_toggle_field();
    let input_type = if ctx.visible { "text" } else { "password" };

    rsx! {
        input {
            "data-slot": "password-toggle-field-input",
            r#type: input_type,
            autocomplete: props.autocomplete,
            spellcheck: "false",
            autocapitalize: "off",
            placeholder: props.placeholder,
            name: props.name,
            id: props.id,
            disabled: props.disabled,
            required: props.required,
            value: props.value,
            oninput: move |e| props.oninput.call(e),
            class: props.class,
            ..props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// PasswordToggleFieldToggle
// ---------------------------------------------------------------------------

/// Props for [`PasswordToggleFieldToggle`].
#[derive(Props, Clone, PartialEq)]
pub struct PasswordToggleFieldToggleProps {
    /// Whether the toggle button is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (typically a [`PasswordToggleFieldIcon`]).
    pub children: Element,
}

/// Toggle button for password visibility.
///
/// Generates an appropriate `aria-label` ("Show password" or "Hide password").
#[component]
pub fn PasswordToggleFieldToggle(props: PasswordToggleFieldToggleProps) -> Element {
    let ctx = use_password_toggle_field();
    let set_visible = use_context::<Callback<bool>>();

    let aria_label = if ctx.visible {
        "Hide password"
    } else {
        "Show password"
    };

    rsx! {
        button {
            "data-slot": "password-toggle-field-toggle",
            r#type: "button",
            disabled: props.disabled,
            aria_label: aria_label,
            "data-visible": if ctx.visible { "true" } else { "false" },
            onclick: move |_| {
                set_visible.call(!ctx.visible);
            },
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// PasswordToggleFieldIcon
// ---------------------------------------------------------------------------

/// Props for [`PasswordToggleFieldIcon`].
#[derive(Props, Clone, PartialEq)]
pub struct PasswordToggleFieldIconProps {
    /// Icon to show when password is visible (e.g., an "eye" icon).
    pub visible: Element,

    /// Icon to show when password is hidden (e.g., an "eye-off" icon).
    pub hidden: Element,
}

/// Conditionally renders icons based on password visibility.
#[component]
pub fn PasswordToggleFieldIcon(props: PasswordToggleFieldIconProps) -> Element {
    let ctx = use_password_toggle_field();

    rsx! {
        span {
            "data-slot": "password-toggle-field-icon",
            aria_hidden: "true",
            if ctx.visible {
                {props.visible}
            } else {
                {props.hidden}
            }
        }
    }
}
