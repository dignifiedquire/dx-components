//! Styled item list matching shadcn/ui.
//!
//! Pure HTML + Tailwind — no Radix primitive needed.

use dioxus::prelude::*;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Variants
// ---------------------------------------------------------------------------

/// Visual variant for [`Item`].
#[derive(Default, Clone, Copy, PartialEq)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

/// Size for [`Item`].
#[derive(Default, Clone, Copy, PartialEq)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
}

/// Visual variant for [`ItemMedia`].
#[derive(Default, Clone, Copy, PartialEq)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

// ---------------------------------------------------------------------------
// ItemGroup
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ItemGroupProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ItemGroup(props: ItemGroupProps) -> Element {
    let class = tw_merge!("group/item-group flex flex-col", props.class);

    rsx! {
        div {
            "data-slot": "item-group",
            role: "list",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ItemSeparator
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ItemSeparatorProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ItemSeparator(props: ItemSeparatorProps) -> Element {
    let class = tw_merge!(
        "my-0 shrink-0 bg-border data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "item-separator",
            role: "none",
            "data-orientation": "horizontal",
            class: class,
            ..props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// Item
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ItemProps {
    /// Visual variant.
    #[props(default)]
    pub variant: ItemVariant,

    /// Size.
    #[props(default)]
    pub size: ItemSize,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn Item(props: ItemProps) -> Element {
    let variant_class = match props.variant {
        ItemVariant::Default => "bg-transparent",
        ItemVariant::Outline => "border-border",
        ItemVariant::Muted => "bg-muted/50",
    };

    let size_class = match props.size {
        ItemSize::Default => "gap-4 p-4",
        ItemSize::Sm => "gap-2.5 px-4 py-3",
    };

    let class = tw_merge!(
        "group/item flex flex-wrap items-center rounded-md border border-transparent text-sm transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors [a]:hover:bg-accent/50",
        variant_class,
        size_class,
        props.class,
    );

    rsx! {
        div {
            "data-slot": "item",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ItemMedia
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ItemMediaProps {
    /// Visual variant.
    #[props(default)]
    pub variant: ItemMediaVariant,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ItemMedia(props: ItemMediaProps) -> Element {
    let variant_class = match props.variant {
        ItemMediaVariant::Default => "bg-transparent",
        ItemMediaVariant::Icon => {
            "size-8 rounded-sm border bg-muted [&_svg:not([class*='size-'])]:size-4"
        }
        ItemMediaVariant::Image => {
            "size-10 overflow-hidden rounded-sm [&_img]:size-full [&_img]:object-cover"
        }
    };

    let class = tw_merge!(
        "flex shrink-0 items-center justify-center gap-2 group-has-[[data-slot=item-description]]/item:translate-y-0.5 group-has-[[data-slot=item-description]]/item:self-start [&_svg]:pointer-events-none",
        variant_class,
        props.class,
    );

    rsx! {
        div {
            "data-slot": "item-media",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ItemContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ItemContentProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ItemContent(props: ItemContentProps) -> Element {
    let class = tw_merge!(
        "flex flex-1 flex-col gap-1 [&+[data-slot=item-content]]:flex-none",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "item-content",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ItemTitle
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ItemTitleProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ItemTitle(props: ItemTitleProps) -> Element {
    let class = tw_merge!(
        "flex w-fit items-center gap-2 text-sm leading-snug font-medium",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "item-title",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ItemDescription
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ItemDescriptionProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ItemDescription(props: ItemDescriptionProps) -> Element {
    let class = tw_merge!(
        "line-clamp-2 text-sm leading-normal font-normal text-balance text-muted-foreground [&>a]:underline [&>a]:underline-offset-4 [&>a:hover]:text-primary",
        props.class,
    );

    rsx! {
        p {
            "data-slot": "item-description",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ItemActions
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ItemActionsProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ItemActions(props: ItemActionsProps) -> Element {
    let class = tw_merge!("flex items-center gap-2", props.class);

    rsx! {
        div {
            "data-slot": "item-actions",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ItemHeader
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ItemHeaderProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ItemHeader(props: ItemHeaderProps) -> Element {
    let class = tw_merge!(
        "flex basis-full items-center justify-between gap-2",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "item-header",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ItemFooter
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ItemFooterProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ItemFooter(props: ItemFooterProps) -> Element {
    let class = tw_merge!(
        "flex basis-full items-center justify-between gap-2",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "item-footer",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}
