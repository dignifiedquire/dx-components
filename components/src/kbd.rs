//! Styled keyboard shortcut display matching shadcn/ui.
//!
//! Pure HTML + Tailwind component.
//! No primitive dependency — renders native `<kbd>` elements.

use dioxus::prelude::*;
use tailwind_fuse::*;

/// The props for the styled [`Kbd`] component.
#[derive(Props, Clone, PartialEq)]
pub struct KbdProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the kbd.
    pub children: Element,
}

/// Styled Kbd — matches shadcn exactly.
#[component]
pub fn Kbd(props: KbdProps) -> Element {
    let class = tw_merge!(
        "pointer-events-none inline-flex h-5 w-fit min-w-5 items-center justify-center gap-1 rounded-sm bg-muted px-1 font-sans text-xs font-medium text-muted-foreground select-none",
        "[&_svg:not([class*='size-'])]:size-3",
        "[[data-slot=tooltip-content]_&]:bg-background/20 [[data-slot=tooltip-content]_&]:text-background dark:[[data-slot=tooltip-content]_&]:bg-background/10",
        props.class,
    );

    rsx! {
        kbd {
            "data-slot": "kbd",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the styled [`KbdGroup`] component.
#[derive(Props, Clone, PartialEq)]
pub struct KbdGroupProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the kbd group.
    pub children: Element,
}

/// Styled KbdGroup — matches shadcn exactly.
#[component]
pub fn KbdGroup(props: KbdGroupProps) -> Element {
    let class = tw_merge!("inline-flex items-center gap-1", props.class);

    rsx! {
        kbd {
            "data-slot": "kbd-group",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}
