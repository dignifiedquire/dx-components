//! Styled popover matching shadcn/ui.
//!
//! Wraps `dioxus_primitives::popover` with shadcn Tailwind classes.

use dioxus::prelude::*;
use dioxus_primitives::popover as primitives;
pub use dioxus_primitives::popper::{Align, Side};
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Popover (Root — no DOM)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct PopoverProps {
    #[props(default)]
    pub modal: bool,

    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    #[props(default)]
    pub default_open: bool,

    #[props(default)]
    pub on_open_change: Callback<bool>,

    pub children: Element,
}

#[component]
pub fn Popover(props: PopoverProps) -> Element {
    rsx! {
        primitives::PopoverRoot {
            modal: props.modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// PopoverTrigger
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct PopoverTriggerProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    rsx! {
        primitives::PopoverTrigger {
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// PopoverContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct PopoverContentProps {
    #[props(default)]
    pub force_mount: bool,

    #[props(default)]
    pub side: Side,

    /// Offset from the trigger edge in pixels. Defaults to 0.
    #[props(default)]
    pub side_offset: f64,

    #[props(default)]
    pub align: Align,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
    let class = tw_merge!(
        "z-50 w-72 origin-(--radix-popover-content-transform-origin) rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-hidden data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
        props.class,
    );

    rsx! {
        primitives::PopoverContent {
            force_mount: props.force_mount,
            side: props.side,
            side_offset: props.side_offset,
            align: props.align,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// PopoverClose
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct PopoverCloseProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn PopoverClose(props: PopoverCloseProps) -> Element {
    rsx! {
        primitives::PopoverClose {
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
