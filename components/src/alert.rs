//! Styled alert matching shadcn/ui.
//!
//! Pure HTML + Tailwind component with variant support.
//! No primitive dependency — renders native HTML elements.

use dioxus::prelude::*;
use tailwind_fuse::*;

/// Visual variant of the alert.
#[derive(TwVariant, PartialEq)]
pub enum AlertVariant {
    #[tw(default, class = "bg-card text-card-foreground")]
    Default,
    #[tw(
        class = "bg-card text-destructive *:data-[slot=alert-description]:text-destructive/90 [&>svg]:text-current"
    )]
    Destructive,
}

/// Class builder for the Alert component.
#[derive(TwClass)]
#[tw(
    class = "relative grid w-full grid-cols-[0_1fr] items-start gap-y-0.5 rounded-lg border px-4 py-3 text-sm has-[>svg]:grid-cols-[calc(var(--spacing)*4)_1fr] has-[>svg]:gap-x-3 [&>svg]:size-4 [&>svg]:translate-y-0.5 [&>svg]:text-current"
)]
struct AlertClass {
    variant: AlertVariant,
}

/// The props for the styled [`Alert`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    /// Visual variant of the alert.
    #[props(default)]
    pub variant: AlertVariant,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the alert.
    pub children: Element,
}

/// Styled Alert root — matches shadcn exactly.
#[component]
pub fn Alert(props: AlertProps) -> Element {
    let class = AlertClass {
        variant: props.variant,
    }
    .with_class(props.class.unwrap_or_default());

    rsx! {
        div {
            role: "alert",
            "data-slot": "alert",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the styled [`AlertTitle`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertTitleProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the alert title.
    pub children: Element,
}

/// Styled AlertTitle — matches shadcn exactly.
#[component]
pub fn AlertTitle(props: AlertTitleProps) -> Element {
    let class = tw_merge!(
        "col-start-2 line-clamp-1 min-h-4 font-medium tracking-tight",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "alert-title",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the styled [`AlertDescription`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDescriptionProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the alert description.
    pub children: Element,
}

/// Styled AlertDescription — matches shadcn exactly.
#[component]
pub fn AlertDescription(props: AlertDescriptionProps) -> Element {
    let class = tw_merge!(
        "col-start-2 grid justify-items-start gap-1 text-sm text-muted-foreground [&_p]:leading-relaxed",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "alert-description",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}
