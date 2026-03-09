//! Styled card matching shadcn/ui.
//!
//! Pure HTML + Tailwind component with 7 sub-components.
//! No primitive dependency — renders native HTML elements.

use dioxus::prelude::*;
use tailwind_fuse::*;

/// The props for the styled [`Card`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the card.
    pub children: Element,
}

/// Styled Card root — matches shadcn exactly.
#[component]
pub fn Card(props: CardProps) -> Element {
    let class = tw_merge!(
        "flex flex-col gap-6 rounded-xl border bg-card py-6 text-card-foreground shadow-sm",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "card",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the styled [`CardHeader`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the card header.
    pub children: Element,
}

/// Styled CardHeader — matches shadcn exactly.
#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    let class = tw_merge!(
        "@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-2 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "card-header",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the styled [`CardTitle`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CardTitleProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the card title.
    pub children: Element,
}

/// Styled CardTitle — matches shadcn exactly.
#[component]
pub fn CardTitle(props: CardTitleProps) -> Element {
    let class = tw_merge!("leading-none font-semibold", props.class);

    rsx! {
        div {
            "data-slot": "card-title",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the styled [`CardDescription`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CardDescriptionProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the card description.
    pub children: Element,
}

/// Styled CardDescription — matches shadcn exactly.
#[component]
pub fn CardDescription(props: CardDescriptionProps) -> Element {
    let class = tw_merge!("text-sm text-muted-foreground", props.class);

    rsx! {
        div {
            "data-slot": "card-description",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the styled [`CardAction`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CardActionProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the card action.
    pub children: Element,
}

/// Styled CardAction — matches shadcn exactly.
#[component]
pub fn CardAction(props: CardActionProps) -> Element {
    let class = tw_merge!(
        "col-start-2 row-span-2 row-start-1 self-start justify-self-end",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "card-action",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the styled [`CardContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CardContentProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the card content.
    pub children: Element,
}

/// Styled CardContent — matches shadcn exactly.
#[component]
pub fn CardContent(props: CardContentProps) -> Element {
    let class = tw_merge!("px-6", props.class);

    rsx! {
        div {
            "data-slot": "card-content",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the styled [`CardFooter`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CardFooterProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the card footer.
    pub children: Element,
}

/// Styled CardFooter — matches shadcn exactly.
#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    let class = tw_merge!("flex items-center px-6 [.border-t]:pt-6", props.class,);

    rsx! {
        div {
            "data-slot": "card-footer",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}
