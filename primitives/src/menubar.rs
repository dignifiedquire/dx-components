//! Menubar primitive — matches Radix UI Menubar + shadcn exports.
//!
//! Custom Root, Menu, Trigger, and Content; all other components re-exported
//! from the shared `menu` base.
//!
//! ## Exports (16, matching shadcn)
//!
//! - [`Menubar`] / [`MenubarRoot`]
//! - [`MenubarMenu`]
//! - [`MenubarTrigger`]
//! - [`MenubarContent`] (wrapper with arrow key menu switching)
//! - [`MenubarItem`] (re-export)
//! - [`MenubarCheckboxItem`] (re-export)
//! - [`MenubarRadioGroup`] (re-export)
//! - [`MenubarRadioItem`] (re-export)
//! - [`MenubarItemIndicator`] (re-export)
//! - [`MenubarSeparator`] (re-export)
//! - [`MenubarLabel`] (re-export)
//! - [`MenubarGroup`] (re-export)
//! - [`MenubarShortcut`] (re-export)
//! - [`MenubarSub`] (re-export)
//! - [`MenubarSubTrigger`] (re-export)
//! - [`MenubarSubContent`] (re-export)
//! - [`MenubarPortal`] (re-export)

use std::rc::Rc;

use crate::direction::Orientation;
use crate::menu::MenuCtx;
use crate::popper::{Align, Popper, PopperContent, PopperCtx, Side};
use crate::presence::Presence;
use crate::roving_focus::{RovingFocusGroup, RovingFocusGroupItem, RovingFocusSlotProps};
use crate::{
    merge_attributes, use_global_escape_listener, use_id_or, use_outside_click_with_exclude,
    use_refocus_on_close, use_unique_id,
};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use dioxus_attributes::attributes;

// ---------------------------------------------------------------------------
// Re-exports from menu base (13 components)
// ---------------------------------------------------------------------------

/// Checkbox menu item — re-export with menubar data-slot prefix.
pub use crate::menu::MenuCheckboxItem as MenubarCheckboxItem;
/// Grouping element — re-export with menubar data-slot prefix.
pub use crate::menu::MenuGroup as MenubarGroup;
/// Menu item — re-export with menubar data-slot prefix.
pub use crate::menu::MenuItem as MenubarItem;
/// Indicator for checkbox/radio items — re-export with menubar data-slot prefix.
pub use crate::menu::MenuItemIndicator as MenubarItemIndicator;
/// Non-interactive label — re-export with menubar data-slot prefix.
pub use crate::menu::MenuLabel as MenubarLabel;
/// Portal pass-through — re-export.
pub use crate::menu::MenuPortal as MenubarPortal;
/// Radio group — re-export with menubar data-slot prefix.
pub use crate::menu::MenuRadioGroup as MenubarRadioGroup;
/// Radio item — re-export with menubar data-slot prefix.
pub use crate::menu::MenuRadioItem as MenubarRadioItem;
/// Visual separator — re-export with menubar data-slot prefix.
pub use crate::menu::MenuSeparator as MenubarSeparator;
/// Keyboard shortcut hint — re-export with menubar data-slot prefix.
pub use crate::menu::MenuShortcut as MenubarShortcut;
/// Sub-menu context provider — re-export.
pub use crate::menu::MenuSub as MenubarSub;
/// Sub-menu content — re-export with menubar data-slot prefix.
pub use crate::menu::MenuSubContent as MenubarSubContent;
/// Sub-menu trigger — re-export with menubar data-slot prefix.
pub use crate::menu::MenuSubTrigger as MenubarSubTrigger;

// ---------------------------------------------------------------------------
// Internal contexts
// ---------------------------------------------------------------------------

/// A registered menu in the menubar (for arrow key menu switching).
#[derive(Clone)]
struct RegisteredMenu {
    menu_id: String,
    trigger_element: Signal<Option<Rc<MountedData>>>,
}

/// Root-level context shared among all MenubarMenu/Trigger/Content.
#[derive(Clone, Copy)]
struct MenubarInternalCtx {
    /// Which menu is currently open (by menu_id), or None.
    open_menu_id: Signal<Option<String>>,
    disabled: bool,
    /// Ordered list of registered menus (for ArrowLeft/Right switching).
    menus: Signal<Vec<RegisteredMenu>>,
}

