//! DropdownMenu primitive — matches Radix UI DropdownMenu + shadcn exports.
//!
//! Thin wrapper over shared `menu` base. Only Root and Trigger are custom;
//! all other components are re-exported from `menu.rs`.
//!
//! ## Exports (15, matching shadcn)
//!
//! - [`DropdownMenu`] / [`DropdownMenuRoot`]
//! - [`DropdownMenuTrigger`]
//! - [`DropdownMenuContent`] (re-export)
//! - [`DropdownMenuItem`] (re-export)
//! - [`DropdownMenuCheckboxItem`] (re-export)
//! - [`DropdownMenuRadioGroup`] (re-export)
//! - [`DropdownMenuRadioItem`] (re-export)
//! - [`DropdownMenuItemIndicator`] (re-export)
//! - [`DropdownMenuSeparator`] (re-export)
//! - [`DropdownMenuLabel`] (re-export)
//! - [`DropdownMenuGroup`] (re-export)
//! - [`DropdownMenuShortcut`] (re-export)
//! - [`DropdownMenuSub`] (re-export)
//! - [`DropdownMenuSubTrigger`] (re-export)
//! - [`DropdownMenuSubContent`] (re-export)
//! - [`DropdownMenuPortal`] (re-export)

use std::rc::Rc;

use crate::focus_scope::FocusScope;
use crate::menu::MenuCtx;
use crate::popper::{Align, Popper, PopperContent, PopperCtx, Side};
use crate::scroll_lock::use_scroll_lock;
use crate::{
    merge_attributes, use_controlled, use_global_escape_listener, use_id_or, use_outside_click,
    use_presence, use_refocus_on_close, use_unique_id,
};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use dioxus_attributes::attributes;

// ---------------------------------------------------------------------------
// Re-exports from menu base (14 components)
// ---------------------------------------------------------------------------

/// Checkbox menu item — re-export with dropdown-menu data-slot prefix.
pub use crate::menu::MenuCheckboxItem as DropdownMenuCheckboxItem;
// DropdownMenuContent is defined below (not a re-export) — it wraps
// MenuContent in PopperContent for Popper-based positioning.
/// Grouping element — re-export with dropdown-menu data-slot prefix.
pub use crate::menu::MenuGroup as DropdownMenuGroup;
/// Menu item — re-export with dropdown-menu data-slot prefix.
pub use crate::menu::MenuItem as DropdownMenuItem;
/// Indicator for checkbox/radio items — re-export with dropdown-menu data-slot prefix.
pub use crate::menu::MenuItemIndicator as DropdownMenuItemIndicator;
/// Non-interactive label — re-export with dropdown-menu data-slot prefix.
pub use crate::menu::MenuLabel as DropdownMenuLabel;
/// Portal pass-through — re-export.
pub use crate::menu::MenuPortal as DropdownMenuPortal;
/// Radio group for menu items — re-export with dropdown-menu data-slot prefix.
pub use crate::menu::MenuRadioGroup as DropdownMenuRadioGroup;
/// Radio item within a MenuRadioGroup — re-export with dropdown-menu data-slot prefix.
pub use crate::menu::MenuRadioItem as DropdownMenuRadioItem;
/// Visual separator — re-export with dropdown-menu data-slot prefix.
pub use crate::menu::MenuSeparator as DropdownMenuSeparator;
/// Keyboard shortcut hint — re-export with dropdown-menu data-slot prefix.
pub use crate::menu::MenuShortcut as DropdownMenuShortcut;
/// Sub-menu context provider — re-export.
pub use crate::menu::MenuSub as DropdownMenuSub;
/// Sub-menu content — re-export with dropdown-menu data-slot prefix.
pub use crate::menu::MenuSubContent as DropdownMenuSubContent;
/// Sub-menu trigger — re-export with dropdown-menu data-slot prefix.
pub use crate::menu::MenuSubTrigger as DropdownMenuSubTrigger;

