//! Styled badge matching shadcn/ui.
//!
//! Pure HTML + Tailwind component with 6 variant options.
//! No primitive dependency — renders a native `<span>`.

use dioxus::prelude::*;
use tailwind_fuse::*;

/// Visual variant of the badge.
#[derive(TwVariant, PartialEq)]
pub enum BadgeVariant {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground [a&]:hover:bg-primary/90"
    )]
    Default,
    #[tw(class = "bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90")]
    Secondary,
    #[tw(
        class = "bg-destructive text-white focus-visible:ring-destructive/20 dark:bg-destructive/60 dark:focus-visible:ring-destructive/40 [a&]:hover:bg-destructive/90"
    )]
    Destructive,
    #[tw(
        class = "border-border text-foreground [a&]:hover:bg-accent [a&]:hover:text-accent-foreground"
    )]
    Outline,
    #[tw(class = "[a&]:hover:bg-accent [a&]:hover:text-accent-foreground")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 [a&]:hover:underline")]
    Link,
}

/// Class builder for the Badge component.
#[derive(TwClass)]
#[tw(
    class = "inline-flex w-fit shrink-0 items-center justify-center gap-1 overflow-hidden rounded-full border border-transparent px-2 py-0.5 text-xs font-medium whitespace-nowrap transition-[color,box-shadow] focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&>svg]:pointer-events-none [&>svg]:size-3"
)]
struct BadgeClass {
    variant: BadgeVariant,
}

/// The props for the styled [`Badge`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    /// Visual variant of the badge.
    #[props(default)]
    pub variant: BadgeVariant,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the badge.
    pub children: Element,
}

/// Styled Badge — matches shadcn exactly.
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let class = BadgeClass {
        variant: props.variant,
    }
    .with_class(props.class.unwrap_or_default());

    let variant_name = match props.variant {
        BadgeVariant::Default => "default",
        BadgeVariant::Secondary => "secondary",
        BadgeVariant::Destructive => "destructive",
        BadgeVariant::Outline => "outline",
        BadgeVariant::Ghost => "ghost",
        BadgeVariant::Link => "link",
    };

    rsx! {
        span {
            "data-slot": "badge",
            "data-variant": variant_name,
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}
