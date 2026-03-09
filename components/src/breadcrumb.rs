//! Styled breadcrumb navigation matching shadcn/ui.
//!
//! Pure HTML + Tailwind — no Radix primitive needed.

use dioxus::prelude::*;
use dx_icons_lucide::{IconChevronRight, IconEllipsis};
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Breadcrumb
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// Semantic `<nav>` wrapper with `aria-label="breadcrumb"`.
#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    rsx! {
        nav {
            "data-slot": "breadcrumb",
            "aria-label": "breadcrumb",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// BreadcrumbList
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbListProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn BreadcrumbList(props: BreadcrumbListProps) -> Element {
    let class = tw_merge!(
        "flex flex-wrap items-center gap-1.5 text-sm break-words text-muted-foreground sm:gap-2.5",
        props.class,
    );

    rsx! {
        ol {
            "data-slot": "breadcrumb-list",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// BreadcrumbItem
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbItemProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn BreadcrumbItem(props: BreadcrumbItemProps) -> Element {
    let class = tw_merge!("inline-flex items-center gap-1.5", props.class);

    rsx! {
        li {
            "data-slot": "breadcrumb-item",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// BreadcrumbLink
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbLinkProps {
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
pub fn BreadcrumbLink(props: BreadcrumbLinkProps) -> Element {
    let class = tw_merge!("transition-colors hover:text-foreground", props.class);

    rsx! {
        a {
            "data-slot": "breadcrumb-link",
            class: class,
            href: props.href,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// BreadcrumbPage
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbPageProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// Current page indicator — renders a `<span>` with `aria-current="page"`.
#[component]
pub fn BreadcrumbPage(props: BreadcrumbPageProps) -> Element {
    let class = tw_merge!("font-normal text-foreground", props.class);

    rsx! {
        span {
            "data-slot": "breadcrumb-page",
            role: "link",
            "aria-disabled": "true",
            "aria-current": "page",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// BreadcrumbSeparator
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbSeparatorProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Separator between breadcrumb items. Renders a ChevronRight icon.
#[component]
pub fn BreadcrumbSeparator(props: BreadcrumbSeparatorProps) -> Element {
    let class = tw_merge!("[&>svg]:size-3.5", props.class);

    rsx! {
        li {
            "data-slot": "breadcrumb-separator",
            role: "presentation",
            "aria-hidden": "true",
            class: class,
            ..props.attributes,
            IconChevronRight {}
        }
    }
}

// ---------------------------------------------------------------------------
// BreadcrumbEllipsis
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbEllipsisProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn BreadcrumbEllipsis(props: BreadcrumbEllipsisProps) -> Element {
    let class = tw_merge!("flex size-9 items-center justify-center", props.class,);

    rsx! {
        span {
            "data-slot": "breadcrumb-ellipsis",
            role: "presentation",
            "aria-hidden": "true",
            class: class,
            ..props.attributes,
            IconEllipsis { class: "size-4" }
            span { class: "sr-only", "More" }
        }
    }
}
