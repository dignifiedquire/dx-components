//! Menubar primitive — matches Radix UI Menubar structure.
//!
//! - [`MenubarRoot`] (aliased as [`Menubar`]): `<div role="menubar">` container
//! - [`MenubarMenu`]: No DOM, context provider for a single menu
//! - [`MenubarTrigger`]: Button that opens/closes a menu, `role="menuitem"`
//! - [`MenubarContent`]: Menu container with `role="menu"`
//! - [`MenubarItem`]: Individual item with `role="menuitem"`
//! - [`MenubarSeparator`]: Visual separator with `role="separator"`
//! - [`MenubarLabel`]: Non-interactive label
//! - [`MenubarGroup`]: Grouping element with `role="group"`
//! - [`MenubarShortcut`]: Keyboard shortcut hint

use crate::{
    focus::{use_focus_controlled_item, use_focus_entry, use_focus_provider, FocusState},
    use_animated_open, use_unique_id,
};
use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Root-level context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct MenubarCtx {
    open_menu: Signal<Option<usize>>,
    disabled: bool,
    focus: FocusState,
}

// ---------------------------------------------------------------------------
// Menu-level context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct MenubarMenuCtx {
    index: ReadSignal<usize>,
    is_open: Memo<bool>,
    disabled: bool,
    focus: FocusState,
    trigger_id: Signal<String>,
    content_id: Signal<String>,
}

// ---------------------------------------------------------------------------
// MenubarRoot
// ---------------------------------------------------------------------------

/// Props for [`MenubarRoot`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarRootProps {
    /// Whether the menubar is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether focus should loop when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes for the menubar element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (should include [`MenubarMenu`] components).
    pub children: Element,
}

/// The menubar container. Renders as `<div>` with `role="menubar"`.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::menubar::{
///     MenubarRoot, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem,
/// };
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         MenubarRoot {
///             MenubarMenu { index: 0usize,
///                 MenubarTrigger { "File" }
///                 MenubarContent {
///                     MenubarItem { index: 0usize, "New" }
///                     MenubarItem { index: 1usize, "Open" }
///                 }
///             }
///             MenubarMenu { index: 1usize,
///                 MenubarTrigger { "Edit" }
///                 MenubarContent {
///                     MenubarItem { index: 0usize, "Cut" }
///                     MenubarItem { index: 1usize, "Copy" }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn MenubarRoot(props: MenubarRootProps) -> Element {
    let open_menu = use_signal(|| None::<usize>);
    let focus = use_focus_provider(props.roving_loop);

    let mut ctx = use_context_provider(|| MenubarCtx {
        open_menu,
        disabled: props.disabled,
        focus,
    });

    // Sync focus with open menu
    use_effect(move || {
        let index = ctx.focus.current_focus();
        if ctx.open_menu.peek().is_some() {
            ctx.open_menu.set(index);
        }
    });

    rsx! {
        div {
            role: "menubar",
            "data-slot": "menubar",
            "data-disabled": if props.disabled { "true" } else { "" },
            tabindex: if !ctx.focus.any_focused() { "0" } else { "-1" },
            onfocus: move |_| {
                ctx.focus.set_focus(Some(ctx.focus.recent_focus_or_default()));
            },
            ..props.attributes,
            {props.children}
        }
    }
}

/// Backward-compatible alias for [`MenubarRoot`].
#[component]
pub fn Menubar(props: MenubarRootProps) -> Element {
    MenubarRoot(props)
}

// ---------------------------------------------------------------------------
// MenubarMenu (no DOM — pure context provider)
// ---------------------------------------------------------------------------

/// Props for [`MenubarMenu`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarMenuProps {
    /// The index of this menu in the menubar (for keyboard navigation between triggers).
    pub index: ReadSignal<usize>,

    /// Whether this menu is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Children (should include [`MenubarTrigger`] and [`MenubarContent`]).
    pub children: Element,
}

