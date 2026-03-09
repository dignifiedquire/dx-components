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

use crate::menu::MenuCtx;
use crate::{merge_attributes, use_controlled, use_unique_id};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

// ---------------------------------------------------------------------------
// Re-exports from menu base (14 components)
// ---------------------------------------------------------------------------

/// Checkbox menu item — re-export with dropdown-menu data-slot prefix.
pub use crate::menu::MenuCheckboxItem as DropdownMenuCheckboxItem;
/// Menu content container — re-export with dropdown-menu data-slot prefix.
pub use crate::menu::MenuContent as DropdownMenuContent;
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
    use_context_provider(|| MenuCtx {
        open,
        on_close: Callback::new(move |()| set_open_cb.call(false)),
        content_id,
        trigger_id,
        slot_prefix: "dropdown-menu",
    });

    use_context_provider(|| DropdownMenuInternalCtx {
        set_open,
        disabled: props.disabled,
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
    let ctx: MenuCtx = use_context();
    let internal: DropdownMenuInternalCtx = use_context();
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
            element.set(Some(e.data()));
        },
        onclick: move |_| {
            if internal.disabled {
                return;
            }
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
