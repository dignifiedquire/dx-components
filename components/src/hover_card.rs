//! Styled hover card matching shadcn/ui.
//!
//! Wraps `dioxus_primitives::hover_card` with shadcn Tailwind classes.

use dioxus::prelude::*;
use dioxus_primitives::hover_card as primitives;
pub use dioxus_primitives::{ContentAlign, ContentSide};
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// HoverCard (Root — no DOM)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct HoverCardProps {
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    #[props(default)]
    pub default_open: bool,

    #[props(default)]
    pub on_open_change: Callback<bool>,

    pub children: Element,
}

#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
    rsx! {
        primitives::HoverCardRoot {
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// HoverCardTrigger
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct HoverCardTriggerProps {
    #[props(default)]
    pub id: Option<String>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
    rsx! {
        primitives::HoverCardTrigger {
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// HoverCardContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct HoverCardContentProps {
    #[props(default)]
    pub force_mount: bool,

    #[props(default = ContentSide::Bottom)]
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
pub fn HoverCardContent(props: HoverCardContentProps) -> Element {
    let class = tw_merge!(
        "z-50 w-64 origin-(--radix-hover-card-content-transform-origin) rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-hidden data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
        props.class,
    );

    rsx! {
        primitives::HoverCardContent {
            force_mount: props.force_mount,
            side: props.side,
            align: props.align,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