/// No-DOM context provider for a single menu within the menubar.
#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    let ctx: MenubarCtx = use_context();

    let is_open = use_memo(move || (ctx.open_menu)() == Some((props.index)()));
    let focus = use_focus_provider(ctx.focus.roving_loop);
    let trigger_id = use_unique_id();
    let content_id = use_unique_id();

    let mut menu_ctx = use_context_provider(|| MenubarMenuCtx {
        index: props.index,
        is_open,
        disabled: props.disabled,
        focus,
        trigger_id,
        content_id,
    });

    // Blur items when menu closes
    use_effect(move || {
        if !is_open() {
            menu_ctx.focus.blur();
        }
    });

    // Register this menu as a focus entry at the menubar level
    use_focus_entry(ctx.focus, props.index);

    rsx! { {props.children} }
}

// ---------------------------------------------------------------------------
// MenubarTrigger
// ---------------------------------------------------------------------------

/// Props for [`MenubarTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarTriggerProps {
    /// Additional attributes for the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children of the trigger.
    pub children: Element,
}

/// The trigger button for a menu. Has `role="menuitem"`, `aria-haspopup="menu"`.
#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    let mut ctx: MenubarCtx = use_context();
    let menu_ctx: MenubarMenuCtx = use_context();
    let onmounted = crate::focus::use_focus_control(ctx.focus, menu_ctx.index);

    let disabled = ctx.disabled || menu_ctx.disabled;
    let is_open = menu_ctx.is_open;
    let index = menu_ctx.index;

    let is_focused = move || {
        ctx.focus.current_focus() == Some((menu_ctx.index)()) && !menu_ctx.focus.any_focused()
    };

    rsx! {
        button {
            id: menu_ctx.trigger_id,
            r#type: "button",
            role: "menuitem",
            "data-slot": "menubar-trigger",
            "data-state": if is_open() { "open" } else { "closed" },
            "data-highlighted": if is_focused() { "" } else { None::<&str> },
            "data-disabled": if disabled { "true" } else { "" },
            disabled: disabled,
            aria_haspopup: "menu",
            aria_expanded: is_open(),
            aria_controls: if is_open() { Some(menu_ctx.content_id.cloned()) } else { None },
            tabindex: if is_focused() { "0" } else { "-1" },
            onmounted,
            onpointerup: move |_| {
                if !disabled {
                    let new_open = if is_open() { None } else { Some((index)()) };
                    ctx.open_menu.set(new_open);
                    ctx.focus.set_focus(Some((index)()));
                }
            },
            onmouseenter: move |_| {
                if !disabled && (ctx.open_menu)().is_some() {
                    ctx.focus.set_focus(Some((index)()));
                }
            },
            onkeydown: move |event: Event<KeyboardData>| {
                if disabled {
                    return;
                }
                match event.key() {
                    Key::ArrowDown => {
                        if !is_open() {
                            ctx.open_menu.set(Some((index)()));
                        }
                        event.prevent_default();
                    }
                    Key::ArrowLeft => {
                        ctx.focus.focus_prev();
                        event.prevent_default();
                    }
                    Key::ArrowRight => {
                        ctx.focus.focus_next();
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
                    key if key == Key::Enter || key == Key::Character(" ".to_string()) => {
                        let new_open = if is_open() { None } else { Some((index)()) };
                        ctx.open_menu.set(new_open);
                        event.prevent_default();
                    }
                    _ => {}
                }
            },
            onblur: move |_| {
                if is_focused() {
                    ctx.focus.set_focus(None);
                    ctx.open_menu.set(None);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarContent
// ---------------------------------------------------------------------------

/// Props for [`MenubarContent`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarContentProps {
    /// Additional attributes for the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (should include [`MenubarItem`] components).
    pub children: Element,
}

/// The menu content container. Has `role="menu"`, `aria-labelledby` linking to trigger.
#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    let mut ctx: MenubarCtx = use_context();
    let mut menu_ctx: MenubarMenuCtx = use_context();
    let id = menu_ctx.content_id;

    let render = use_animated_open(id, menu_ctx.is_open);

    rsx! {
        if render() {
            div {
                id,
                role: "menu",
                "data-slot": "menubar-content",
                "data-state": if (menu_ctx.is_open)() { "open" } else { "closed" },
                aria_orientation: "vertical",
                aria_labelledby: menu_ctx.trigger_id.cloned(),
                onkeydown: move |event: Event<KeyboardData>| {
                    match event.key() {
                        Key::Escape => {
                            ctx.open_menu.set(None);
                            event.prevent_default();
                        }
                        Key::ArrowDown => {
                            menu_ctx.focus.focus_next();
                            event.prevent_default();
                        }
                        Key::ArrowUp => {
                            menu_ctx.focus.focus_prev();
                            event.prevent_default();
                        }
                        Key::ArrowLeft => {
                            ctx.focus.focus_prev();
                            event.prevent_default();
                        }
                        Key::ArrowRight => {
                            ctx.focus.focus_next();
                            event.prevent_default();
                        }
                        Key::Home => {
                            menu_ctx.focus.focus_first();
                            event.prevent_default();
                        }
                        Key::End => {
                            menu_ctx.focus.focus_last();
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
// MenubarItem
// ---------------------------------------------------------------------------

/// Props for [`MenubarItem`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarItemProps {
    /// The index of the item for keyboard navigation within this menu.
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
pub fn MenubarItem(props: MenubarItemProps) -> Element {
    let mut ctx: MenubarCtx = use_context();
    let mut menu_ctx: MenubarMenuCtx = use_context();

    let item_disabled = props.disabled || ctx.disabled || menu_ctx.disabled;
    let focused = move || menu_ctx.focus.is_focused((props.index)()) && (menu_ctx.is_open)();
    let onmounted = use_focus_controlled_item(props.index);

    rsx! {
        div {
            role: "menuitem",
            "data-slot": "menubar-item",
            "data-disabled": if item_disabled { "true" } else { "" },
            "data-highlighted": if focused() { "" } else { None::<&str> },
            aria_disabled: if item_disabled { Some("true") } else { None },
            tabindex: if focused() { "0" } else { "-1" },
            onmounted,
            onclick: move |e: Event<MouseData>| {
                e.stop_propagation();
                if !item_disabled {
                    props.on_select.call(());
                    ctx.open_menu.set(None);
                }
            },
            onkeydown: move |event: Event<KeyboardData>| {
                let key = event.key();
                if key == Key::Enter || key == Key::Character(" ".to_string()) {
                    if !item_disabled {
                        props.on_select.call(());
                        ctx.open_menu.set(None);
                    }
                    event.prevent_default();
                    event.stop_propagation();
                }
            },
            onblur: move |_| {
                if focused() {
                    menu_ctx.focus.blur();
                    ctx.focus.set_focus(None);
                    ctx.open_menu.set(None);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarSeparator
// ---------------------------------------------------------------------------

/// Props for [`MenubarSeparator`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarSeparatorProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A visual separator between menu items. Has `role="separator"`.
#[component]
pub fn MenubarSeparator(props: MenubarSeparatorProps) -> Element {
    rsx! {
        div {
            role: "separator",
            "data-slot": "menubar-separator",
            aria_orientation: "horizontal",
            ..props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarLabel
// ---------------------------------------------------------------------------

/// Props for [`MenubarLabel`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarLabelProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A non-interactive label within a menu.
#[component]
pub fn MenubarLabel(props: MenubarLabelProps) -> Element {
    rsx! {
        div {
            "data-slot": "menubar-label",
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarGroup
// ---------------------------------------------------------------------------

/// Props for [`MenubarGroup`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarGroupProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A grouping element for menu items. Has `role="group"`.
#[component]
pub fn MenubarGroup(props: MenubarGroupProps) -> Element {
    rsx! {
        div {
            role: "group",
            "data-slot": "menubar-group",
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarShortcut
// ---------------------------------------------------------------------------

/// Props for [`MenubarShortcut`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarShortcutProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (the shortcut text).
    pub children: Element,
}

/// A keyboard shortcut hint displayed alongside a menu item.
#[component]
pub fn MenubarShortcut(props: MenubarShortcutProps) -> Element {
    rsx! {
        span {
            "data-slot": "menubar-shortcut",
            ..props.attributes,
            {props.children}
        }
    }
}
