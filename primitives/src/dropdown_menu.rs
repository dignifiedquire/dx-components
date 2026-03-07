//! DropdownMenu primitive — matches Radix UI DropdownMenu structure.
//!
//! - [`DropdownMenuRoot`] (aliased as [`DropdownMenu`]): No DOM, pure context provider
//! - [`DropdownMenuTrigger`]: Button that toggles the menu
//! - [`DropdownMenuContent`]: Menu container with `role="menu"`
//! - [`DropdownMenuItem`]: Individual item with `role="menuitem"`
//! - [`DropdownMenuSeparator`]: Visual separator with `role="separator"`
//! - [`DropdownMenuLabel`]: Non-interactive label
//! - [`DropdownMenuGroup`]: Grouping element with `role="group"`
//! - [`DropdownMenuShortcut`]: Keyboard shortcut hint

use std::rc::Rc;

use crate::{
    focus::{use_focus_controlled_item, use_focus_provider, FocusState},
    merge_attributes, use_animated_open, use_controlled, use_unique_id,
};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct DropdownMenuCtx {
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: bool,
    focus: FocusState,
    trigger_id: Signal<String>,
    content_id: Signal<String>,
}

// ---------------------------------------------------------------------------
// DropdownMenuRoot (no DOM — pure context provider)
// ---------------------------------------------------------------------------

/// Props for [`DropdownMenuRoot`].
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuRootProps {
    /// Controlled open state.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// Default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Whether the dropdown menu is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether focus should loop when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Children (should include [`DropdownMenuTrigger`] and [`DropdownMenuContent`]).
    pub children: Element,
}

/// No-DOM context provider for a dropdown menu.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dropdown_menu::{DropdownMenuRoot, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         DropdownMenuRoot {
///             DropdownMenuTrigger { "Open" }
///             DropdownMenuContent {
///                 DropdownMenuItem { index: 0usize, "Edit" }
///                 DropdownMenuItem { index: 1usize, "Delete" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn DropdownMenuRoot(props: DropdownMenuRootProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let trigger_id = use_unique_id();
    let content_id = use_unique_id();
    let focus = use_focus_provider(props.roving_loop);

    let ctx = use_context_provider(|| DropdownMenuCtx {
        open,
        set_open,
        disabled: props.disabled,
        focus,
        trigger_id,
        content_id,
    });

    // Sync focus state with open state
    use_effect(move || {
        let focused = focus.any_focused();
        if *ctx.open.peek() != focused {
            (ctx.set_open)(focused);
        }
    });

    rsx! { {props.children} }
}

/// Backward-compatible alias for [`DropdownMenuRoot`].
#[component]
pub fn DropdownMenu(props: DropdownMenuRootProps) -> Element {
    DropdownMenuRoot(props)
}

// ---------------------------------------------------------------------------
// DropdownMenuTrigger
// ---------------------------------------------------------------------------

/// Props for [`DropdownMenuTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuTriggerProps {
    /// Render the trigger as a custom element (asChild pattern).
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,

    /// Additional attributes for the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children of the trigger.
    pub children: Element,
}

