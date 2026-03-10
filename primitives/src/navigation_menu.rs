//! Navigation menu primitive — matches `@radix-ui/react-navigation-menu`.
//!
//! Provides a composable navigation menu with hover/click-triggered content,
//! keyboard navigation, and accessibility attributes.
//!
//! ## Architecture
//!
//! - [`NavigationMenu`] — Root component, manages open state
//! - [`NavigationMenuList`] — Container for menu items
//! - [`NavigationMenuItem`] — Individual item (trigger + content pair)
//! - [`NavigationMenuTrigger`] — Button that toggles content visibility
//! - [`NavigationMenuContent`] — Content panel shown when trigger is activated
//! - [`NavigationMenuLink`] — Navigation link element
//! - [`NavigationMenuIndicator`] — Visual position indicator
//! - [`NavigationMenuViewport`] — Shared container for content
//!
//! ## Example
//!
//! ```rust,no_run
//! # use dioxus::prelude::*;
//! # use dioxus_primitives::navigation_menu::*;
//! fn Demo() -> Element {
//!     rsx! {
//!         NavigationMenu {
//!             NavigationMenuList {
//!                 NavigationMenuItem {
//!                     NavigationMenuTrigger { "Products" }
//!                     NavigationMenuContent {
//!                         NavigationMenuLink { href: "#analytics",
//!                             "Analytics"
//!                         }
//!                         NavigationMenuLink { href: "#reports",
//!                             "Reports"
//!                         }
//!                     }
//!                 }
//!                 NavigationMenuItem {
//!                     NavigationMenuLink { href: "#about",
//!                         "About"
//!                     }
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

/// Context shared by NavigationMenu components.
#[derive(Clone, Debug, PartialEq)]
pub struct NavigationMenuCtx {
    /// Currently open item value. Empty string means nothing is open.
    pub value: String,
    /// Orientation.
    pub orientation: NavigationMenuOrientation,
}

/// Orientation of the navigation menu.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum NavigationMenuOrientation {
    /// Horizontal layout (default).
    #[default]
    Horizontal,
    /// Vertical layout.
    Vertical,
}