// ---------------------------------------------------------------------------
// Internal context (for Trigger ↔ Root communication)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct DropdownMenuInternalCtx {
    set_open: Callback<bool>,
    disabled: bool,
    modal: bool,
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

    /// Whether the menu is modal (traps focus and locks scroll). Defaults to `true`.
    #[props(default = true)]
    pub modal: bool,

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
///                 DropdownMenuItem { "Edit" }
///                 DropdownMenuItem { "Delete" }
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

    let set_open_cb = set_open;
    let typeahead_items = use_signal(Vec::new);
    let grace_intent = use_signal(|| None);
    use_context_provider(|| MenuCtx {
        open,
        on_close: Callback::new(move |()| set_open_cb.call(false)),
        content_id,
        trigger_id,
        slot_prefix: "dropdown-menu",
        typeahead_items,
        grace_intent,
    });

    use_context_provider(|| DropdownMenuInternalCtx {
        set_open,
        disabled: props.disabled,
        modal: props.modal,
    });

    rsx! {
        Popper {
            {props.children}
        }
    }
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
    let ctx: MenuCtx = use_context();
    let internal: DropdownMenuInternalCtx = use_context();
    let popper_ctx: PopperCtx = use_context();
    let mut element = use_signal(|| None::<Rc<MountedData>>);

    let is_open = (ctx.open)();

    let base = attributes!(button {
        id: ctx.trigger_id,
        r#type: "button",
        "data-slot": "dropdown-menu-trigger",
        "data-state": if is_open { "open" } else { "closed" },
        "data-disabled": if internal.disabled { "true" } else { "" },
        disabled: internal.disabled,
        aria_expanded: is_open,
        aria_haspopup: "menu",
        aria_controls: if is_open { Some(ctx.content_id.cloned()) } else { None },
        onmounted: move |e: MountedEvent| {
            let data = e.data();
            element.set(Some(data.clone()));
            popper_ctx.set_anchor_ref(data);
        },
        onpointerdown: move |event: Event<PointerData>| {
            // Upstream: only react to left-click without Ctrl (dropdown-menu.tsx:117-126)
            if internal.disabled
                || event.trigger_button() != Some(MouseButton::Primary)
                || event.modifiers().ctrl()
            {
                return;
            }
            // Prevent trigger from stealing focus when opening
            event.prevent_default();
            let new_open = !is_open;
            internal.set_open.call(new_open);
            if let Some(data) = element() {
                spawn(async move {
                    _ = data.set_focus(true).await;
                });
            }
        },
        onkeydown: move |event: Event<KeyboardData>| {
            if internal.disabled {
                return;
            }
            match event.key() {
                key if key == Key::Enter || key == Key::Character(" ".to_string()) => {
                    event.prevent_default();
                    let new_open = !(ctx.open)();
                    internal.set_open.call(new_open);
                }
                Key::ArrowDown => {
                    internal.set_open.call(true);
                    event.prevent_default();
                }
                _ => {}
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
// DropdownMenuContent (positioned via PopperContent)
// ---------------------------------------------------------------------------

/// Props for [`DropdownMenuContent`].
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuContentProps {
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

    /// Alignment relative to the trigger. Defaults to `Center`.
    #[props(default)]
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

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (menu items).
    pub children: Element,
}

/// The menu content container, positioned via [`PopperContent`].
///
/// Owns presence tracking. Passes `class`, `data-state`, `data-slot` to
/// PopperContent's inner div. Delegates keyboard/focus behavior to
/// [`MenuContent`](crate::menu::MenuContent).
#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element {
    let ctx: MenuCtx = use_context();
    let internal: DropdownMenuInternalCtx = use_context();
    let id = use_id_or(ctx.content_id, props.id);
    let mut presence = use_presence(ctx.open, id);
    let is_modal = internal.modal;

    // Modal: lock scroll when open
    let scroll_lock_active = use_memo(move || is_modal && (ctx.open)());
    use_scroll_lock(scroll_lock_active);

    // Refocus trigger when menu closes (upstream onCloseAutoFocus)
    use_refocus_on_close(ctx.open, ctx.trigger_id);

    // Document-level Escape listener (closes menu even without focus inside)
    {
        let on_close = ctx.on_close;
        let open = ctx.open;
        use_global_escape_listener(move || {
            if *open.peek() {
                on_close.call(());
            }
        });
    }

    // Dismiss on click outside content
    {
        let on_close = ctx.on_close;
        let open = ctx.open;
        use_outside_click(id, move || {
            if *open.peek() {
                on_close.call(());
            }
        });
    }

    if !presence.is_present() && !props.force_mount {
        return rsx! {};
    }

    let content_attrs = attributes!(div {
        id: id,
        "data-slot": "dropdown-menu-content",
        "data-state": presence.data_state(),
        aria_labelledby: (ctx.trigger_id)(),
    });
    let merged = merge_attributes(vec![content_attrs, props.attributes]);

    rsx! {
        PopperContent {
            side: props.side,
            side_offset: props.side_offset,
            align: props.align,
            align_offset: props.align_offset,
            avoid_collisions: props.avoid_collisions,
            collision_padding: props.collision_padding,
            css_var_prefix: "dropdown-menu",
            class: props.class,
            content_attributes: merged,
            on_animation_end: move |_: Event<AnimationData>| presence.on_animation_end(),

            FocusScope {
                trapped: is_modal && (ctx.open)(),
                r#loop: is_modal && (ctx.open)(),
                crate::menu::MenuContent {
                    content_id: id,
                    {props.children}
                }
            }
        }
    }
}
