//! Styled sheet matching shadcn/ui.
//!
//! Wraps `dioxus_primitives::dialog` with side-based slide-in Tailwind classes.
//! Adds `SheetHeader`, `SheetFooter`, and a composed close button with X icon.

use dioxus::prelude::*;
use dioxus_primitives::dialog as primitives;
use dx_icons_lucide::IconX;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// SheetSide
// ---------------------------------------------------------------------------

/// Which edge the sheet slides in from.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

impl SheetSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            SheetSide::Top => "top",
            SheetSide::Right => "right",
            SheetSide::Bottom => "bottom",
            SheetSide::Left => "left",
        }
    }
}

// ---------------------------------------------------------------------------
// Sheet (Root — no DOM)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SheetProps {
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    #[props(default)]
    pub default_open: bool,

    #[props(default)]
    pub on_open_change: Callback<bool>,

    pub children: Element,
}

#[component]
pub fn Sheet(props: SheetProps) -> Element {
    rsx! {
        primitives::DialogRoot {
            modal: true,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// SheetTrigger
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SheetTriggerProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn SheetTrigger(props: SheetTriggerProps) -> Element {
    rsx! {
        primitives::DialogTrigger {
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// SheetOverlay
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SheetOverlayProps {
    #[props(default)]
    pub force_mount: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SheetOverlay(props: SheetOverlayProps) -> Element {
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
// SheetContent
// ---------------------------------------------------------------------------

const SHEET_CONTENT_BASE: &str = "bg-background fixed z-50 flex flex-col gap-4 shadow-lg transition ease-in-out outline-none data-[state=closed]:duration-300 data-[state=open]:duration-500 data-[state=open]:animate-in data-[state=closed]:animate-out";

fn sheet_side_class(side: SheetSide) -> &'static str {
    match side {
        SheetSide::Top => "data-[state=closed]:slide-out-to-top data-[state=open]:slide-in-from-top inset-x-0 top-0 border-b",
        SheetSide::Right => "data-[state=closed]:slide-out-to-right data-[state=open]:slide-in-from-right inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm",
        SheetSide::Bottom => "data-[state=closed]:slide-out-to-bottom data-[state=open]:slide-in-from-bottom inset-x-0 bottom-0 border-t",
        SheetSide::Left => "data-[state=closed]:slide-out-to-left data-[state=open]:slide-in-from-left inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm",
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SheetContentProps {
    #[props(default)]
    pub side: SheetSide,

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
pub fn SheetContent(props: SheetContentProps) -> Element {
    let class = tw_merge!(
        SHEET_CONTENT_BASE,
        sheet_side_class(props.side),
        props.class,
    );

    rsx! {
        primitives::DialogContent {
            force_mount: props.force_mount,
            class: class,
            "data-slot": "sheet-content",
            "data-side": props.side.as_str(),
            attributes: props.attributes,
            {props.children}
            if props.show_close {
                primitives::DialogClose {
                    class: "ring-offset-background focus:ring-ring data-[state=open]:bg-secondary absolute top-4 right-4 rounded-xs opacity-70 transition-opacity hover:opacity-100 focus:ring-2 focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none",
                    IconX { class: "size-4" }
                    span { class: "sr-only", "Close" }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// SheetClose
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SheetCloseProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn SheetClose(props: SheetCloseProps) -> Element {
    rsx! {
        primitives::DialogClose {
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// SheetHeader (HTML-only layout helper)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SheetHeaderProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn SheetHeader(props: SheetHeaderProps) -> Element {
    let class = tw_merge!("flex flex-col gap-2", props.class);

    rsx! {
        div {
            "data-slot": "sheet-header",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// SheetFooter (HTML-only layout helper)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SheetFooterProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn SheetFooter(props: SheetFooterProps) -> Element {
    let class = tw_merge!(
        "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "sheet-footer",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// SheetTitle
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SheetTitleProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn SheetTitle(props: SheetTitleProps) -> Element {
    let class = tw_merge!("text-lg leading-none font-semibold", props.class);

    rsx! {
        primitives::DialogTitle {
            class: class,
            "data-slot": "sheet-title",
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// SheetDescription
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SheetDescriptionProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn SheetDescription(props: SheetDescriptionProps) -> Element {
    let class = tw_merge!("text-sm text-muted-foreground", props.class);

    rsx! {
        primitives::DialogDescription {
            class: class,
            "data-slot": "sheet-description",
            attributes: props.attributes,
            {props.children}
        }
    }
}
