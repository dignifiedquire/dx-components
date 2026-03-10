//! Styled pagination matching shadcn/ui.
//!
//! Pure HTML + Tailwind. Uses button variant classes for link styling.

use dioxus::prelude::*;
use dx_icons_lucide::{IconChevronLeft, IconChevronRight, IconEllipsis};
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Pagination
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct PaginationProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let class = tw_merge!("mx-auto flex w-full justify-center", props.class);

    rsx! {
        nav {
            "data-slot": "pagination",
            role: "navigation",
            "aria-label": "pagination",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// PaginationContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct PaginationContentProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn PaginationContent(props: PaginationContentProps) -> Element {
    let class = tw_merge!("flex flex-row items-center gap-1", props.class);

    rsx! {
        ul {
            "data-slot": "pagination-content",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// PaginationItem
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct PaginationItemProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn PaginationItem(props: PaginationItemProps) -> Element {
    rsx! {
        li {
            "data-slot": "pagination-item",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// PaginationLink
// ---------------------------------------------------------------------------

/// Button base classes (shared with shadcn button).
const BUTTON_BASE: &str = "inline-flex shrink-0 items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-all cursor-pointer outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";
const BUTTON_GHOST: &str = "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50";
const BUTTON_OUTLINE: &str = "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:border-input dark:bg-input/30 dark:hover:bg-input/50";
const BUTTON_SIZE_ICON: &str = "size-9";
const BUTTON_SIZE_DEFAULT: &str = "h-9 px-4 py-2 has-[>svg]:px-3";

#[derive(Props, Clone, PartialEq)]
pub struct PaginationLinkProps {
    /// Whether this is the active/current page.
    #[props(default)]
    pub is_active: bool,

    /// The link destination.
    #[props(default)]
    pub href: Option<String>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn PaginationLink(props: PaginationLinkProps) -> Element {
    let variant_class = if props.is_active {
        BUTTON_OUTLINE
    } else {
        BUTTON_GHOST
    };

    let class = tw_merge!(BUTTON_BASE, variant_class, BUTTON_SIZE_ICON, props.class,);

    rsx! {
        a {
            "data-slot": "pagination-link",
            "data-active": props.is_active,
            "aria-current": if props.is_active { Some("page") } else { None },
            href: props.href,
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// PaginationPrevious
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct PaginationPreviousProps {
    #[props(default)]
    pub href: Option<String>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn PaginationPrevious(props: PaginationPreviousProps) -> Element {
    let class = tw_merge!(
        BUTTON_BASE,
        BUTTON_GHOST,
        BUTTON_SIZE_DEFAULT,
        "gap-1 px-2.5 sm:pl-2.5",
        props.class,
    );

    rsx! {
        a {
            "data-slot": "pagination-previous",
            "aria-label": "Go to previous page",
            href: props.href,
            class: class,
            ..props.attributes,
            IconChevronLeft { class: "size-4" }
            span { "Previous" }
        }
    }
}

// ---------------------------------------------------------------------------
// PaginationNext
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct PaginationNextProps {
    #[props(default)]
    pub href: Option<String>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn PaginationNext(props: PaginationNextProps) -> Element {
    let class = tw_merge!(
        BUTTON_BASE,
        BUTTON_GHOST,
        BUTTON_SIZE_DEFAULT,
        "gap-1 px-2.5 sm:pr-2.5",
        props.class,
    );

    rsx! {
        a {
            "data-slot": "pagination-next",
            "aria-label": "Go to next page",
            href: props.href,
            class: class,
            ..props.attributes,
            span { "Next" }
            IconChevronRight { class: "size-4" }
        }
    }
}

// ---------------------------------------------------------------------------
// PaginationEllipsis
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct PaginationEllipsisProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn PaginationEllipsis(props: PaginationEllipsisProps) -> Element {
    let class = tw_merge!("flex size-9 items-center justify-center", props.class,);

    rsx! {
        span {
            "data-slot": "pagination-ellipsis",
            "aria-hidden": "true",
            class: class,
            ..props.attributes,
            IconEllipsis { class: "size-4" }
            span { class: "sr-only", "More pages" }
        }
    }
}
