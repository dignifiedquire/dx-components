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
use crate::{merge_attributes, use_controlled, use_unique_id};
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
    position: Signal<(i32, i32)>,
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
    let position = use_signal(|| (0i32, 0i32));

    let set_open_cb = set_open;
    use_context_provider(|| MenuCtx {
        open,
        on_close: Callback::new(move |()| set_open_cb.call(false)),
        content_id,
        trigger_id,
        slot_prefix: "context-menu",
    });

    use_context_provider(|| ContextMenuInternalCtx {
        set_open,
        disabled: props.disabled,
        position,
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
                internal.position.set((
                    event.data().client_coordinates().x as i32,
                    event.data().client_coordinates().y as i32,
                ));
                internal.set_open.call(true);
                event.prevent_default();
            },
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuContent (wrapper adding position styling)
// ---------------------------------------------------------------------------

/// Props for [`ContextMenuContent`].
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuContentProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes for the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (should include menu items).
    pub children: Element,
}

/// The menu content container, positioned at click coordinates.
///
/// Wraps `MenuContent` with `position: fixed` at the right-click location.
#[component]
pub fn ContextMenuContent(props: ContextMenuContentProps) -> Element {
    let internal: ContextMenuInternalCtx = use_context();
    let (x, y) = (internal.position)();

    let pos_attrs = attributes!(div {
        position: "fixed",
        left: format!("{x}px"),
        top: format!("{y}px"),
    });
    let combined = merge_attributes(vec![pos_attrs, props.attributes]);

    rsx! {
        crate::menu::MenuContent {
            class: props.class,
            extra_attributes: combined,
            {props.children}
        }
    }
}
