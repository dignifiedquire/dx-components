//! Styled form field components matching shadcn/ui.
//!
//! Pure HTML + Tailwind — no Radix primitive needed.
//! Provides semantic form layout with label, description, and error support.

use dioxus::prelude::*;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// FieldOrientation
// ---------------------------------------------------------------------------

/// Layout orientation for [`Field`].
#[derive(Default, Clone, Copy, PartialEq)]
pub enum FieldOrientation {
    #[default]
    Vertical,
    Horizontal,
    Responsive,
}

// ---------------------------------------------------------------------------
// FieldLegendVariant
// ---------------------------------------------------------------------------

/// Visual variant for [`FieldLegend`].
#[derive(Default, Clone, Copy, PartialEq)]
pub enum FieldLegendVariant {
    #[default]
    Legend,
    Label,
}

// ---------------------------------------------------------------------------
// FieldSet
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct FieldSetProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn FieldSet(props: FieldSetProps) -> Element {
    let class = tw_merge!(
        "flex flex-col gap-6 has-[>[data-slot=checkbox-group]]:gap-3 has-[>[data-slot=radio-group]]:gap-3",
        props.class,
    );

    rsx! {
        fieldset {
            "data-slot": "field-set",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FieldLegend
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct FieldLegendProps {
    /// Visual variant: legend (larger) or label (smaller).
    #[props(default)]
    pub variant: FieldLegendVariant,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn FieldLegend(props: FieldLegendProps) -> Element {
    let variant_attr = match props.variant {
        FieldLegendVariant::Legend => "legend",
        FieldLegendVariant::Label => "label",
    };

    let class = tw_merge!(
        "mb-3 font-medium data-[variant=legend]:text-base data-[variant=label]:text-sm",
        props.class,
    );

    rsx! {
        legend {
            "data-slot": "field-legend",
            "data-variant": variant_attr,
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FieldGroup
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct FieldGroupProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn FieldGroup(props: FieldGroupProps) -> Element {
    let class = tw_merge!(
        "group/field-group flex w-full flex-col gap-7 data-[slot=checkbox-group]:gap-3 [&>[data-slot=field-group]]:gap-4",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "field-group",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// Field
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct FieldProps {
    /// Layout orientation.
    #[props(default)]
    pub orientation: FieldOrientation,

    /// Whether the field is invalid.
    #[props(default)]
    pub invalid: bool,

    /// Whether the field is disabled.
    #[props(default)]
    pub disabled: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn Field(props: FieldProps) -> Element {
    let orientation_class = match props.orientation {
        FieldOrientation::Vertical => "flex-col [&>*]:w-full [&>.sr-only]:w-auto",
        FieldOrientation::Horizontal => {
            "flex-row items-center [&>[data-slot=field-label]]:flex-auto has-[>[data-slot=field-content]]:items-start"
        }
        FieldOrientation::Responsive => "flex-col [&>*]:w-full [&>.sr-only]:w-auto",
    };

    let class = tw_merge!(
        "group/field flex w-full gap-3 data-[invalid=true]:text-destructive",
        orientation_class,
        props.class,
    );

    rsx! {
        div {
            "data-slot": "field",
            "data-invalid": if props.invalid { Some("true") } else { None },
            "data-disabled": if props.disabled { Some("true") } else { None },
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FieldContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct FieldContentProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn FieldContent(props: FieldContentProps) -> Element {
    let class = tw_merge!(
        "group/field-content flex flex-1 flex-col gap-1.5 leading-snug",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "field-content",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FieldLabel
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct FieldLabelProps {
    /// The `for` attribute linking to a form element.
    #[props(default)]
    pub html_for: Option<String>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn FieldLabel(props: FieldLabelProps) -> Element {
    let class = tw_merge!(
        "group/field-label peer/field-label flex w-fit gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[>[data-slot=field]]:w-full has-[>[data-slot=field]]:flex-col has-[>[data-slot=field]]:rounded-md has-[>[data-slot=field]]:border has-data-[state=checked]:border-primary has-data-[state=checked]:bg-primary/5 dark:has-data-[state=checked]:bg-primary/10",
        props.class,
    );

    rsx! {
        label {
            "data-slot": "field-label",
            r#for: props.html_for,
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FieldTitle
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct FieldTitleProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn FieldTitle(props: FieldTitleProps) -> Element {
    let class = tw_merge!(
        "flex w-fit items-center gap-2 text-sm leading-snug font-medium group-data-[disabled=true]/field:opacity-50",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "field-label",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FieldDescription
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct FieldDescriptionProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn FieldDescription(props: FieldDescriptionProps) -> Element {
    let class = tw_merge!(
        "text-sm leading-normal font-normal text-muted-foreground [&>a]:underline [&>a]:underline-offset-4 [&>a:hover]:text-primary",
        props.class,
    );

    rsx! {
        p {
            "data-slot": "field-description",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FieldError
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct FieldErrorProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn FieldError(props: FieldErrorProps) -> Element {
    let class = tw_merge!("text-sm font-normal text-destructive", props.class);

    rsx! {
        div {
            "data-slot": "field-error",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// FieldSeparator
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct FieldSeparatorProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Optional text to display in the separator.
    #[props(default)]
    pub text: Option<String>,
}

#[component]
pub fn FieldSeparator(props: FieldSeparatorProps) -> Element {
    let class = tw_merge!("relative -my-2 h-5 text-sm", props.class);

    rsx! {
        div {
            "data-slot": "field-separator",
            class: class,
            ..props.attributes,
            div {
                class: "shrink-0 bg-border h-px w-full absolute top-1/2 -translate-y-1/2",
                role: "none",
            }
            if let Some(text) = &props.text {
                span {
                    "data-slot": "field-separator-content",
                    class: "relative mx-auto block w-fit bg-background px-2 text-muted-foreground",
                    "{text}"
                }
            }
        }
    }
}
