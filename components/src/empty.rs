//! Styled empty state matching shadcn/ui.
//!
//! Pure HTML + Tailwind — no Radix primitive needed.

use dioxus::prelude::*;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// EmptyMediaVariant
// ---------------------------------------------------------------------------

/// Visual variant for [`EmptyMedia`].
#[derive(Default, Clone, Copy, PartialEq)]
pub enum EmptyMediaVariant {
    #[default]
    Default,
    Icon,
}

// ---------------------------------------------------------------------------
// Empty
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct EmptyProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn Empty(props: EmptyProps) -> Element {
    let class = tw_merge!(
        "flex min-w-0 flex-1 flex-col items-center justify-center gap-6 rounded-lg border-dashed p-6 text-center text-balance md:p-12",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "empty",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// EmptyHeader
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct EmptyHeaderProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn EmptyHeader(props: EmptyHeaderProps) -> Element {
    let class = tw_merge!(
        "flex max-w-sm flex-col items-center gap-2 text-center",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "empty-header",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// EmptyMedia
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct EmptyMediaProps {
    /// Visual variant.
    #[props(default)]
    pub variant: EmptyMediaVariant,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn EmptyMedia(props: EmptyMediaProps) -> Element {
    let variant_class = match props.variant {
        EmptyMediaVariant::Default => "bg-transparent",
        EmptyMediaVariant::Icon => {
            "flex size-10 shrink-0 items-center justify-center rounded-lg bg-muted text-foreground [&_svg:not([class*='size-'])]:size-6"
        }
    };

    let class = tw_merge!(
        "mb-2 flex shrink-0 items-center justify-center [&_svg]:pointer-events-none [&_svg]:shrink-0",
        variant_class,
        props.class,
    );

    rsx! {
        div {
            "data-slot": "empty-icon",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// EmptyTitle
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct EmptyTitleProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn EmptyTitle(props: EmptyTitleProps) -> Element {
    let class = tw_merge!("text-lg font-medium tracking-tight", props.class);

    rsx! {
        div {
            "data-slot": "empty-title",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// EmptyDescription
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct EmptyDescriptionProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn EmptyDescription(props: EmptyDescriptionProps) -> Element {
    let class = tw_merge!(
        "text-sm/relaxed text-muted-foreground [&>a]:underline [&>a]:underline-offset-4 [&>a:hover]:text-primary",
        props.class,
    );

    rsx! {
        p {
            "data-slot": "empty-description",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// EmptyContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct EmptyContentProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn EmptyContent(props: EmptyContentProps) -> Element {
    let class = tw_merge!(
        "flex w-full max-w-sm min-w-0 flex-col items-center gap-4 text-sm text-balance",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "empty-content",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}
