//! Form primitive with validation context.
//!
//! Provides structural components for building accessible forms with
//! field-level validation, error messages, and proper ARIA wiring.
//!
//! This is a Dioxus-native implementation inspired by Radix's `@radix-ui/react-form`
//! and shadcn/ui's `form.tsx`. Instead of wrapping `react-hook-form`, it provides
//! a lightweight context-based validation system where the consumer drives
//! validation logic via signals.
//!
//! ## Architecture
//!
//! - [`Form`] — `<form>` root, prevents default browser validation UI
//! - [`FormField`] — Context provider for a named field (links label, control, messages)
//! - [`FormItem`] — Layout container for a field's sub-components
//! - [`FormLabel`] — Label auto-wired to the field's control via `for`
//! - [`FormControl`] — Passes `aria-describedby` and `aria-invalid` to child
//! - [`FormDescription`] — Description text linked via `aria-describedby`
//! - [`FormMessage`] — Error message linked via `aria-describedby`
//! - [`FormSubmit`] — Submit button
//!
//! ## Example
//!
//! ```rust,no_run
//! # use dioxus::prelude::*;
//! # use dioxus_primitives::form::*;
//! fn Demo() -> Element {
//!     let mut email_error = use_signal(|| None::<String>);
//!
//!     rsx! {
//!         Form {
//!             on_submit: move |_| {
//!                 // validate and set errors
//!             },
//!             FormField { name: "email", error: email_error(),
//!                 FormItem {
//!                     FormLabel { "Email" }
//!                     FormControl {
//!                         input { r#type: "email", placeholder: "you@example.com" }
//!                     }
//!                     FormDescription { "We'll never share your email." }
//!                     FormMessage {}
//!                 }
//!             }
//!             FormSubmit { "Submit" }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// FormField context — shared between Label, Control, Description, Message
// ---------------------------------------------------------------------------

/// Context provided by [`FormField`] to its descendants.
#[derive(Clone, Debug)]
pub struct FormFieldCtx {
    /// The field name (used for form data submission).
    pub name: String,
    /// Auto-generated id for the control element.
    pub control_id: String,
    /// Id for the description element (for `aria-describedby`).
    pub description_id: String,
    /// Id for the message element (for `aria-describedby`).
    pub message_id: String,
    /// Current error message, if any.
    pub error: Option<String>,
}

/// Access the nearest [`FormField`] context.
///
/// Panics if called outside a `FormField`.
pub fn use_form_field() -> FormFieldCtx {
    use_context::<Signal<FormFieldCtx>>().cloned()
}

// ---------------------------------------------------------------------------
// Form (root)
// ---------------------------------------------------------------------------

/// Props for [`Form`].
#[derive(Props, Clone, PartialEq)]
pub struct FormProps {
    /// Callback fired when the form is submitted.
    /// The native submit event is always prevented.
    #[props(default)]
    pub on_submit: Callback<FormEvent>,

