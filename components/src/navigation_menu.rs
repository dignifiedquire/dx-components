//! Styled NavigationMenu matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::navigation_menu` primitive with
//! Tailwind classes — matching the shadcn/ui navigation-menu component.

use dioxus::prelude::*;
use dioxus_primitives::navigation_menu as primitives;
use tailwind_fuse::*;

// Re-export context and types
pub use primitives::{NavigationMenuCtx, NavigationMenuItemCtx, NavigationMenuOrientation};

/// Shared trigger style matching shadcn's `navigationMenuTriggerStyle()`.
pub const NAVIGATION_MENU_TRIGGER_STYLE: &str = "group inline-flex h-9 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-[color,box-shadow] outline-none hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 disabled:pointer-events-none disabled:opacity-50 data-[state=open]:bg-accent/50 data-[state=open]:text-accent-foreground data-[state=open]:hover:bg-accent data-[state=open]:focus:bg-accent";

// ---------------------------------------------------------------------------
// NavigationMenu (root)
// ---------------------------------------------------------------------------

/// Props for the styled [`NavigationMenu`].
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuProps {
    /// Controlled value (the currently open item).
    #[props(default)]
    pub value: Option<String>,

    /// Default value (uncontrolled).
    #[props(default)]
    pub default_value: String,

    /// Callback when value changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Menu orientation.
    #[props(default)]
    pub orientation: NavigationMenuOrientation,

    /// Whether to render a viewport container.
    #[props(default = true)]
    pub viewport: bool,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled NavigationMenu — matches shadcn.
#[component]
pub fn NavigationMenu(props: NavigationMenuProps) -> Element {
    let class = tw_merge!(
        "group/navigation-menu relative flex max-w-max flex-1 items-center justify-center",
        props.class,
    );

    rsx! {
        primitives::NavigationMenu {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            orientation: props.orientation,
            viewport: props.viewport,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuList
// ---------------------------------------------------------------------------

/// Props for the styled [`NavigationMenuList`].
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuListProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled NavigationMenuList — matches shadcn.
#[component]
pub fn NavigationMenuList(props: NavigationMenuListProps) -> Element {
    let class = tw_merge!(
        "group flex flex-1 list-none items-center justify-center gap-1",
        props.class,
    );

    rsx! {
        primitives::NavigationMenuList {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuItem
// ---------------------------------------------------------------------------

/// Props for the styled [`NavigationMenuItem`].
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuItemProps {
    /// Unique value for this item.
    #[props(default)]
    pub value: Option<String>,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled NavigationMenuItem — matches shadcn.
#[component]
pub fn NavigationMenuItem(props: NavigationMenuItemProps) -> Element {
    let class = tw_merge!("relative", props.class);

    rsx! {
        primitives::NavigationMenuItem {
            value: props.value,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuTrigger
// ---------------------------------------------------------------------------

/// Props for the styled [`NavigationMenuTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuTriggerProps {
    /// Whether the trigger is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled NavigationMenuTrigger — matches shadcn.
#[component]
pub fn NavigationMenuTrigger(props: NavigationMenuTriggerProps) -> Element {
    let class = tw_merge!(NAVIGATION_MENU_TRIGGER_STYLE, props.class);

    rsx! {
        primitives::NavigationMenuTrigger {
            disabled: props.disabled,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuContent
// ---------------------------------------------------------------------------

/// Props for the styled [`NavigationMenuContent`].
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuContentProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled NavigationMenuContent — matches shadcn.
#[component]
pub fn NavigationMenuContent(props: NavigationMenuContentProps) -> Element {
    let class = tw_merge!(
        "top-0 left-0 w-full p-2 pr-2.5 md:absolute md:w-auto",
        props.class,
    );

    rsx! {
        primitives::NavigationMenuContent {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuLink
// ---------------------------------------------------------------------------

/// Props for the styled [`NavigationMenuLink`].
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuLinkProps {
    /// Link href.
    #[props(default)]
    pub href: Option<String>,

    /// Whether this link is the active page.
    #[props(default)]
    pub active: bool,

    /// Click handler.
    #[props(default)]
    pub onclick: Callback<MouseEvent>,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled NavigationMenuLink — matches shadcn.
#[component]
pub fn NavigationMenuLink(props: NavigationMenuLinkProps) -> Element {
    let class = tw_merge!(
        "flex flex-col gap-1 rounded-sm p-2 text-sm transition-all outline-none hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 data-[active=true]:bg-accent/50 data-[active=true]:text-accent-foreground data-[active=true]:hover:bg-accent data-[active=true]:focus:bg-accent [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-muted-foreground",
        props.class,
    );

    rsx! {
        primitives::NavigationMenuLink {
            href: props.href,
            active: props.active,
            onclick: props.onclick,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuIndicator
// ---------------------------------------------------------------------------

/// Props for the styled [`NavigationMenuIndicator`].
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuIndicatorProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Custom indicator content.
    #[props(default)]
    pub children: Element,
}

/// Styled NavigationMenuIndicator — matches shadcn.
#[component]
pub fn NavigationMenuIndicator(props: NavigationMenuIndicatorProps) -> Element {
    let class = tw_merge!(
        "top-full z-[1] flex h-1.5 items-end justify-center overflow-hidden data-[state=hidden]:animate-out data-[state=hidden]:fade-out data-[state=visible]:animate-in data-[state=visible]:fade-in",
        props.class,
    );

    let has_children = props.children != Ok(VNode::placeholder());

    if has_children {
        rsx! {
            primitives::NavigationMenuIndicator {
                class: class,
                attributes: props.attributes,
                {props.children}
            }
        }
    } else {
        rsx! {
            primitives::NavigationMenuIndicator {
                class: class,
                attributes: props.attributes,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuViewport
// ---------------------------------------------------------------------------

/// Props for the styled [`NavigationMenuViewport`].
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuViewportProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled NavigationMenuViewport — matches shadcn.
#[component]
pub fn NavigationMenuViewport(props: NavigationMenuViewportProps) -> Element {
    let class = tw_merge!(
        "origin-top-center relative mt-1.5 h-[var(--radix-navigation-menu-viewport-height)] w-full overflow-hidden rounded-md border bg-popover text-popover-foreground shadow data-[state=closed]:animate-out data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:zoom-in-90 md:w-[var(--radix-navigation-menu-viewport-width)]",
        props.class,
    );

    rsx! {
        primitives::NavigationMenuViewport {
            class: class,
            attributes: props.attributes,
        }
    }
}