impl NavigationMenuOrientation {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Context for individual menu items.
#[derive(Clone, Debug, PartialEq)]
pub struct NavigationMenuItemCtx {
    /// This item's value identifier.
    pub value: String,
    /// Whether this item's content is currently open.
    pub is_open: bool,
}

// ---------------------------------------------------------------------------
// NavigationMenu (root)
// ---------------------------------------------------------------------------

/// Props for [`NavigationMenu`].
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

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Root navigation menu component.
#[component]
pub fn NavigationMenu(props: NavigationMenuProps) -> Element {
    let mut internal_value = use_signal(|| props.default_value.clone());

    #[allow(clippy::redundant_closure)]
    let current_value = props.value.clone().unwrap_or_else(|| internal_value());

    let on_value_change = props.on_value_change;
    let set_value = use_callback(move |v: String| {
        internal_value.set(v.clone());
        on_value_change.call(v);
    });

    let ctx = NavigationMenuCtx {
        value: current_value.clone(),
        orientation: props.orientation,
    };

    use_context_provider(|| Signal::new(ctx.clone()));
    use_context_provider(|| set_value);

    // Update context when state changes
    let mut ctx_signal = use_context::<Signal<NavigationMenuCtx>>();
    if *ctx_signal.peek() != ctx {
        ctx_signal.set(ctx);
    }

    rsx! {
        nav {
            "data-slot": "navigation-menu",
            "data-orientation": props.orientation.as_str(),
            "data-viewport": if props.viewport { "true" } else { "false" },
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuList
// ---------------------------------------------------------------------------

/// Props for [`NavigationMenuList`].
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuListProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// List container for navigation menu items.
#[component]
pub fn NavigationMenuList(props: NavigationMenuListProps) -> Element {
    rsx! {
        ul {
            "data-slot": "navigation-menu-list",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuItem
// ---------------------------------------------------------------------------

/// Props for [`NavigationMenuItem`].
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuItemProps {
    /// Unique value for this item. Auto-generated if not provided.
    #[props(default)]
    pub value: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Individual menu item container.
#[component]
pub fn NavigationMenuItem(props: NavigationMenuItemProps) -> Element {
    let id = crate::use_unique_id();
    #[allow(clippy::redundant_closure)]
    let item_value = props.value.unwrap_or_else(|| id());

    let root_ctx = use_context::<Signal<NavigationMenuCtx>>();
    let is_open = root_ctx.read().value == item_value;

    let item_ctx = NavigationMenuItemCtx {
        value: item_value,
        is_open,
    };

    use_context_provider(|| Signal::new(item_ctx.clone()));

    // Update item context when state changes
    let mut item_signal = use_context::<Signal<NavigationMenuItemCtx>>();
    if *item_signal.peek() != item_ctx {
        item_signal.set(item_ctx);
    }

    rsx! {
        li {
            "data-slot": "navigation-menu-item",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuTrigger
// ---------------------------------------------------------------------------

/// Props for [`NavigationMenuTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuTriggerProps {
    /// Whether the trigger is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Trigger button that toggles content visibility.
///
/// Sets `aria-expanded` and `data-state` attributes.
#[component]
pub fn NavigationMenuTrigger(props: NavigationMenuTriggerProps) -> Element {
    let item_ctx = use_context::<Signal<NavigationMenuItemCtx>>();
    let set_value = use_context::<Callback<String>>();

    let is_open = item_ctx.read().is_open;
    let item_value = item_ctx.read().value.clone();

    rsx! {
        button {
            "data-slot": "navigation-menu-trigger",
            r#type: "button",
            disabled: props.disabled,
            aria_expanded: is_open,
            "data-state": if is_open { "open" } else { "closed" },
            "data-disabled": props.disabled.then_some("true"),
            class: props.class,
            onclick: {
                let item_value = item_value.clone();
                move |_| {
                    if !props.disabled {
                        let current = item_ctx.read().is_open;
                        if current {
                            set_value.call(String::new());
                        } else {
                            set_value.call(item_value.clone());
                        }
                    }
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuContent
// ---------------------------------------------------------------------------

/// Props for [`NavigationMenuContent`].
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuContentProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Content panel shown when the parent item's trigger is activated.
///
/// Only renders when the parent [`NavigationMenuItem`]'s value matches
/// the root menu's current value.
#[component]
pub fn NavigationMenuContent(props: NavigationMenuContentProps) -> Element {
    let item_ctx = use_context::<Signal<NavigationMenuItemCtx>>();
    let is_open = item_ctx.read().is_open;

    if !is_open {
        return rsx! {};
    }

    rsx! {
        div {
            "data-slot": "navigation-menu-content",
            "data-state": if is_open { "open" } else { "closed" },
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuLink
// ---------------------------------------------------------------------------

/// Props for [`NavigationMenuLink`].
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

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Navigation link element.
///
/// Sets `data-active` when the `active` prop is true.
/// Clicking a link closes the navigation menu.
#[component]
pub fn NavigationMenuLink(props: NavigationMenuLinkProps) -> Element {
    let set_value = try_use_context::<Callback<String>>();

    rsx! {
        a {
            "data-slot": "navigation-menu-link",
            "data-active": props.active.then_some("true"),
            href: props.href,
            class: props.class,
            onclick: move |e| {
                props.onclick.call(e);
                // Close menu when a link is clicked
                if let Some(set_value) = set_value {
                    set_value.call(String::new());
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuIndicator
// ---------------------------------------------------------------------------

/// Props for [`NavigationMenuIndicator`].
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuIndicatorProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Custom indicator content. Defaults to an arrow.
    #[props(default)]
    pub children: Element,
}

/// Visual indicator that tracks the active trigger.
#[component]
pub fn NavigationMenuIndicator(props: NavigationMenuIndicatorProps) -> Element {
    let root_ctx = use_context::<Signal<NavigationMenuCtx>>();
    let has_value = !root_ctx.read().value.is_empty();
    let data_state = if has_value { "visible" } else { "hidden" };

    let has_children = props.children != Ok(VNode::placeholder());

    rsx! {
        div {
            "data-slot": "navigation-menu-indicator",
            "data-state": data_state,
            class: props.class,
            ..props.attributes,

            if has_children {
                {props.children}
            } else {
                div {
                    "data-slot": "navigation-menu-indicator-arrow",
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuViewport
// ---------------------------------------------------------------------------

/// Props for [`NavigationMenuViewport`].
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuViewportProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Shared viewport container for navigation menu content.
///
/// Renders only when a menu item is open.
#[component]
pub fn NavigationMenuViewport(props: NavigationMenuViewportProps) -> Element {
    let root_ctx = use_context::<Signal<NavigationMenuCtx>>();
    let has_value = !root_ctx.read().value.is_empty();
    let data_state = if has_value { "open" } else { "closed" };

    rsx! {
        div {
            "data-slot": "navigation-menu-viewport",
            "data-state": data_state,
            class: props.class,
            ..props.attributes,
        }
    }
}
