//! ContextMenu primitive — matches Radix UI ContextMenu structure.
//!
//! - [`ContextMenuRoot`] (aliased as [`ContextMenu`]): No DOM, pure context provider
//! - [`ContextMenuTrigger`]: Span element that opens menu on right-click
//! - [`ContextMenuContent`]: Menu container with `role="menu"`, positioned at click coordinates
//! - [`ContextMenuItem`]: Individual item with `role="menuitem"`
//! - [`ContextMenuSeparator`]: Visual separator with `role="separator"`
//! - [`ContextMenuLabel`]: Non-interactive label
//! - [`ContextMenuGroup`]: Grouping element with `role="group"`
//! - [`ContextMenuShortcut`]: Keyboard shortcut hint

use crate::{
    focus::{use_focus_controlled_item, use_focus_provider, FocusState},
    use_animated_open, use_controlled, use_unique_id,
};
use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct ContextMenuCtx {
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: bool,
    focus: FocusState,
    position: Signal<(i32, i32)>,
    content_id: Signal<String>,
}

// ---------------------------------------------------------------------------
// ContextMenuRoot (no DOM — pure context provider)
// ---------------------------------------------------------------------------

/// Props for [`ContextMenuRoot`].
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuRootProps {
    /// Controlled open state.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// Default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Whether the context menu is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether focus should loop when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Children (should include [`ContextMenuTrigger`] and [`ContextMenuContent`]).
    pub children: Element,
}

/// No-DOM context provider for a context menu.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::context_menu::{ContextMenuRoot, ContextMenuTrigger, ContextMenuContent, ContextMenuItem};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ContextMenuRoot {
///             ContextMenuTrigger { "Right click here" }
///             ContextMenuContent {
///                 ContextMenuItem { index: 0usize, "Edit" }
///                 ContextMenuItem { index: 1usize, "Delete" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn ContextMenuRoot(props: ContextMenuRootProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let content_id = use_unique_id();
    let position = use_signal(|| (0, 0));
    let focus = use_focus_provider(props.roving_loop);

    let ctx = use_context_provider(|| ContextMenuCtx {
        open,
        set_open,
        disabled: props.disabled,
        focus,
        position,
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

/// Backward-compatible alias for [`ContextMenuRoot`].
#[component]
pub fn ContextMenu(props: ContextMenuRootProps) -> Element {
    ContextMenuRoot(props)
}

// ---------------------------------------------------------------------------
// ContextMenuTrigger
// ---------------------------------------------------------------------------

/// Props for [`ContextMenuTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuTriggerProps {
    /// Additional attributes for the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children of the trigger.
    pub children: Element,
}

/// The trigger element. Renders as `<span>` matching Radix `Primitive.span`.
///
/// Opens the context menu on right-click (`oncontextmenu`).
#[component]
pub fn ContextMenuTrigger(props: ContextMenuTriggerProps) -> Element {
    let mut ctx: ContextMenuCtx = use_context();

    let is_open = (ctx.open)();

    rsx! {
        span {
            "data-slot": "context-menu-trigger",
            "data-state": if is_open { "open" } else { "closed" },
            "data-disabled": if ctx.disabled { "true" } else { "" },
            oncontextmenu: move |event: Event<MouseData>| {
                if ctx.disabled {
                    return;
                }
                ctx.position.set((
                    event.data().client_coordinates().x as i32,
                    event.data().client_coordinates().y as i32,
                ));
                ctx.set_open.call(true);
                event.prevent_default();
            },
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuContent
// ---------------------------------------------------------------------------

/// Props for [`ContextMenuContent`].
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuContentProps {
    /// Additional attributes for the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (should include [`ContextMenuItem`] components).
    pub children: Element,
}

/// The menu content container. Has `role="menu"`, positioned at click coordinates.
#[component]
pub fn ContextMenuContent(props: ContextMenuContentProps) -> Element {
    let mut ctx: ContextMenuCtx = use_context();
    let id = ctx.content_id;
    let position = ctx.position;

    let render = use_animated_open(id, ctx.open);

    rsx! {
        if render() {
            div {
                id,
                role: "menu",
                "data-slot": "context-menu-content",
                "data-state": if (ctx.open)() { "open" } else { "closed" },
                aria_orientation: "vertical",
                position: "fixed",
                left: "{position().0}px",
                top: "{position().1}px",
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
// ContextMenuItem
// ---------------------------------------------------------------------------

/// Props for [`ContextMenuItem`].
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuItemProps {
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
pub fn ContextMenuItem(props: ContextMenuItemProps) -> Element {
    let mut ctx: ContextMenuCtx = use_context();

    let item_disabled = props.disabled || ctx.disabled;
    let focused = move || ctx.focus.is_focused((props.index)());
    let onmounted = use_focus_controlled_item(props.index);

    rsx! {
        div {
            role: "menuitem",
            "data-slot": "context-menu-item",
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
// ContextMenuSeparator
// ---------------------------------------------------------------------------

/// Props for [`ContextMenuSeparator`].
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuSeparatorProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A visual separator between menu items. Has `role="separator"`.
#[component]
pub fn ContextMenuSeparator(props: ContextMenuSeparatorProps) -> Element {
    rsx! {
        div {
            role: "separator",
            "data-slot": "context-menu-separator",
            aria_orientation: "horizontal",
            ..props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuLabel
// ---------------------------------------------------------------------------

/// Props for [`ContextMenuLabel`].
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuLabelProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A non-interactive label within the menu.
#[component]
pub fn ContextMenuLabel(props: ContextMenuLabelProps) -> Element {
    rsx! {
        div {
            "data-slot": "context-menu-label",
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuGroup
// ---------------------------------------------------------------------------

/// Props for [`ContextMenuGroup`].
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuGroupProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A grouping element for menu items. Has `role="group"`.
#[component]
pub fn ContextMenuGroup(props: ContextMenuGroupProps) -> Element {
    rsx! {
        div {
            role: "group",
            "data-slot": "context-menu-group",
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuShortcut
// ---------------------------------------------------------------------------

/// Props for [`ContextMenuShortcut`].
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuShortcutProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (the shortcut text).
    pub children: Element,
}

/// A keyboard shortcut hint displayed alongside a menu item.
#[component]
pub fn ContextMenuShortcut(props: ContextMenuShortcutProps) -> Element {
    rsx! {
        span {
            "data-slot": "context-menu-shortcut",
            ..props.attributes,
            {props.children}
        }
    }
}