/// The trigger button. Renders as `<button>` with `aria-haspopup="menu"`.
#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    let mut ctx: DropdownMenuCtx = use_context();
    let mut element = use_signal(|| None::<Rc<MountedData>>);

    let is_open = (ctx.open)();

    let base = attributes!(button {
        id: ctx.trigger_id,
        r#type: "button",
        "data-slot": "dropdown-menu-trigger",
        "data-state": if is_open { "open" } else { "closed" },
        "data-disabled": if ctx.disabled { "true" } else { "" },
        disabled: ctx.disabled,
        aria_expanded: is_open,
        aria_haspopup: "menu",
        aria_controls: if is_open { Some(ctx.content_id.cloned()) } else { None },
        onmounted: move |e: MountedEvent| {
            element.set(Some(e.data()));
        },
        onclick: move |_| {
            if ctx.disabled {
                return;
            }
            let new_open = !is_open;
            ctx.set_open.call(new_open);
            if let Some(data) = element() {
                spawn(async move {
                    _ = data.set_focus(true).await;
                });
            }
        },
        onkeydown: move |event: Event<KeyboardData>| {
            if ctx.disabled {
                return;
            }
            match event.key() {
                key if key == Key::Enter || key == Key::Character(" ".to_string()) => {
                    event.prevent_default();
                    let new_open = !(ctx.open)();
                    ctx.set_open.call(new_open);
                }
                Key::ArrowDown => {
                    ctx.set_open.call(true);
                    ctx.focus.focus_first();
                    event.prevent_default();
                }
                _ => {}
            }
        },
        onblur: move |_| {
            if !ctx.focus.any_focused() {
                ctx.focus.blur();
            }
        },
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    if let Some(dynamic) = props.r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            button { ..merged, {props.children} }
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuContent
// ---------------------------------------------------------------------------

/// Props for [`DropdownMenuContent`].
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuContentProps {
    /// Additional attributes for the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (should include [`DropdownMenuItem`] components).
    pub children: Element,
}

/// The menu content container. Has `role="menu"`.
#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element {
    let mut ctx: DropdownMenuCtx = use_context();
    let id = ctx.content_id;

    let render = use_animated_open(id, ctx.open);

    rsx! {
        if render() {
            div {
                id,
                role: "menu",
                "data-slot": "dropdown-menu-content",
                "data-state": if (ctx.open)() { "open" } else { "closed" },
                aria_orientation: "vertical",
                aria_labelledby: ctx.trigger_id.cloned(),
                onkeydown: move |event: Event<KeyboardData>| {
                    match event.key() {
                        Key::Escape => {
                            ctx.set_open.call(false);
                            event.prevent_default();
                        }
                        Key::ArrowDown => {
                            ctx.focus.focus_next();
                            event.prevent_default();
                        }
                        Key::ArrowUp => {
                            ctx.focus.focus_prev();
                            event.prevent_default();
                        }
                        Key::Home => {
                            ctx.focus.focus_first();
                            event.prevent_default();
                        }
                        Key::End => {
                            ctx.focus.focus_last();
                            event.prevent_default();
                        }
                        _ => {}
                    }
                },
                onpointerdown: move |event| {
                    event.prevent_default();
                    event.stop_propagation();
                },
                ..props.attributes,
                {props.children}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuItem
// ---------------------------------------------------------------------------

/// Props for [`DropdownMenuItem`].
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuItemProps {
    /// The index of the item for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// Whether the item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Called when the item is selected (click or Enter/Space).
    #[props(default)]
    pub on_select: EventHandler<()>,

    /// Additional attributes for the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children of the item.
    pub children: Element,
}

/// A menu item. Has `role="menuitem"`.
#[component]
pub fn DropdownMenuItem(props: DropdownMenuItemProps) -> Element {
    let mut ctx: DropdownMenuCtx = use_context();

    let item_disabled = props.disabled || ctx.disabled;
    let focused = move || ctx.focus.is_focused((props.index)());
    let onmounted = use_focus_controlled_item(props.index);

    rsx! {
        div {
            role: "menuitem",
            "data-slot": "dropdown-menu-item",
            "data-disabled": if item_disabled { "true" } else { "" },
            "data-highlighted": if focused() { "" } else { None::<&str> },
            aria_disabled: if item_disabled { Some("true") } else { None },
            tabindex: if focused() { "0" } else { "-1" },
            onmounted,
            onclick: move |e: Event<MouseData>| {
                e.stop_propagation();
                if !item_disabled {
                    props.on_select.call(());
                    ctx.set_open.call(false);
                }
            },
            onkeydown: move |event: Event<KeyboardData>| {
                let key = event.key();
                if key == Key::Enter || key == Key::Character(" ".to_string()) {
                    if !item_disabled {
                        props.on_select.call(());
                        ctx.set_open.call(false);
                    }
                    event.prevent_default();
                    event.stop_propagation();
                }
            },
            onblur: move |_| {
                if focused() {
                    ctx.focus.blur();
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuSeparator
// ---------------------------------------------------------------------------

/// Props for [`DropdownMenuSeparator`].
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuSeparatorProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A visual separator between menu items. Has `role="separator"`.
#[component]
pub fn DropdownMenuSeparator(props: DropdownMenuSeparatorProps) -> Element {
    rsx! {
        div {
            role: "separator",
            "data-slot": "dropdown-menu-separator",
            aria_orientation: "horizontal",
            ..props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuLabel
// ---------------------------------------------------------------------------

/// Props for [`DropdownMenuLabel`].
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuLabelProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A non-interactive label within the menu.
#[component]
pub fn DropdownMenuLabel(props: DropdownMenuLabelProps) -> Element {
    rsx! {
        div {
            "data-slot": "dropdown-menu-label",
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuGroup
// ---------------------------------------------------------------------------

/// Props for [`DropdownMenuGroup`].
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuGroupProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A grouping element for menu items. Has `role="group"`.
#[component]
pub fn DropdownMenuGroup(props: DropdownMenuGroupProps) -> Element {
    rsx! {
        div {
            role: "group",
            "data-slot": "dropdown-menu-group",
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuShortcut
// ---------------------------------------------------------------------------

/// Props for [`DropdownMenuShortcut`].
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuShortcutProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (the shortcut text).
    pub children: Element,
}

/// A keyboard shortcut hint displayed alongside a menu item.
#[component]
pub fn DropdownMenuShortcut(props: DropdownMenuShortcutProps) -> Element {
    rsx! {
        span {
            "data-slot": "dropdown-menu-shortcut",
            ..props.attributes,
            {props.children}
        }
    }
}