    /// Callback fired when the form is reset.
    #[props(default)]
    pub on_reset: Callback<FormEvent>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Form root — wraps `<form>` and prevents default browser validation UI.
///
/// Uses `novalidate` to disable native validation bubbles. Validation
/// is handled via [`FormField`]'s `error` prop and rendered in [`FormMessage`].
#[component]
pub fn Form(props: FormProps) -> Element {
    rsx! {
        form {
            "data-slot": "form",
            novalidate: true,
            class: props.class,
            onsubmit: move |e| {
                e.prevent_default();
                props.on_submit.call(e);
            },
            onreset: move |e| {
                props.on_reset.call(e);
            },
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FormField
// ---------------------------------------------------------------------------

/// Props for [`FormField`].
#[derive(Props, Clone, PartialEq)]
pub struct FormFieldProps {
    /// The field name (used for form data and validation tracking).
    pub name: String,

    /// Current error message for this field. `None` means valid.
    #[props(default)]
    pub error: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Provides per-field context (name, ids, error state) to descendants.
///
/// All [`FormLabel`], [`FormControl`], [`FormDescription`], and [`FormMessage`]
/// inside this field automatically wire up ARIA attributes.
#[component]
pub fn FormField(props: FormFieldProps) -> Element {
    let id = crate::use_unique_id();

    let ctx = FormFieldCtx {
        name: props.name.clone(),
        control_id: format!("{}-control", id()),
        description_id: format!("{}-description", id()),
        message_id: format!("{}-message", id()),
        error: props.error.clone(),
    };

    let has_error = ctx.error.is_some();

    use_context_provider(|| Signal::new(ctx));

    rsx! {
        div {
            "data-slot": "form-field",
            "data-valid": if !has_error { "true" },
            "data-invalid": if has_error { "true" },
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FormItem
// ---------------------------------------------------------------------------

/// Props for [`FormItem`].
#[derive(Props, Clone, PartialEq)]
pub struct FormItemProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Layout container for a form field's sub-components.
#[component]
pub fn FormItem(props: FormItemProps) -> Element {
    rsx! {
        div {
            "data-slot": "form-item",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FormLabel
// ---------------------------------------------------------------------------

/// Props for [`FormLabel`].
#[derive(Props, Clone, PartialEq)]
pub struct FormLabelProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Label auto-wired to the field's control via `for` attribute.
///
/// Sets `data-error="true"` when the field has a validation error.
#[component]
pub fn FormLabel(props: FormLabelProps) -> Element {
    let ctx = use_form_field();
    let has_error = ctx.error.is_some();

    rsx! {
        label {
            "data-slot": "form-label",
            "data-error": has_error.then_some("true"),
            r#for: ctx.control_id,
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FormControl
// ---------------------------------------------------------------------------

/// Props for [`FormControl`].
#[derive(Props, Clone, PartialEq)]
pub struct FormControlProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (typically an input, select, or textarea).
    pub children: Element,
}

/// Wrapper that passes ARIA attributes to its child control.
///
/// Sets `id`, `aria-describedby` (linking description + message), and
/// `aria-invalid` on the wrapper. The consumer's input inside should
/// inherit these via the wrapper or use the context directly.
#[component]
pub fn FormControl(props: FormControlProps) -> Element {
    let ctx = use_form_field();
    let has_error = ctx.error.is_some();

    let aria_describedby = if has_error {
        format!("{} {}", ctx.description_id, ctx.message_id)
    } else {
        ctx.description_id.clone()
    };

    rsx! {
        div {
            "data-slot": "form-control",
            id: ctx.control_id,
            aria_describedby: aria_describedby,
            aria_invalid: has_error,
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FormDescription
// ---------------------------------------------------------------------------

/// Props for [`FormDescription`].
#[derive(Props, Clone, PartialEq)]
pub struct FormDescriptionProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Description text linked to the field's control via `aria-describedby`.
#[component]
pub fn FormDescription(props: FormDescriptionProps) -> Element {
    let ctx = use_form_field();

    rsx! {
        p {
            "data-slot": "form-description",
            id: ctx.description_id,
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FormMessage
// ---------------------------------------------------------------------------

/// Props for [`FormMessage`].
#[derive(Props, Clone, PartialEq)]
pub struct FormMessageProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Custom children override the error message.
    #[props(default)]
    pub children: Element,
}

/// Displays the field's validation error message.
///
/// If the field has no error and no children, nothing is rendered.
/// The error message is linked to the control via `aria-describedby`.
#[component]
pub fn FormMessage(props: FormMessageProps) -> Element {
    let ctx = use_form_field();

    // If children are provided, always show them. Otherwise show the error.
    let has_children = props.children != Ok(VNode::placeholder());
    let body = if has_children {
        props.children
    } else if let Some(ref error) = ctx.error {
        rsx! { "{error}" }
    } else {
        return rsx! {};
    };

    rsx! {
        p {
            "data-slot": "form-message",
            id: ctx.message_id,
            role: "alert",
            class: props.class,
            ..props.attributes,
            {body}
        }
    }
}

// ---------------------------------------------------------------------------
// FormSubmit
// ---------------------------------------------------------------------------

/// Props for [`FormSubmit`].
#[derive(Props, Clone, PartialEq)]
pub struct FormSubmitProps {
    /// Whether the submit button is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Submit button for the form.
#[component]
pub fn FormSubmit(props: FormSubmitProps) -> Element {
    rsx! {
        button {
            "data-slot": "form-submit",
            r#type: "submit",
            disabled: props.disabled,
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}
