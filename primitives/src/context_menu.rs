//! ContextMenu primitive — matches Radix UI ContextMenu + shadcn exports.
//!
//! Thin wrapper over shared `menu` base. Root, Trigger, and Content are custom;
//! all other components are re-exported from `menu.rs`.
//!
//! ## Exports (15, matching shadcn)
//!
//! - [`ContextMenu`] / [`ContextMenuRoot`]
//! - [`ContextMenuTrigger`]
//! - [`ContextMenuContent`] (wrapper adding position styling)
//! - [`ContextMenuItem`] (re-export)
//! - [`ContextMenuCheckboxItem`] (re-export)
//! - [`ContextMenuRadioGroup`] (re-export)
//! - [`ContextMenuRadioItem`] (re-export)
//! - [`ContextMenuItemIndicator`] (re-export)
//! - [`ContextMenuSeparator`] (re-export)
//! - [`ContextMenuLabel`] (re-export)
//! - [`ContextMenuGroup`] (re-export)
//! - [`ContextMenuShortcut`] (re-export)
//! - [`ContextMenuSub`] (re-export)
//! - [`ContextMenuSubTrigger`] (re-export)
//! - [`ContextMenuSubContent`] (re-export)
//! - [`ContextMenuPortal`] (re-export)

use crate::menu::MenuCtx;
use crate::popper::{Align, PopperAnchorKind, PopperContent, PopperCtx, Side};
use crate::{
    merge_attributes, use_controlled, use_global_escape_listener, use_id_or, use_outside_click,
    use_presence, use_unique_id,
};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

// ---------------------------------------------------------------------------
// Re-exports from menu base (13 components)
// ---------------------------------------------------------------------------

/// Checkbox menu item — re-export with context-menu data-slot prefix.
pub use crate::menu::MenuCheckboxItem as ContextMenuCheckboxItem;
/// Grouping element — re-export with context-menu data-slot prefix.
pub use crate::menu::MenuGroup as ContextMenuGroup;
/// Menu item — re-export with context-menu data-slot prefix.
pub use crate::menu::MenuItem as ContextMenuItem;
/// Indicator for checkbox/radio items — re-export with context-menu data-slot prefix.
pub use crate::menu::MenuItemIndicator as ContextMenuItemIndicator;
/// Non-interactive label — re-export with context-menu data-slot prefix.
pub use crate::menu::MenuLabel as ContextMenuLabel;
/// Portal pass-through — re-export.
pub use crate::menu::MenuPortal as ContextMenuPortal;
/// Radio group — re-export with context-menu data-slot prefix.
pub use crate::menu::MenuRadioGroup as ContextMenuRadioGroup;
/// Radio item — re-export with context-menu data-slot prefix.
pub use crate::menu::MenuRadioItem as ContextMenuRadioItem;
/// Visual separator — re-export with context-menu data-slot prefix.
pub use crate::menu::MenuSeparator as ContextMenuSeparator;
/// Keyboard shortcut hint — re-export with context-menu data-slot prefix.
pub use crate::menu::MenuShortcut as ContextMenuShortcut;
/// Sub-menu context provider — re-export.
pub use crate::menu::MenuSub as ContextMenuSub;
/// Sub-menu content — re-export with context-menu data-slot prefix.
pub use crate::menu::MenuSubContent as ContextMenuSubContent;
/// Sub-menu trigger — re-export with context-menu data-slot prefix.
pub use crate::menu::MenuSubTrigger as ContextMenuSubTrigger;

// ---------------------------------------------------------------------------
// Internal context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct ContextMenuInternalCtx {
    set_open: Callback<bool>,
    disabled: bool,
    virtual_x: Signal<f64>,
    virtual_y: Signal<f64>,
    trigger_id: Signal<String>,
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
///                 ContextMenuItem { "Edit" }
///                 ContextMenuItem { "Delete" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn ContextMenuRoot(props: ContextMenuRootProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let trigger_id = use_unique_id();
    let content_id = use_unique_id();
    let virtual_x = use_signal(|| 0.0f64);
    let virtual_y = use_signal(|| 0.0f64);

    let set_open_cb = set_open;
    let typeahead_items = use_signal(Vec::new);
    use_context_provider(|| MenuCtx {
        open,
        on_close: Callback::new(move |()| set_open_cb.call(false)),
        content_id,
        trigger_id,
        slot_prefix: "context-menu",
        typeahead_items,
    });

    // Provide PopperCtx with virtual anchor (click coordinates)
    use_context_provider(|| PopperCtx {
        anchor: PopperAnchorKind::Virtual {
            x: virtual_x,
            y: virtual_y,
        },
    });

    use_context_provider(|| ContextMenuInternalCtx {
        set_open,
        disabled: props.disabled,
        virtual_x,
        virtual_y,
        trigger_id,
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
    let ctx: MenuCtx = use_context();
    let mut internal: ContextMenuInternalCtx = use_context();

    let is_open = (ctx.open)();

    rsx! {
        span {
            id: internal.trigger_id,
            "data-slot": "context-menu-trigger",
            "data-state": if is_open { "open" } else { "closed" },
            "data-disabled": if internal.disabled { "true" } else { "" },
            oncontextmenu: move |event: Event<MouseData>| {
                if internal.disabled {
                    return;
                }
                internal.virtual_x.set(event.data().client_coordinates().x);
                internal.virtual_y.set(event.data().client_coordinates().y);
                internal.set_open.call(true);
                event.prevent_default();
            },
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuContent (positioned via PopperContent at click coordinates)
// ---------------------------------------------------------------------------

/// Props for [`ContextMenuContent`].
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuContentProps {
    /// User-provided id override.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Keep content mounted even when closed.
    #[props(default)]
    pub force_mount: bool,

    /// Side of the anchor point to place content. Defaults to `Right` (matching Radix).
    #[props(default = Side::Right)]
    pub side: Side,

    /// Offset from the anchor edge in pixels. Defaults to 2 (matching Radix).
    #[props(default = 2.0)]
    pub side_offset: f64,

    /// Alignment relative to the anchor. Defaults to `Start` (matching Radix).
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

/// The menu content container, positioned at click coordinates via [`PopperContent`].
///
/// Uses a virtual anchor at the right-click point for Popper positioning.
/// Matches Radix's `ContextMenuContent` (side=Right, sideOffset=2, align=Start).
#[component]
pub fn ContextMenuContent(props: ContextMenuContentProps) -> Element {
    let ctx: MenuCtx = use_context();
    let id = use_id_or(ctx.content_id, props.id);
    let mut presence = use_presence(ctx.open, id);

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
        "data-slot": "context-menu-content",
        "data-state": presence.data_state(),
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
            css_var_prefix: "context-menu",
            class: props.class,
            content_attributes: merged,
            on_animation_end: move |_: Event<AnimationData>| presence.on_animation_end(),

            crate::menu::MenuContent {
                content_id: id,
                {props.children}
            }
        }
    }
}
