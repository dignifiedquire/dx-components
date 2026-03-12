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

use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

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

/// Internal mutable state for the navigation menu (not part of public API).
///
/// Holds signals for content measurement, trigger position tracking,
/// and hover delay timers.
#[derive(Clone, Copy)]
struct NavMenuState {
    /// Registered content dimensions: value → (width, height).
    content_sizes: Signal<HashMap<String, (f64, f64)>>,
    /// Registered trigger positions: value → (offset_from_list_start, size).
    /// For horizontal: (offsetLeft, width). For vertical: (offsetTop, height).
    trigger_rects: Signal<HashMap<String, (f64, f64)>>,
    /// Reference to the list element for computing relative trigger positions.
    list_ref: Signal<Option<Rc<MountedData>>>,
    /// Previous value (for viewport close animations).
    previous_value: Signal<String>,
    /// Generation counter for open-delay timer cancellation.
    open_gen: Signal<u64>,
    /// Generation counter for close-delay timer cancellation.
    close_gen: Signal<u64>,
    /// Whether the next open should be delayed (false during skip-delay window).
    is_open_delayed: Signal<bool>,
    /// Generation counter for skip-delay timer cancellation.
    skip_delay_gen: Signal<u64>,
    /// Delay before opening on hover (default 200ms, matching Radix).
    delay_duration: u64,
    /// Delay before closing when pointer leaves (150ms, matching Radix).
    close_delay: u64,
    /// Grace period after closing where next open has no delay (300ms, matching Radix).
    skip_delay_duration: u64,
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

    /// Delay in ms before opening on hover. Default 200 (matching Radix).
    #[props(default = 200)]
    pub delay_duration: u64,

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

    // Internal mutable state
    use_context_provider(|| NavMenuState {
        content_sizes: Signal::new(HashMap::new()),
        trigger_rects: Signal::new(HashMap::new()),
        list_ref: Signal::new(None),
        previous_value: Signal::new(String::new()),
        open_gen: Signal::new(0u64),
        close_gen: Signal::new(0u64),
        is_open_delayed: Signal::new(true),
        skip_delay_gen: Signal::new(0u64),
        delay_duration: props.delay_duration,
        close_delay: 150,
        skip_delay_duration: 300,
    });

