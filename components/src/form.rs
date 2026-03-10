//! Styled form matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::form` primitive with
//! Tailwind classes — matching the shadcn/ui form component.

use dioxus::prelude::*;
use dioxus_primitives::form as primitives;
use tailwind_fuse::*;

// Re-export context accessor and types
pub use primitives::{FormFieldCtx, use_form_field};

// ---------------------------------------------------------------------------
// Form (root)
// ---------------------------------------------------------------------------

/// The props for the styled [`Form`] component.
#[derive(Props, Clone, PartialEq)]
pub struct FormProps {
    /// Callback fired when the form is submitted.
    #[props(default)]
    pub on_submit: Callback<FormEvent>,

    /// Callback fired when the form is reset.
    #[props(default)]
    pub on_reset: Callback<FormEvent>,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled Form root — matches shadcn.
#[component]
pub fn Form(props: FormProps) -> Element {
    rsx! {
        primitives::Form {
            on_submit: props.on_submit,
            on_reset: props.on_reset,
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FormField
// ---------------------------------------------------------------------------

/// The props for the styled [`FormField`] component.
#[derive(Props, Clone, PartialEq)]
pub struct FormFieldProps {
    /// The field name.
    pub name: String,

    /// Current error message for this field.
    #[props(default)]
    pub error: Option<String>,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled FormField — matches shadcn.
#[component]
pub fn FormField(props: FormFieldProps) -> Element {
    rsx! {
        primitives::FormField {
            name: props.name,
            error: props.error,
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FormItem
// ---------------------------------------------------------------------------

/// The props for the styled [`FormItem`] component.
#[derive(Props, Clone, PartialEq)]
pub struct FormItemProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled FormItem — matches shadcn `grid gap-2`.
#[component]
pub fn FormItem(props: FormItemProps) -> Element {
    let class = tw_merge!("grid gap-2", props.class);

    rsx! {
        primitives::FormItem {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FormLabel
// ---------------------------------------------------------------------------

/// The props for the styled [`FormLabel`] component.
#[derive(Props, Clone, PartialEq)]
pub struct FormLabelProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled FormLabel — matches shadcn with error color.
#[component]
pub fn FormLabel(props: FormLabelProps) -> Element {
    let class = tw_merge!("data-[error=true]:text-destructive", props.class);

    rsx! {
        primitives::FormLabel {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FormControl
// ---------------------------------------------------------------------------

/// The props for the styled [`FormControl`] component.
#[derive(Props, Clone, PartialEq)]
pub struct FormControlProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled FormControl — passes through (no shadcn-specific classes).
#[component]
pub fn FormControl(props: FormControlProps) -> Element {
    rsx! {
        primitives::FormControl {
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FormDescription
// ---------------------------------------------------------------------------

/// The props for the styled [`FormDescription`] component.
#[derive(Props, Clone, PartialEq)]
pub struct FormDescriptionProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled FormDescription — matches shadcn `text-sm text-muted-foreground`.
#[component]
pub fn FormDescription(props: FormDescriptionProps) -> Element {
    let class = tw_merge!("text-sm text-muted-foreground", props.class);

    rsx! {
        primitives::FormDescription {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FormMessage
// ---------------------------------------------------------------------------

/// The props for the styled [`FormMessage`] component.
#[derive(Props, Clone, PartialEq)]
pub struct FormMessageProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Custom children override the error message.
    #[props(default)]
    pub children: Element,
}

/// Styled FormMessage — matches shadcn `text-sm text-destructive`.
#[component]
pub fn FormMessage(props: FormMessageProps) -> Element {
    let class = tw_merge!("text-sm text-destructive", props.class);
    let has_children = props.children != Ok(VNode::placeholder());

    if has_children {
        rsx! {
            primitives::FormMessage {
                class: class,
                attributes: props.attributes,
                {props.children}
            }
        }
    } else {
        rsx! {
            primitives::FormMessage {
                class: class,
                attributes: props.attributes,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// FormSubmit
// ---------------------------------------------------------------------------

/// The props for the styled [`FormSubmit`] component.
#[derive(Props, Clone, PartialEq)]
pub struct FormSubmitProps {
    /// Whether the submit button is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled FormSubmit — thin wrapper.
#[component]
pub fn FormSubmit(props: FormSubmitProps) -> Element {
    rsx! {
        primitives::FormSubmit {
            disabled: props.disabled,
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
