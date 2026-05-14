//! Styled card matching shadcn/ui (radix-flavor).
//!
//! Pure HTML + Tailwind component with 7 sub-components. No primitive
//! dependency — renders native HTML elements with `data-slot` attributes.

use dioxus::prelude::*;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// CardSize
// ---------------------------------------------------------------------------

/// Size variants for the styled [`Card`].
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum CardSize {
    /// Standard padding and gap.
    #[default]
    Default,
    /// Tighter padding and gap — useful inside dense layouts.
    Sm,
}

impl CardSize {
    fn as_data_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
        }
    }
}

// ---------------------------------------------------------------------------
// Card (root)
// ---------------------------------------------------------------------------

/// The props for the styled [`Card`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    /// Size variant.
    #[props(default)]
    pub size: CardSize,

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
        "group/card flex flex-col gap-4 overflow-hidden rounded-xl bg-card py-4 text-sm text-card-foreground ring-1 ring-foreground/10 has-data-[slot=card-footer]:pb-0 has-[>img:first-child]:pt-0 data-[size=sm]:gap-3 data-[size=sm]:py-3 data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "card",
            "data-size": props.size.as_data_attr(),
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CardHeader
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    let class = tw_merge!(
        "group/card-header @container/card-header grid auto-rows-min items-start gap-1 rounded-t-xl px-4 group-data-[size=sm]/card:px-3 has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto] [.border-b]:pb-4 group-data-[size=sm]/card:[.border-b]:pb-3",
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

// ---------------------------------------------------------------------------
// CardTitle
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct CardTitleProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn CardTitle(props: CardTitleProps) -> Element {
    let class = tw_merge!(
        "text-base leading-snug font-medium group-data-[size=sm]/card:text-sm",
        props.class
    );

    rsx! {
        div {
            "data-slot": "card-title",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CardDescription
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct CardDescriptionProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

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

// ---------------------------------------------------------------------------
// CardAction
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct CardActionProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

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

// ---------------------------------------------------------------------------
// CardContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct CardContentProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn CardContent(props: CardContentProps) -> Element {
    let class = tw_merge!("px-4 group-data-[size=sm]/card:px-3", props.class);

    rsx! {
        div {
            "data-slot": "card-content",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CardFooter
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct CardFooterProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    let class = tw_merge!(
        "flex items-center rounded-b-xl border-t bg-muted/50 p-4 group-data-[size=sm]/card:p-3",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "card-footer",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}
