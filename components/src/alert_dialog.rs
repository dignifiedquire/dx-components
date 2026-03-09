//! Styled alert dialog matching shadcn/ui.
//!
//! Wraps `dioxus_primitives::alert_dialog` with shadcn Tailwind classes.
//! Adds `AlertDialogHeader`, `AlertDialogFooter` layout helpers.

use dioxus::prelude::*;
use dioxus_primitives::alert_dialog as primitives;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// AlertDialog (Root — no DOM)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogProps {
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    #[props(default)]
    pub default_open: bool,

    #[props(default)]
    pub on_open_change: Callback<bool>,

    pub children: Element,
}

#[component]
pub fn AlertDialog(props: AlertDialogProps) -> Element {
    rsx! {
        primitives::AlertDialogRoot {
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogTrigger
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogTriggerProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn AlertDialogTrigger(props: AlertDialogTriggerProps) -> Element {
    rsx! {
        primitives::AlertDialogTrigger {
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogOverlay
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogOverlayProps {
    #[props(default)]
    pub force_mount: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn AlertDialogOverlay(props: AlertDialogOverlayProps) -> Element {
    let class = tw_merge!(
        "fixed inset-0 z-50 bg-black/50 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0",
        props.class,
    );

    rsx! {
        primitives::AlertDialogOverlay {
            force_mount: props.force_mount,
            class: class,
            attributes: props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogContentProps {
    #[props(default)]
    pub force_mount: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn AlertDialogContent(props: AlertDialogContentProps) -> Element {
    let class = tw_merge!(
        "fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border bg-background p-6 shadow-lg duration-200 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 sm:max-w-lg",
        props.class,
    );

    rsx! {
        primitives::AlertDialogContent {
            force_mount: props.force_mount,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogHeader (HTML-only layout helper)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogHeaderProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn AlertDialogHeader(props: AlertDialogHeaderProps) -> Element {
    let class = tw_merge!("flex flex-col gap-2 text-center sm:text-left", props.class,);

    rsx! {
        div {
            "data-slot": "alert-dialog-header",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogFooter (HTML-only layout helper)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogFooterProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn AlertDialogFooter(props: AlertDialogFooterProps) -> Element {
    let class = tw_merge!(
        "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "alert-dialog-footer",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogTitle
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogTitleProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    let class = tw_merge!("text-lg font-semibold", props.class,);

    rsx! {
        primitives::AlertDialogTitle {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogDescription
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogDescriptionProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    let class = tw_merge!("text-sm text-muted-foreground", props.class,);

    rsx! {
        primitives::AlertDialogDescription {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogAction
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogActionProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    rsx! {
        primitives::AlertDialogAction {
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AlertDialogCancel
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogCancelProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    rsx! {
        primitives::AlertDialogCancel {
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
