//! Styled tooltip matching shadcn/ui.
//!
//! Wraps `dioxus_primitives::tooltip` with shadcn Tailwind classes.

use dioxus::prelude::*;
use dioxus_primitives::tooltip as primitives;
pub use dioxus_primitives::{ContentAlign, ContentSide};
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Tooltip (Root — no DOM)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    #[props(default)]
    pub default_open: bool,

    #[props(default)]
    pub on_open_change: Callback<bool>,

    #[props(default)]
    pub disabled: bool,

    pub children: Element,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    rsx! {
        primitives::TooltipRoot {
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// TooltipTrigger
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct TooltipTriggerProps {
    #[props(default)]
    pub id: Option<String>,

    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
    rsx! {
        primitives::TooltipTrigger {
            id: props.id,
            r#as: props.r#as,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// TooltipContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct TooltipContentProps {
    #[props(default)]
    pub force_mount: bool,

    #[props(default = ContentSide::Top)]
    pub side: ContentSide,

    #[props(default = ContentAlign::Center)]
    pub align: ContentAlign,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
    let class = tw_merge!(
        "z-50 w-fit origin-(--radix-tooltip-content-transform-origin) animate-in rounded-md bg-foreground px-3 py-1.5 text-xs text-balance text-background fade-in-0 zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95",
        props.class,
    );

    rsx! {
        primitives::TooltipContent {
            force_mount: props.force_mount,
            side: props.side,
            align: props.align,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
