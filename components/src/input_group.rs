//! Styled input group matching shadcn/ui.
//!
//! Pure HTML + Tailwind — no Radix primitive needed.
//! Groups an input (or textarea) with addons, buttons, and text labels.

use dioxus::prelude::*;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// InputGroupAddonAlign
// ---------------------------------------------------------------------------

/// Alignment position for [`InputGroupAddon`].
#[derive(Default, Clone, Copy, PartialEq)]
pub enum InputGroupAddonAlign {
    #[default]
    InlineStart,
    InlineEnd,
    BlockStart,
    BlockEnd,
}

// ---------------------------------------------------------------------------
// InputGroup
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupProps {
    /// Whether the group is disabled.
    #[props(default)]
    pub disabled: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn InputGroup(props: InputGroupProps) -> Element {
    let class = tw_merge!(
        "group/input-group relative flex w-full items-center rounded-md border border-input shadow-xs transition-[color,box-shadow] outline-none dark:bg-input/30 h-9 min-w-0 has-[>textarea]:h-auto has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-[3px] has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:border-destructive has-[[data-slot][aria-invalid=true]]:ring-destructive/20 dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "input-group",
            "data-disabled": if props.disabled { Some("true") } else { None },
            role: "group",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// InputGroupAddon
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupAddonProps {
    /// Where the addon is positioned.
    #[props(default)]
    pub align: InputGroupAddonAlign,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn InputGroupAddon(props: InputGroupAddonProps) -> Element {
    let align_class = match props.align {
        InputGroupAddonAlign::InlineStart => "order-first pl-3",
        InputGroupAddonAlign::InlineEnd => "order-last pr-3",
        InputGroupAddonAlign::BlockStart => "order-first w-full justify-start px-3 pt-3",
        InputGroupAddonAlign::BlockEnd => "order-last w-full justify-start px-3 pb-3",
    };

    let align_attr = match props.align {
        InputGroupAddonAlign::InlineStart => "inline-start",
        InputGroupAddonAlign::InlineEnd => "inline-end",
        InputGroupAddonAlign::BlockStart => "block-start",
        InputGroupAddonAlign::BlockEnd => "block-end",
    };

    let class = tw_merge!(
        "flex h-auto cursor-text items-center justify-center gap-2 py-1.5 text-sm font-medium text-muted-foreground select-none group-data-[disabled=true]/input-group:opacity-50 [&>svg:not([class*='size-'])]:size-4",
        align_class,
        props.class,
    );

    rsx! {
        div {
            "data-slot": "input-group-addon",
            "data-align": align_attr,
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// InputGroupText
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupTextProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn InputGroupText(props: InputGroupTextProps) -> Element {
    let class = tw_merge!(
        "flex items-center gap-2 text-sm text-muted-foreground [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4",
        props.class,
    );

    rsx! {
        span {
            "data-slot": "input-group-text",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// InputGroupInput
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupInputProps {
    /// Input event handler.
    #[props(default)]
    pub oninput: Option<EventHandler<FormEvent>>,

    /// Change event handler.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,

    /// Focus event handler.
    #[props(default)]
    pub onfocus: Option<EventHandler<FocusEvent>>,

    /// Blur event handler.
    #[props(default)]
    pub onblur: Option<EventHandler<FocusEvent>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = input, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn InputGroupInput(props: InputGroupInputProps) -> Element {
    let class = tw_merge!(
        "flex-1 rounded-none border-0 bg-transparent shadow-none outline-none focus-visible:ring-0 h-9 w-full min-w-0 px-3 py-1 text-base md:text-sm dark:bg-transparent",
        props.class,
    );

    rsx! {
        input {
            "data-slot": "input-group-control",
            class: class,
            oninput: move |e| if let Some(handler) = &props.oninput { handler.call(e) },
            onchange: move |e| if let Some(handler) = &props.onchange { handler.call(e) },
            onfocus: move |e| if let Some(handler) = &props.onfocus { handler.call(e) },
            onblur: move |e| if let Some(handler) = &props.onblur { handler.call(e) },
            ..props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// InputGroupTextarea
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupTextareaProps {
    /// Input event handler.
    #[props(default)]
    pub oninput: Option<EventHandler<FormEvent>>,

    /// Focus event handler.
    #[props(default)]
    pub onfocus: Option<EventHandler<FocusEvent>>,

    /// Blur event handler.
    #[props(default)]
    pub onblur: Option<EventHandler<FocusEvent>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = textarea, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn InputGroupTextarea(props: InputGroupTextareaProps) -> Element {
    let class = tw_merge!(
        "flex-1 resize-none rounded-none border-0 bg-transparent py-3 shadow-none outline-none focus-visible:ring-0 w-full min-w-0 px-3 text-base md:text-sm dark:bg-transparent",
        props.class,
    );

    rsx! {
        textarea {
            "data-slot": "input-group-control",
            class: class,
            oninput: move |e| if let Some(handler) = &props.oninput { handler.call(e) },
            onfocus: move |e| if let Some(handler) = &props.onfocus { handler.call(e) },
            onblur: move |e| if let Some(handler) = &props.onblur { handler.call(e) },
            ..props.attributes,
        }
    }
}