    // Update context when state changes
    let mut ctx_signal = use_context::<Signal<NavigationMenuCtx>>();
    if *ctx_signal.peek() != ctx {
        // Track previous value for viewport close animation
        let mut state = use_context::<NavMenuState>();
        let prev = ctx_signal.peek().value.clone();
        if !prev.is_empty() && ctx.value.is_empty() {
            state.previous_value.set(prev);
        }
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
    let mut state = use_context::<NavMenuState>();

    rsx! {
        ul {
            "data-slot": "navigation-menu-list",
            class: props.class,
            onmounted: move |e| {
                state.list_ref.set(Some(e.data()));
            },
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
/// Supports hover-to-open with configurable delay (matching Radix's
/// `delayDuration`), keyboard navigation (ArrowLeft/Right between triggers,
/// ArrowDown to enter content), and position tracking for the indicator.
///
/// ## Radix deviation
/// Radix uses a `FocusGroupItem` wrapper and `focusFirst` utility for
/// keyboard navigation between triggers. We use `document::eval` to query
/// sibling trigger elements because Dioxus has no API for querying siblings
/// by data attribute in the DOM tree.
#[component]
pub fn NavigationMenuTrigger(props: NavigationMenuTriggerProps) -> Element {
    let item_ctx = use_context::<Signal<NavigationMenuItemCtx>>();
    let root_ctx = use_context::<Signal<NavigationMenuCtx>>();
    let set_value = use_context::<Callback<String>>();
    let mut state = use_context::<NavMenuState>();

    let is_open = item_ctx.read().is_open;
    let item_value = item_ctx.read().value.clone();
    let orientation = root_ctx.read().orientation;

    let mut trigger_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    // Register trigger position on mount and whenever it might change
    {
        let item_value = item_value.clone();
        use_effect(move || {
            let _is_open = is_open; // Subscribe to re-run when open state changes
            let item_value = item_value.clone();
            spawn(async move {
                // Small delay to let layout settle
                dioxus_sdk_time::sleep(Duration::from_millis(16)).await;
                if let Some(ref mounted) = *trigger_ref.peek() {
                    if let Ok(trigger_rect) = mounted.get_client_rect().await {
                        // Compute position relative to the list container
                        let list_offset = if let Some(ref list) = *state.list_ref.peek() {
                            list.get_client_rect().await.ok()
                        } else {
                            None
                        };

                        let (offset, size) = match orientation {
                            NavigationMenuOrientation::Horizontal => {
                                let list_left = list_offset.map_or(0.0, |r| r.origin.x);
                                (trigger_rect.origin.x - list_left, trigger_rect.size.width)
                            }
                            NavigationMenuOrientation::Vertical => {
                                let list_top = list_offset.map_or(0.0, |r| r.origin.y);
                                (trigger_rect.origin.y - list_top, trigger_rect.size.height)
                            }
                        };

                        state
                            .trigger_rects
                            .write()
                            .insert(item_value.clone(), (offset, size));
                    }
                }
            });
        });
    }

    // --- Hover delay timers (matching Radix's delayDuration / close behavior) ---

    let handle_open = {
        let item_value = item_value.clone();
        use_callback(move |()| {
            // Cancel any pending close timer
            let close_current = *state.close_gen.peek();
            state.close_gen.set(close_current + 1);

            if *state.is_open_delayed.peek() {
                // Delayed open
                let gen = *state.open_gen.peek() + 1;
                state.open_gen.set(gen);
                let delay = state.delay_duration;
                let open_gen = state.open_gen;
                let item_value = item_value.clone();
                spawn(async move {
                    dioxus_sdk_time::sleep(Duration::from_millis(delay)).await;
                    if *open_gen.peek() == gen {
                        set_value.call(item_value);
                    }
                });
            } else {
                // Immediate open (within skip-delay window)
                set_value.call(item_value.clone());
            }
        })
    };

    let handle_close = use_callback(move |()| {
        // Cancel any pending open timer
        let open_current = *state.open_gen.peek();
        state.open_gen.set(open_current + 1);

        // Start close timer
        let gen = *state.close_gen.peek() + 1;
        state.close_gen.set(gen);
        let close_delay = state.close_delay;
        let close_gen = state.close_gen;
        let skip_delay_duration = state.skip_delay_duration;
        let mut is_open_delayed = state.is_open_delayed;
        let mut skip_delay_gen = state.skip_delay_gen;
        spawn(async move {
            dioxus_sdk_time::sleep(Duration::from_millis(close_delay)).await;
            if *close_gen.peek() == gen {
                set_value.call(String::new());

                // Start skip-delay window: next open will be immediate
                is_open_delayed.set(false);
                let skip_gen = *skip_delay_gen.peek() + 1;
                skip_delay_gen.set(skip_gen);
                spawn(async move {
                    dioxus_sdk_time::sleep(Duration::from_millis(skip_delay_duration)).await;
                    if *skip_delay_gen.peek() == skip_gen {
                        is_open_delayed.set(true);
                    }
                });
            }
        });
    });

    let disabled = props.disabled;

    rsx! {
        button {
            "data-slot": "navigation-menu-trigger",
            r#type: "button",
            disabled: props.disabled,
            aria_expanded: is_open,
            "data-state": if is_open { "open" } else { "closed" },
            "data-disabled": props.disabled.then_some("true"),
            class: props.class,
            onmounted: move |e| {
                trigger_ref.set(Some(e.data()));
            },
            onclick: {
                let item_value = item_value.clone();
                move |_| {
                    if !disabled {
                        let current = item_ctx.read().is_open;
                        if current {
                            set_value.call(String::new());
                        } else {
                            set_value.call(item_value.clone());
                        }
                    }
                }
            },
            onpointerenter: move |e: PointerEvent| {
                // Only respond to mouse hover, not touch (matching Radix's whenMouse)
                if !disabled && e.pointer_type() == "mouse" {
                    handle_open.call(());
                }
            },
            onpointerleave: move |e: PointerEvent| {
                if !disabled && e.pointer_type() == "mouse" {
                    handle_close.call(());
                }
            },
            onkeydown: {
                let item_value = item_value.clone();
                move |e: KeyboardEvent| {
                    if disabled {
                        return;
                    }
                    let key = e.key();
                    match orientation {
                        NavigationMenuOrientation::Horizontal => {
                            match key {
                                Key::ArrowDown => {
                                    // Open content and focus first focusable element inside
                                    e.prevent_default();
                                    if !is_open {
                                        set_value.call(item_value.clone());
                                    }
                                    // Radix deviation: Radix uses itemContext.onEntryKeyDown
                                    // to focus into content. We rely on the content having
                                    // autofocus or the user tabbing into it after open.
                                }
                                Key::ArrowLeft | Key::ArrowRight => {
                                    e.prevent_default();
                                    // Radix deviation: Radix uses FocusGroupItem + focusFirst
                                    // to navigate between trigger siblings. We use eval to
                                    // query sibling triggers because Dioxus has no API for
                                    // querying siblings by data attribute.
                                    let dir = if key == Key::ArrowRight { "next" } else { "prev" };
                                    let js = format!(
                                        r#"(function() {{
                                            var triggers = Array.from(document.querySelectorAll('[data-slot="navigation-menu-trigger"]'));
                                            var current = document.activeElement;
                                            var idx = triggers.indexOf(current);
                                            if (idx < 0) return;
                                            var target = triggers[idx + ({dir} === "next" ? 1 : -1)];
                                            if (target) target.focus();
                                        }})()"#,
                                        dir = if dir == "next" { "\"next\"" } else { "\"prev\"" },
                                    );
                                    document::eval(&js);
                                }
                                Key::Home => {
                                    e.prevent_default();
                                    document::eval(
                                        r#"(function() {
                                            var t = document.querySelector('[data-slot="navigation-menu-trigger"]');
                                            if (t) t.focus();
                                        })()"#,
                                    );
                                }
                                Key::End => {
                                    e.prevent_default();
                                    document::eval(
                                        r#"(function() {
                                            var triggers = document.querySelectorAll('[data-slot="navigation-menu-trigger"]');
                                            if (triggers.length) triggers[triggers.length - 1].focus();
                                        })()"#,
                                    );
                                }
                                Key::Escape => {
                                    set_value.call(String::new());
                                }
                                _ => {}
                            }
                        }
                        NavigationMenuOrientation::Vertical => {
                            match key {
                                Key::ArrowRight => {
                                    e.prevent_default();
                                    if !is_open {
                                        set_value.call(item_value.clone());
                                    }
                                }
                                Key::ArrowUp | Key::ArrowDown => {
                                    e.prevent_default();
                                    let dir = if key == Key::ArrowDown { "next" } else { "prev" };
                                    let js = format!(
                                        r#"(function() {{
                                            var triggers = Array.from(document.querySelectorAll('[data-slot="navigation-menu-trigger"]'));
                                            var current = document.activeElement;
                                            var idx = triggers.indexOf(current);
                                            if (idx < 0) return;
                                            var target = triggers[idx + ({dir} === "next" ? 1 : -1)];
                                            if (target) target.focus();
                                        }})()"#,
                                        dir = if dir == "next" { "\"next\"" } else { "\"prev\"" },
                                    );
                                    document::eval(&js);
                                }
                                Key::Escape => {
                                    set_value.call(String::new());
                                }
                                _ => {}
                            }
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
/// Measures its own dimensions and registers them with the root context
/// so that [`NavigationMenuViewport`] can set CSS custom properties for
/// smooth size transitions.
///
/// ## Radix deviation
/// Radix uses a `ResizeObserver` to watch content dimensions. We measure
/// once on mount via `MountedData::get_client_rect()` because Dioxus does
/// not expose a ResizeObserver API.
#[component]
pub fn NavigationMenuContent(props: NavigationMenuContentProps) -> Element {
    let item_ctx = use_context::<Signal<NavigationMenuItemCtx>>();
    let mut state = use_context::<NavMenuState>();
    let set_value = use_context::<Callback<String>>();

    let is_open = item_ctx.read().is_open;
    let item_value = item_ctx.read().value.clone();

    // Measure content dimensions on mount
    let mut content_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    {
        let item_value = item_value.clone();
        use_effect(move || {
            if is_open {
                let item_value = item_value.clone();
                spawn(async move {
                    // Small delay to let layout settle
                    dioxus_sdk_time::sleep(Duration::from_millis(16)).await;
                    if let Some(ref mounted) = *content_ref.peek() {
                        if let Ok(rect) = mounted.get_client_rect().await {
                            state
                                .content_sizes
                                .write()
                                .insert(item_value.clone(), (rect.size.width, rect.size.height));
                        }
                    }
                });
            }
        });
    }

    // Deregister on unmount
    {
        let item_value = item_value.clone();
        crate::use_effect_cleanup(move || {
            state.content_sizes.write().remove(&item_value);
        });
    }

    if !is_open {
        return rsx! {};
    }

    rsx! {
        div {
            "data-slot": "navigation-menu-content",
            "data-state": if is_open { "open" } else { "closed" },
            class: props.class,
            onmounted: move |e| {
                content_ref.set(Some(e.data()));
            },
            // Pointer enter: cancel close timer (matching Radix's onContentEnter)
            onpointerenter: move |e: PointerEvent| {
                if e.pointer_type() == "mouse" {
                    let close_current = *state.close_gen.peek();
                    state.close_gen.set(close_current + 1);
                }
            },
            // Pointer leave: start close timer (matching Radix's onContentLeave)
            onpointerleave: move |e: PointerEvent| {
                if e.pointer_type() == "mouse" {
                    // Cancel any pending open timer
                    let open_current = *state.open_gen.peek();
                    state.open_gen.set(open_current + 1);

                    // Start close timer
                    let gen = *state.close_gen.peek() + 1;
                    state.close_gen.set(gen);
                    let close_delay = state.close_delay;
                    let close_gen = state.close_gen;
                    let skip_delay_duration = state.skip_delay_duration;
                    let mut is_open_delayed = state.is_open_delayed;
                    let mut skip_delay_gen = state.skip_delay_gen;
                    spawn(async move {
                        dioxus_sdk_time::sleep(Duration::from_millis(close_delay)).await;
                        if *close_gen.peek() == gen {
                            set_value.call(String::new());

                            is_open_delayed.set(false);
                            let skip_gen = *skip_delay_gen.peek() + 1;
                            skip_delay_gen.set(skip_gen);
                            spawn(async move {
                                dioxus_sdk_time::sleep(Duration::from_millis(skip_delay_duration))
                                    .await;
                                if *skip_delay_gen.peek() == skip_gen {
                                    is_open_delayed.set(true);
                                }
                            });
                        }
                    });
                }
            },
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

/// Visual indicator that tracks the active trigger position.
///
/// Sets positioning styles based on the active trigger's offset and size,
/// matching Radix's indicator behavior. Uses absolute positioning with
/// `transform: translateX/Y` for smooth transitions.
///
/// ## Radix deviation
/// Radix uses `ResizeObserver` on both the trigger and the indicator track
/// to detect size changes. We measure trigger positions in
/// [`NavigationMenuTrigger`] via `get_client_rect()` and read them here.
/// Position is relative to the [`NavigationMenuList`] container.
#[component]
pub fn NavigationMenuIndicator(props: NavigationMenuIndicatorProps) -> Element {
    let root_ctx = use_context::<Signal<NavigationMenuCtx>>();
    let state = use_context::<NavMenuState>();

    let value = root_ctx.read().value.clone();
    let has_value = !value.is_empty();
    let orientation = root_ctx.read().orientation;
    let data_state = if has_value { "visible" } else { "hidden" };

    // Look up the active trigger's position
    let position = state.trigger_rects.read().get(&value).copied();

    let has_children = props.children != Ok(VNode::placeholder());

    // Build inline style for positioning (matching Radix's indicator positioning)
    let style = if let Some((offset, size)) = position {
        match orientation {
            NavigationMenuOrientation::Horizontal => {
                format!(
                    "position: absolute; left: 0; width: {size}px; transform: translateX({offset}px);"
                )
            }
            NavigationMenuOrientation::Vertical => {
                format!(
                    "position: absolute; top: 0; height: {size}px; transform: translateY({offset}px);"
                )
            }
        }
    } else {
        String::new()
    };

    rsx! {
        div {
            "data-slot": "navigation-menu-indicator",
            "data-state": data_state,
            "data-orientation": orientation.as_str(),
            style: style,
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
/// Sets CSS custom properties for the active content's dimensions,
/// enabling smooth size transitions via CSS.
///
/// - `--radix-navigation-menu-viewport-width`: Width of active content in px.
/// - `--radix-navigation-menu-viewport-height`: Height of active content in px.
///
/// ## Radix deviation
/// Radix renders content elements inside the viewport and uses `ResizeObserver`
/// for dynamic sizing. We set CSS custom properties based on dimensions
/// measured by [`NavigationMenuContent`] on mount. The viewport itself is
/// a styled container that consumers can animate with CSS transitions on
/// width/height using the custom properties.
#[component]
pub fn NavigationMenuViewport(props: NavigationMenuViewportProps) -> Element {
    let root_ctx = use_context::<Signal<NavigationMenuCtx>>();
    let state = use_context::<NavMenuState>();

    let value = root_ctx.read().value.clone();
    let has_value = !value.is_empty();
    let data_state = if has_value { "open" } else { "closed" };

    // Use active value, or fall back to previous value for close animation
    let size_key = if has_value {
        value
    } else {
        state.previous_value.read().clone()
    };

    let sizes = state.content_sizes.read();
    let (width, height) = sizes.get(&size_key).copied().unwrap_or((0.0, 0.0));

    let style = format!(
        "--radix-navigation-menu-viewport-width: {width}px; --radix-navigation-menu-viewport-height: {height}px;"
    );

    rsx! {
        div {
            "data-slot": "navigation-menu-viewport",
            "data-state": data_state,
            style: style,
            class: props.class,
            ..props.attributes,
        }
    }
}