impl MenubarInternalCtx {
    /// Navigate to the prev/next menu. `delta` is -1 for prev, +1 for next.
    fn navigate(&mut self, delta: i32) {
        let menus = self.menus.read();
        if menus.is_empty() {
            return;
        }
        let current_id = (self.open_menu_id)();
        let current_idx = current_id
            .as_ref()
            .and_then(|id| menus.iter().position(|m| m.menu_id == *id))
            .unwrap_or(0);
        let new_idx = ((current_idx as i32 + delta).rem_euclid(menus.len() as i32)) as usize;
        let new_menu = &menus[new_idx];
        self.open_menu_id.set(Some(new_menu.menu_id.clone()));
        // Focus the new trigger
        if let Some(el) = (new_menu.trigger_element)() {
            spawn(async move {
                _ = el.set_focus(true).await;
            });
        }
    }
}

/// Per-menu context for trigger ↔ content communication.
#[derive(Clone, Copy)]
struct MenubarMenuInternalCtx {
    menu_id: Signal<String>,
    trigger_element: Signal<Option<Rc<MountedData>>>,
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

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

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
///             MenubarMenu {
///                 MenubarTrigger { "File" }
///                 MenubarContent {
///                     MenubarItem { "New" }
///                     MenubarItem { "Open" }
///                 }
///             }
///             MenubarMenu {
///                 MenubarTrigger { "Edit" }
///                 MenubarContent {
///                     MenubarItem { "Cut" }
///                     MenubarItem { "Copy" }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn MenubarRoot(props: MenubarRootProps) -> Element {
    use_context_provider(|| MenubarInternalCtx {
        open_menu_id: Signal::new(None),
        disabled: props.disabled,
        menus: Signal::new(Vec::new()),
    });

    let children = props.children;
    let class = props.class;
    let user_attrs = props.attributes;
    let disabled = props.disabled;

    rsx! {
        RovingFocusGroup {
            orientation: Signal::new(Some(Orientation::Horizontal)),
            r#loop: Signal::new(true),
            r#as: {
                let children = children.clone();
                let class = class.clone();
                let user_attrs = user_attrs.clone();
                move |roving_attrs: Vec<Attribute>| {
                    let bar_attrs = attributes!(div {
                        role: "menubar",
                        "data-slot": "menubar",
                        "data-disabled": if disabled { Some("true") } else { None },
                        class: class.clone(),
                    });
                    let merged = merge_attributes(vec![roving_attrs, bar_attrs, user_attrs.clone()]);

                    rsx! {
                        div { ..merged, {children.clone()} }
                    }
                }
            },
        }
    }
}

/// Backward-compatible alias for [`MenubarRoot`].
#[component]
pub fn Menubar(props: MenubarRootProps) -> Element {
    MenubarRoot(props)
}

// ---------------------------------------------------------------------------
// MenubarMenu (no DOM — per-menu context provider)
// ---------------------------------------------------------------------------

/// Props for [`MenubarMenu`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarMenuProps {
    /// Whether this menu is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Children (should include [`MenubarTrigger`] and [`MenubarContent`]).
    pub children: Element,
}

