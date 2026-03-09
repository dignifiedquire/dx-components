//! Styled dialog matching shadcn/ui.
//!
//! Wraps `dioxus_primitives::dialog` with shadcn Tailwind classes.
//! Adds `DialogHeader`, `DialogFooter`, and a composed close button with X icon.

use dioxus::prelude::*;
use dioxus_primitives::dialog as primitives;
use dx_icons_lucide::IconX;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Dialog (Root — no DOM)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    #[props(default = true)]
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
pub fn Dialog(props: DialogProps) -> Element {
    rsx! {
        primitives::DialogRoot {
            modal: props.modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DialogTrigger
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DialogTriggerProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn DialogTrigger(props: DialogTriggerProps) -> Element {
    rsx! {
        primitives::DialogTrigger {
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DialogOverlay
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DialogOverlayProps {
    #[props(default)]
    pub force_mount: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn DialogOverlay(props: DialogOverlayProps) -> Element {
    let class = tw_merge!(
        "fixed inset-0 z-50 bg-black/50 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0",
        props.class,
    );

    rsx! {
        primitives::DialogOverlay {
            force_mount: props.force_mount,
            class: class,
            attributes: props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// DialogContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DialogContentProps {
    #[props(default)]
    pub force_mount: bool,

    /// Show the default close button (X icon) in top-right corner.
    #[props(default = true)]
    pub show_close: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn DialogContent(props: DialogContentProps) -> Element {
    let class = tw_merge!(
        "fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border bg-background p-6 shadow-lg duration-200 outline-none data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 sm:max-w-lg",
        props.class,
    );

    rsx! {
        primitives::DialogContent {
            force_mount: props.force_mount,
            class: class,
            attributes: props.attributes,
            {props.children}
            if props.show_close {
                primitives::DialogClose {
                    class: "absolute top-4 right-4 rounded-xs opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
                    IconX { class: "size-4" }
                    span { class: "sr-only", "Close" }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// DialogClose
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DialogCloseProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn DialogClose(props: DialogCloseProps) -> Element {
    rsx! {
        primitives::DialogClose {
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DialogHeader (HTML-only layout helper)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DialogHeaderProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn DialogHeader(props: DialogHeaderProps) -> Element {
    let class = tw_merge!("flex flex-col gap-2 text-center sm:text-left", props.class,);

    rsx! {
        div {
            "data-slot": "dialog-header",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DialogFooter (HTML-only layout helper)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DialogFooterProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn DialogFooter(props: DialogFooterProps) -> Element {
    let class = tw_merge!(
        "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "dialog-footer",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DialogTitle
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let class = tw_merge!("text-lg leading-none font-semibold", props.class,);

    rsx! {
        primitives::DialogTitle {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DialogDescription
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DialogDescriptionProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let class = tw_merge!("text-sm text-muted-foreground", props.class,);

    rsx! {
        primitives::DialogDescription {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
