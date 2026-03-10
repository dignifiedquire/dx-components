//! Styled drawer matching shadcn/ui.
//!
//! Wraps `dioxus_primitives::dialog` with edge-sliding Tailwind classes
//! matching shadcn's vaul-based Drawer component. Unlike Sheet, Drawer
//! uses rounded corners and max-height/max-width constraints.

use dioxus::prelude::*;
use dioxus_primitives::dialog as primitives;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// DrawerDirection
// ---------------------------------------------------------------------------

/// Which edge the drawer slides in from.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DrawerDirection {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

impl DrawerDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }
}

// ---------------------------------------------------------------------------
// Drawer (Root — no DOM)
// ---------------------------------------------------------------------------

/// Props for the styled [`Drawer`].
#[derive(Props, Clone, PartialEq)]
pub struct DrawerProps {
    /// Controlled open state.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// Default open state (uncontrolled).
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Children.
    pub children: Element,
}

/// Styled Drawer root — wraps Dialog with drawer semantics.
#[component]
pub fn Drawer(props: DrawerProps) -> Element {
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
// DrawerTrigger
// ---------------------------------------------------------------------------

/// Props for the styled [`DrawerTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct DrawerTriggerProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled DrawerTrigger — matches shadcn.
#[component]
pub fn DrawerTrigger(props: DrawerTriggerProps) -> Element {
    rsx! {
        primitives::DialogTrigger {
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DrawerOverlay
// ---------------------------------------------------------------------------

/// Props for the styled [`DrawerOverlay`].
#[derive(Props, Clone, PartialEq)]
pub struct DrawerOverlayProps {
    /// Force mount the overlay.
    #[props(default)]
    pub force_mount: bool,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled DrawerOverlay — matches shadcn.
#[component]
pub fn DrawerOverlay(props: DrawerOverlayProps) -> Element {
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
// DrawerContent
// ---------------------------------------------------------------------------

const DRAWER_CONTENT_BASE: &str = "group/drawer-content bg-background fixed z-50 flex h-auto flex-col shadow-lg outline-none data-[state=open]:animate-in data-[state=closed]:animate-out";

fn drawer_direction_class(direction: DrawerDirection) -> &'static str {
    match direction {
        DrawerDirection::Top => {
            "inset-x-0 top-0 max-h-[80%] rounded-b-[10px] border-b data-[state=closed]:slide-out-to-top data-[state=open]:slide-in-from-top"
        }
        DrawerDirection::Right => {
            "inset-y-0 right-0 h-full w-[80%] max-w-[500px] rounded-l-[10px] border-l data-[state=closed]:slide-out-to-right data-[state=open]:slide-in-from-right"
        }
        DrawerDirection::Bottom => {
            "inset-x-0 bottom-0 max-h-[80%] rounded-t-[10px] border-t data-[state=closed]:slide-out-to-bottom data-[state=open]:slide-in-from-bottom"
        }
        DrawerDirection::Left => {
            "inset-y-0 left-0 h-full w-[80%] max-w-[500px] rounded-r-[10px] border-r data-[state=closed]:slide-out-to-left data-[state=open]:slide-in-from-left"
        }
    }
}

/// Props for the styled [`DrawerContent`].
#[derive(Props, Clone, PartialEq)]
pub struct DrawerContentProps {
    /// Which edge the drawer slides from.
    #[props(default)]
    pub direction: DrawerDirection,

    /// Force mount the content.
    #[props(default)]
    pub force_mount: bool,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled DrawerContent — matches shadcn drawer with directional sliding.
#[component]
pub fn DrawerContent(props: DrawerContentProps) -> Element {
    let class = tw_merge!(
        DRAWER_CONTENT_BASE,
        drawer_direction_class(props.direction),
        props.class,
    );

    rsx! {
        primitives::DialogContent {
            force_mount: props.force_mount,
            class: class,
            "data-slot": "drawer-content",
            "data-vaul-drawer-direction": props.direction.as_str(),
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DrawerClose
// ---------------------------------------------------------------------------

/// Props for the styled [`DrawerClose`].
#[derive(Props, Clone, PartialEq)]
pub struct DrawerCloseProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled DrawerClose — matches shadcn.
#[component]
pub fn DrawerClose(props: DrawerCloseProps) -> Element {
    rsx! {
        primitives::DialogClose {
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DrawerHeader (HTML-only layout helper)
// ---------------------------------------------------------------------------

/// Props for the styled [`DrawerHeader`].
#[derive(Props, Clone, PartialEq)]
pub struct DrawerHeaderProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled DrawerHeader — matches shadcn.
#[component]
pub fn DrawerHeader(props: DrawerHeaderProps) -> Element {
    let class = tw_merge!("flex flex-col gap-1.5 p-4", props.class);

    rsx! {
        div {
            "data-slot": "drawer-header",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DrawerFooter (HTML-only layout helper)
// ---------------------------------------------------------------------------

/// Props for the styled [`DrawerFooter`].
#[derive(Props, Clone, PartialEq)]
pub struct DrawerFooterProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled DrawerFooter — matches shadcn.
#[component]
pub fn DrawerFooter(props: DrawerFooterProps) -> Element {
    let class = tw_merge!("mt-auto flex flex-col gap-2 p-4", props.class);

    rsx! {
        div {
            "data-slot": "drawer-footer",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DrawerTitle
// ---------------------------------------------------------------------------

/// Props for the styled [`DrawerTitle`].
#[derive(Props, Clone, PartialEq)]
pub struct DrawerTitleProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled DrawerTitle — matches shadcn.
#[component]
pub fn DrawerTitle(props: DrawerTitleProps) -> Element {
    let class = tw_merge!("font-semibold text-foreground", props.class);

    rsx! {
        primitives::DialogTitle {
            class: class,
            "data-slot": "drawer-title",
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DrawerDescription
// ---------------------------------------------------------------------------

/// Props for the styled [`DrawerDescription`].
#[derive(Props, Clone, PartialEq)]
pub struct DrawerDescriptionProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled DrawerDescription — matches shadcn.
#[component]
pub fn DrawerDescription(props: DrawerDescriptionProps) -> Element {
    let class = tw_merge!("text-sm text-muted-foreground", props.class);

    rsx! {
        primitives::DialogDescription {
            class: class,
            "data-slot": "drawer-description",
            attributes: props.attributes,
            {props.children}
        }
    }
}