/// No-DOM context provider for a single menu within the menubar.
#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    let mut bar_ctx: MenubarInternalCtx = use_context();
    let menu_id = use_unique_id();
    let trigger_id = use_unique_id();
    let content_id = use_unique_id();
    let trigger_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let is_open = use_memo(move || {
        (bar_ctx.open_menu_id)()
            .as_ref()
            .is_some_and(|id| id == &(menu_id)())
    });

    let mut open_menu_sig = bar_ctx.open_menu_id;
    let on_close = Callback::new(move |()| {
        open_menu_sig.set(None);
    });

    // Provide MenuCtx for the shared menu base components (MenuItem, etc.)
    let typeahead_items = use_signal(Vec::new);
    let grace_intent = use_signal(|| None);
    use_context_provider(|| MenuCtx {
        open: is_open,
        on_close,
        content_id,
        trigger_id,
        slot_prefix: "menubar",
        typeahead_items,
        grace_intent,
    });

    use_context_provider(|| MenubarMenuInternalCtx {
        menu_id,
        trigger_element,
    });

    // Register this menu in the menubar's list
    let menu_id_str = (menu_id)();
    use_effect(move || {
        let entry = RegisteredMenu {
            menu_id: menu_id_str.clone(),
            trigger_element,
        };
        bar_ctx.menus.write().push(entry);
    });

    // Unregister on drop
    let menu_id_cleanup = (menu_id)();
    use_drop(move || {
        bar_ctx
            .menus
            .write()
            .retain(|m| m.menu_id != menu_id_cleanup);
    });

    rsx! {
        Popper {
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarTrigger
// ---------------------------------------------------------------------------

/// Props for [`MenubarTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarTriggerProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes for the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children of the trigger.
    pub children: Element,
}

/// The trigger button for a menu. Has `role="menuitem"`, `aria-haspopup="menu"`.
///
/// Wrapped in `RovingFocusGroupItem` for horizontal keyboard navigation.
#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    let mut bar_ctx: MenubarInternalCtx = use_context();
    let ctx: MenuCtx = use_context();
    let mut menu_ctx: MenubarMenuInternalCtx = use_context();
    let popper_ctx: PopperCtx = use_context();

    let disabled = bar_ctx.disabled;
    let is_open = ctx.open;
    let trigger_id = ctx.trigger_id;
    let content_id = ctx.content_id;
    let menu_id = menu_ctx.menu_id;
    let class = props.class;
    let user_attrs = props.attributes;
    let children = props.children;
    let mut is_focused = use_signal(|| false);

    rsx! {
        RovingFocusGroupItem {
            focusable: !disabled,
            r#as: {
                let class = class.clone();
                let user_attrs = user_attrs.clone();
                let children = children.clone();
                move |slot_props: RovingFocusSlotProps| {
                    let trigger_attrs = attributes!(button {
                        id: trigger_id,
                        r#type: "button",
                        role: "menuitem",
                        "data-slot": "menubar-trigger",
                        "data-state": if is_open() { "open" } else { "closed" },
                        "data-highlighted": if is_focused() { Some("") } else { None },
                        "data-disabled": if disabled { Some("true") } else { None },
                        disabled: disabled,
                        aria_haspopup: "menu",
                        aria_expanded: is_open(),
                        aria_controls: if is_open() { Some(content_id.cloned()) } else { None },
                        class: class.clone(),
                    });
                    let merged = merge_attributes(vec![slot_props.attributes, trigger_attrs, user_attrs.clone()]);

                    rsx! {
                        button {
                            onmounted: move |e: MountedEvent| {
                                slot_props.on_mounted.call(e.clone());
                                let data = e.data();
                                menu_ctx.trigger_element.set(Some(data.clone()));
                                popper_ctx.set_anchor_ref(data);
                            },
                            onfocus: move |e| {
                                is_focused.set(true);
                                slot_props.on_focus.call(e);
                            },
                            onfocusout: move |_| {
                                is_focused.set(false);
                            },
                            onmousedown: move |e| slot_props.on_mousedown.call(e),
                            onpointerdown: move |event: Event<PointerData>| {
                                // Upstream: only react to left-click without Ctrl (menubar.tsx:243-251)
                                if disabled
                                    || event.trigger_button() != Some(MouseButton::Primary)
                                    || event.modifiers().ctrl()
                                {
                                    return;
                                }
                                event.prevent_default();
                                if is_open() {
                                    bar_ctx.open_menu_id.set(None);
                                } else {
                                    bar_ctx.open_menu_id.set(Some((menu_id)()));
                                }
                            },
                            onmouseenter: move |_| {
                                // Switch to this menu if another is already open
                                if !disabled && (bar_ctx.open_menu_id)().is_some() && !is_open() {
                                    bar_ctx.open_menu_id.set(Some((menu_id)()));
                                }
                            },
                            onkeydown: move |event: Event<KeyboardData>| {
                                if disabled {
                                    slot_props.on_keydown.call(event);
                                    return;
                                }
                                match event.key() {
                                    Key::ArrowDown => {
                                        if !is_open() {
                                            bar_ctx.open_menu_id.set(Some((menu_id)()));
                                        }
                                        event.prevent_default();
                                    }
                                    key if key == Key::Enter || key == Key::Character(" ".to_string()) => {
                                        if is_open() {
                                            bar_ctx.open_menu_id.set(None);
                                        } else {
                                            bar_ctx.open_menu_id.set(Some((menu_id)()));
                                        }
                                        event.prevent_default();
                                    }
                                    _ => {
                                        // Let RovingFocusGroup handle ArrowLeft/Right
                                        slot_props.on_keydown.call(event);
                                    }
                                }
                            },
                            ..merged,
                            {children.clone()}
                        }
                    }
                }
            },
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarContent (wrapper with arrow key menu switching)
// ---------------------------------------------------------------------------

/// Props for [`MenubarContent`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarContentProps {
    /// User-provided id override.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Keep content mounted even when closed.
    #[props(default)]
    pub force_mount: bool,

    /// Side of the trigger to place content. Defaults to `Bottom`.
    #[props(default)]
    pub side: Side,

    /// Offset from the trigger edge in pixels. Defaults to 0.
    #[props(default)]
    pub side_offset: f64,

    /// Alignment relative to the trigger. Defaults to `Start`.
    #[props(default = Align::Start)]
    pub align: Align,

    /// Offset along the alignment axis. Defaults to 0.
    #[props(default)]
    pub align_offset: f64,

    /// Whether to avoid viewport edge collisions. Defaults to `true`.
    #[props(default = true)]
    pub avoid_collisions: bool,

    /// Collision padding in pixels. Defaults to 0.
    #[props(default)]
    pub collision_padding: f64,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes for the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (should include menu items).
    pub children: Element,
}

/// Menu content for a menubar menu.
///
/// Owns presence tracking. Passes `class`, `data-state`, `data-slot` to
/// PopperContent's inner div. Delegates keyboard/focus to `MenuContent` with
/// ArrowLeft/Right handlers for switching between menus.
#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    let ctx: MenuCtx = use_context();
    let mut bar_ctx: MenubarInternalCtx = use_context();

    let id = use_id_or(ctx.content_id, props.id);

    // Refocus trigger when menu closes (upstream onCloseAutoFocus)
    use_refocus_on_close(ctx.open, ctx.trigger_id);

    // Document-level Escape listener
    use_global_escape_listener(move || {
        if *ctx.open.peek() {
            bar_ctx.open_menu_id.set(None);
        }
    });

    // Dismiss on click outside content (but not when clicking another menubar trigger)
    {
        let open = ctx.open;
        use_outside_click_with_exclude(id, "[data-slot=\"menubar-trigger\"]", move || {
            if *open.peek() {
                bar_ctx.open_menu_id.set(None);
            }
        });
    }

    let on_escape = Callback::new(move |()| {
        bar_ctx.open_menu_id.set(None);
    });
    let on_arrow_left = Callback::new(move |()| {
        bar_ctx.navigate(-1);
    });
    let on_arrow_right = Callback::new(move |()| {
        bar_ctx.navigate(1);
    });

    let data_state = if (ctx.open)() { "open" } else { "closed" };

    let content_attrs = attributes!(div {
        id: id,
        "data-slot": "menubar-content",
        "data-state": data_state,
    });
    let merged = merge_attributes(vec![content_attrs, props.attributes]);

    rsx! {
        Presence {
            present: props.force_mount || (ctx.open)(),
            id: id,
            PopperContent {
                side: props.side,
                side_offset: props.side_offset,
                align: props.align,
                align_offset: props.align_offset,
                avoid_collisions: props.avoid_collisions,
                collision_padding: props.collision_padding,
                css_var_prefix: "menubar",
                class: props.class,
                content_attributes: merged,

                crate::menu::MenuContent {
                    content_id: id,
                    on_escape_override: on_escape,
                    on_arrow_left: on_arrow_left,
                    on_arrow_right: on_arrow_right,
                    focus_exclude_selector: "[data-slot=\"menubar-trigger\"]",
                    {props.children}
                }
            }
        }
    }
}
