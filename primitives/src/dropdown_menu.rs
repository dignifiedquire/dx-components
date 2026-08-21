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

use crate::dismissable_layer::DismissableEvent;
use crate::focus_scope::FocusScope;
use crate::menu::{use_menu_dismissal, MenuCtx, MenuDismissalOptions};
use crate::popper::{Align, CollisionPadding, Popper, PopperContent, PopperCtx, Side};
use crate::presence::Presence;
use crate::presence::PresenceContext;
use crate::scroll_lock::use_scroll_lock;
use crate::top_layer::{use_top_layer, TopLayerKind};
use crate::{
    merge_attributes, use_controlled, use_id_or, use_refocus_on_close_unless, use_unique_id,
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
    pub collision_padding: CollisionPadding,

    /// Called when Escape is pressed while this is the topmost layer. Can be
    /// prevented. Upstream: `onEscapeKeyDown`.
    #[props(default)]
    pub on_escape_key_down: Callback<DismissableEvent>,

    /// Called on a pointer-down outside the content. Can be prevented.
    /// Upstream: `onPointerDownOutside`.
    #[props(default)]
    pub on_pointer_down_outside: Callback<DismissableEvent>,

    /// Called when focus moves outside the content. Can be prevented.
    /// Upstream: `onFocusOutside`.
    #[props(default)]
    pub on_focus_outside: Callback<DismissableEvent>,

    /// Called on any outside interaction, pointer or focus. Can be prevented.
    /// Upstream: `onInteractOutside`.
    #[props(default)]
    pub on_interact_outside: Callback<DismissableEvent>,

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
/// Mirrors upstream's split: this gates mounting through [`Presence`] so exit
/// animations can finish, and [`DropdownMenuContentImpl`] holds everything that
/// only exists while the menu is mounted.
#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element {
    let ctx: MenuCtx = use_context();
    let id = use_id_or(ctx.content_id, props.id);

    rsx! {
        Presence {
            present: props.force_mount || (ctx.open)(),
            id: id,
            DropdownMenuContentImpl {
                content_id: id,
                side: props.side,
                side_offset: props.side_offset,
                align: props.align,
                align_offset: props.align_offset,
                avoid_collisions: props.avoid_collisions,
                collision_padding: props.collision_padding,
                on_escape_key_down: props.on_escape_key_down,
                on_pointer_down_outside: props.on_pointer_down_outside,
                on_focus_outside: props.on_focus_outside,
                on_interact_outside: props.on_interact_outside,
                class: props.class,
                attributes: props.attributes,
                children: props.children,
            }
        }
    }
}

/// Props for [`DropdownMenuContentImpl`].
#[derive(Props, Clone, PartialEq)]
struct DropdownMenuContentImplProps {
    content_id: Memo<String>,
    side: Side,
    side_offset: f64,
    align: Align,
    align_offset: f64,
    avoid_collisions: bool,
    collision_padding: CollisionPadding,
    on_escape_key_down: Callback<DismissableEvent>,
    on_pointer_down_outside: Callback<DismissableEvent>,
    on_focus_outside: Callback<DismissableEvent>,
    on_interact_outside: Callback<DismissableEvent>,
    class: Option<String>,
    attributes: Vec<Attribute>,
    children: Element,
}

/// Everything that only exists while the menu is mounted.
///
/// Sits inside [`Presence`], so it can read [`PresenceContext`] and keep the
/// element in the top layer until the exit animation has finished rather than
/// the instant `open` flips.
#[component]
fn DropdownMenuContentImpl(props: DropdownMenuContentImplProps) -> Element {
    let ctx: MenuCtx = use_context();
    let internal: DropdownMenuInternalCtx = use_context();
    let id = props.content_id;
    let is_modal = internal.modal;

    // Modal: lock scroll while open (upstream: `RemoveScroll`).
    let modal_active = use_memo(move || is_modal && (ctx.open)());
    use_scroll_lock(modal_active);

    // Refocus trigger when the menu closes (upstream `onCloseAutoFocus`),
    // unless the close was caused by interacting somewhere else.
    let dismissal = use_menu_dismissal(
        id,
        MenuDismissalOptions {
            is_modal,
            on_escape_key_down: props.on_escape_key_down,
            on_pointer_down_outside: props.on_pointer_down_outside,
            on_focus_outside: props.on_focus_outside,
            on_interact_outside: props.on_interact_outside,
        },
    );
    use_refocus_on_close_unless(
        ctx.open,
        ctx.trigger_id,
        dismissal.has_interacted_outside.into(),
    );

    let data_state = if (ctx.open)() { "open" } else { "closed" };

    let content_attrs = attributes!(div {
        id: id,
        role: "menu",
        aria_orientation: "vertical",
        "data-slot": "dropdown-menu-content",
        "data-state": data_state,
        aria_labelledby: (ctx.trigger_id)(),
    });
    let merged = merge_attributes(vec![content_attrs, dismissal.attributes, props.attributes]);

    // `popover="manual"` puts the floated wrapper in the browser top layer,
    // escaping ancestor overflow/transform/stacking contexts. `manual` rather
    // than `auto` because dismissal is the layer's job and nested submenus must
    // coexist — `auto` allows only one open popover at a time.
    //
    // Driven by Presence's animation-aware `present`, not `open`: hiding on
    // `open` sets `display: none` before the exit animation can run.
    let presence: PresenceContext = use_context();
    let present = presence.present;
    let mut wrapper_mounted = use_signal(|| None::<Rc<MountedData>>);
    let set_open = Callback::new(move |open: bool| {
        if !open {
            ctx.on_close.call(());
        }
    });
    use_top_layer(
        wrapper_mounted.into(),
        present.into(),
        set_open,
        TopLayerKind::PopoverManual,
    );

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
            content_style: dismissal.content_style,
            wrapper_attributes: attributes!(div { popover: "manual" }),
            on_wrapper_mounted: move |evt: Event<MountedData>| {
                wrapper_mounted.set(Some(evt.data()));
            },

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
