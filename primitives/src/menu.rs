//! Internal shared menu base — matches `@radix-ui/react-menu`.
//!
//! Not exposed publicly. DropdownMenu, ContextMenu, and Menubar re-export
//! these components with prefixed names matching shadcn's data-slot convention.

use std::rc::Rc;

use crate::direction::Orientation;
use crate::merge_attributes;
use crate::popper::{Popper, PopperContent, PopperCtx, Side};
use crate::portal::Portal;
use crate::roving_focus::{RovingFocusGroup, RovingFocusGroupItem, RovingFocusSlotProps};
use crate::typeahead::{use_typeahead, TypeaheadItem};
use crate::{use_controlled, use_id_or, use_presence, use_unique_id};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

// ---------------------------------------------------------------------------
// Contexts
// ---------------------------------------------------------------------------

/// Typeahead entry — stores text and element ref for focus-on-match.
#[derive(Clone)]
pub(crate) struct MenuTypeaheadEntry {
    pub text: String,
    pub element_ref: Signal<Option<Rc<MountedData>>>,
}

/// A point in 2D space.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Point {
    x: f64,
    y: f64,
}

/// Grace area intent — polygon from pointer to sub-content edges.
/// When set, pointer moves inside this polygon don't close the sub-menu.
#[derive(Clone, Debug)]
pub(crate) struct GraceIntent {
    pub area: Vec<Point>,
}

/// Provided by each wrapper (DropdownMenu, ContextMenu, Menubar).
#[derive(Clone, Copy)]
pub(crate) struct MenuCtx {
    pub open: Memo<bool>,
    pub on_close: Callback<()>,
    pub content_id: Signal<String>,
    pub trigger_id: Signal<String>,
    pub slot_prefix: &'static str,
    /// Registry for typeahead search — items register their text_value + element ref.
    pub typeahead_items: Signal<Vec<MenuTypeaheadEntry>>,
    /// Grace area for sub-menu pointer navigation.
    pub grace_intent: Signal<Option<GraceIntent>>,
}

/// Provided by MenuCheckboxItem / MenuRadioItem for MenuItemIndicator.
#[derive(Clone, Copy)]
struct MenuItemCheckedCtx {
    checked: Memo<bool>,
}

/// Provided by MenuRadioGroup for MenuRadioItem.
#[derive(Clone, Copy)]
struct MenuRadioCtx {
    value: Memo<Option<String>>,
    on_value_change: Callback<String>,
}

/// Provided by MenuSub for MenuSubTrigger / MenuSubContent.
#[derive(Clone, Copy)]
struct MenuSubCtx {
    open: Memo<bool>,
    set_open: Callback<bool>,
    content_id: Signal<String>,
    trigger_id: Signal<String>,
}

// ---------------------------------------------------------------------------
// MenuPortal (no-op pass-through)
// ---------------------------------------------------------------------------

/// Props for [`MenuPortal`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuPortalProps {
    /// Children.
    pub children: Element,
}

/// Teleports menu content to the nearest [`PortalHost`](crate::portal::PortalHost).
///
/// Matches Radix's `MenuPortal` which uses `ReactDOM.createPortal` to render
/// at `document.body`. We use our context-based Portal component instead.
#[component]
pub fn MenuPortal(props: MenuPortalProps) -> Element {
    rsx! {
        Portal {
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenuContent
// ---------------------------------------------------------------------------

/// Props for [`MenuContent`].
///
/// This is a behavioral wrapper only — presence tracking, `class`, `data-state`,
/// and `id` live on PopperContent's inner div (owned by the consumer component).
#[derive(Props, Clone, PartialEq)]
pub struct MenuContentProps {
    /// Resolved content element ID. Not rendered as an attribute here — it is on
    /// PopperContent's inner div. Used only for the focusout contains-check.
    pub content_id: Memo<String>,

    /// Children (menu items).
    pub children: Element,

    // -- Internal callbacks (not for public use) --
    /// Override default Escape behavior (used by Menubar).
    #[props(default)]
    pub on_escape_override: Option<Callback<()>>,

    /// Called on ArrowLeft in content (used by Menubar to switch menus).
    #[props(default)]
    pub on_arrow_left: Option<Callback<()>>,

    /// Called on ArrowRight in content (used by Menubar to switch menus).
    #[props(default)]
    pub on_arrow_right: Option<Callback<()>>,

    /// CSS selector for elements that should NOT trigger focusout dismiss.
    /// Used by Menubar to prevent close when focus moves to another trigger.
    #[props(default)]
    pub focus_exclude_selector: Option<&'static str>,
}

/// Menu behavior container. Has `role="menu"` with keyboard navigation.
///
/// Wraps children in a vertical `RovingFocusGroup`. Does **not** manage presence
/// or visual styling — those are owned by the consumer (DropdownMenuContent,
/// ContextMenuContent, MenubarContent) which passes `class`/`data-state` to
/// PopperContent.
#[component]
pub fn MenuContent(props: MenuContentProps) -> Element {
    let ctx: MenuCtx = use_context();

    // Typeahead: prefix search with 1s auto-clear (matching Radix menu behavior)
    let mut typeahead = use_typeahead(1000);

    let on_close = ctx.on_close;
    let on_escape_override = props.on_escape_override;
    let on_arrow_left = props.on_arrow_left;
    let on_arrow_right = props.on_arrow_right;
    let focus_exclude_selector = props.focus_exclude_selector;
    let children = props.children;
    let trigger_id = ctx.trigger_id;
    let content_id = props.content_id;

    rsx! {
        RovingFocusGroup {
            orientation: Signal::new(Some(Orientation::Vertical)),
            r#loop: Signal::new(true),
            r#as: {
                let children = children.clone();
                move |roving_attrs: Vec<Attribute>| {
                    let menu_attrs = attributes!(div {
                        role: "menu",
                        aria_orientation: "vertical",
                        aria_labelledby: (trigger_id)(),
                    });
                    let merged = merge_attributes(vec![roving_attrs, menu_attrs]);

                    rsx! {
                        div {
                            onkeydown: move |event: Event<KeyboardData>| {
                                match event.key() {
                                    Key::Escape => {
                                        if let Some(on_esc) = on_escape_override {
                                            on_esc.call(());
                                        } else {
                                            on_close.call(());
                                        }
                                        event.prevent_default();
                                    }
                                    Key::Tab => {
                                        // Prevent Tab from moving focus out of open menus (matching Radix)
                                        event.prevent_default();
                                    }
                                    Key::ArrowLeft => {
                                        if let Some(cb) = on_arrow_left {
                                            cb.call(());
                                            event.prevent_default();
                                        }
                                    }
                                    Key::ArrowRight => {
                                        if let Some(cb) = on_arrow_right {
                                            cb.call(());
                                            event.prevent_default();
                                        }
                                    }
                                    Key::Character(ch) => {
                                        // Typeahead: type to jump to items (matching Radix)
                                        if let Some(first_char) = ch.chars().next() {
                                            let items: Vec<TypeaheadItem> = ctx
                                                .typeahead_items
                                                .read()
                                                .iter()
                                                .enumerate()
                                                .map(|(i, entry)| TypeaheadItem {
                                                    text: entry.text.clone(),
                                                    index: i,
                                                })
                                                .collect();
                                            if let Some(matched_idx) = typeahead.search(first_char, &items) {
                                                let entries = ctx.typeahead_items.read();
                                                if let Some(entry) = entries.get(matched_idx) {
                                                    if let Some(ref el) = *entry.element_ref.read() {
                                                        let el = el.clone();
                                                        spawn(async move { let _ = el.set_focus(true).await; });
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            },
                            onfocusout: move |_| {
                                let id_str = content_id();
                                let open = ctx.open;
                                let on_close = ctx.on_close;
                                let exclude = focus_exclude_selector;
                                spawn(async move {
                                    // Wait a frame for focus to settle, then check if focus
                                    // is still inside the content or moved to an excluded element
                                    let exclude_check = match exclude {
                                        Some(sel) => format!(
                                            "||document.activeElement&&document.activeElement.closest('{sel}')"
                                        ),
                                        None => String::new(),
                                    };
                                    let js = format!(
                                        "var e=document.getElementById('{id_str}');\
                                         dioxus.send(e&&e.contains(document.activeElement){exclude_check})"
                                    );
                                    let mut eval = document::eval(&js);
                                    if let Ok(skip_close) = eval.recv::<bool>().await {
                                        if !skip_close && *open.peek() {
                                            on_close.call(());
                                        }
                                    }
                                });
                            },
                            onpointerdown: move |event| {
                                event.prevent_default();
                                event.stop_propagation();
                            },
                            // Grace area: if pointer is inside the grace polygon, don't close sub-menu
                            onpointermove: {
                                let mut grace = ctx.grace_intent;
                                move |event: Event<PointerData>| {
                                    let should_clear = {
                                        let read = grace.read();
                                        if let Some(ref intent) = *read {
                                            let px = event.data().client_coordinates().x;
                                            let py = event.data().client_coordinates().y;
                                            !is_point_in_polygon(px, py, &intent.area)
                                        } else {
                                            false
                                        }
                                    };
                                    if should_clear {
                                        grace.set(None);
                                    }
                                }
                            },
                            // Upstream: outline: 'none' on menu content (menu.tsx:508)
                            outline: "none",
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
// MenuItem
// ---------------------------------------------------------------------------

/// Props for [`MenuItem`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuItemProps {
    /// Text value for typeahead search. If not provided, typeahead won't match this item.
    /// Matches Radix's `textValue` prop on MenuItem.
    #[props(default)]
    pub text_value: Option<String>,

    /// Whether the item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Called when the item is selected (click or Enter/Space).
    #[props(default)]
    pub on_select: EventHandler<()>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A menu item. Has `role="menuitem"`.
///
/// Wraps in `RovingFocusGroupItem` for keyboard navigation.
#[component]
pub fn MenuItem(props: MenuItemProps) -> Element {
    let mut ctx: MenuCtx = use_context();
    let slot = format!("{}-item", ctx.slot_prefix);
    let disabled = props.disabled;
    let on_close = ctx.on_close;
    let class = props.class;
    let user_attrs = props.attributes;
    let children = props.children;

    // Register for typeahead search (matching Radix textValue prop)
    let mut element_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    if let Some(text) = props.text_value.clone() {
        let entry_index = use_hook(|| {
            let mut items = ctx.typeahead_items.write();
            let idx = items.len();
            items.push(MenuTypeaheadEntry { text, element_ref });
            idx
        });
        crate::use_effect_cleanup(move || {
            let mut items = ctx.typeahead_items.write();
            if entry_index < items.len() {
                items.remove(entry_index);
            }
        });
    }

    rsx! {
        RovingFocusGroupItem {
            focusable: !disabled,
            r#as: {
                let slot = slot.clone();
                let class = class.clone();
                let user_attrs = user_attrs.clone();
                let children = children.clone();
                move |slot_props: RovingFocusSlotProps| {
                    let item_attrs = attributes!(div {
                        role: "menuitem",
                        "data-slot": slot.clone(),
                        "data-disabled": if disabled { Some("true") } else { None },
                        aria_disabled: if disabled { Some("true") } else { None },
                        class: class.clone(),
                    });
                    let merged = merge_attributes(vec![slot_props.attributes, item_attrs, user_attrs.clone()]);

                    rsx! {
                        div {
                            onmounted: move |e: Event<MountedData>| {
                                element_ref.set(Some(e.data()));
                                slot_props.on_mounted.call(e);
                            },
                            onfocus: move |e| slot_props.on_focus.call(e),
                            onmousedown: move |e| slot_props.on_mousedown.call(e),
                            onkeydown: {
                                move |event: Event<KeyboardData>| {
                                    let key = event.key();
                                    if key == Key::Enter || key == Key::Character(" ".to_string()) {
                                        if !disabled {
                                            props.on_select.call(());
                                            on_close.call(());
                                        }
                                        event.prevent_default();
                                        event.stop_propagation();
                                    } else {
                                        slot_props.on_keydown.call(event);
                                    }
                                }
                            },
                            onclick: move |e: Event<MouseData>| {
                                e.stop_propagation();
                                if !disabled {
                                    props.on_select.call(());
                                    on_close.call(());
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
// MenuCheckboxItem
// ---------------------------------------------------------------------------

/// Props for [`MenuCheckboxItem`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuCheckboxItemProps {
    /// Whether the checkbox is checked.
    #[props(default)]
    pub checked: ReadSignal<bool>,

    /// Called when the checked state changes.
    #[props(default)]
    pub on_checked_change: Callback<bool>,

    /// Whether the item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Called when the item is selected.
    #[props(default)]
    pub on_select: EventHandler<()>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A menu item with a checkbox. Has `role="menuitemcheckbox"`.
#[component]
pub fn MenuCheckboxItem(props: MenuCheckboxItemProps) -> Element {
    let ctx: MenuCtx = use_context();
    let slot = format!("{}-checkbox-item", ctx.slot_prefix);
    let disabled = props.disabled;
    let on_close = ctx.on_close;
    let checked = props.checked;
    let on_checked_change = props.on_checked_change;
    let class = props.class;
    let user_attrs = props.attributes;
    let children = props.children;

    #[allow(clippy::redundant_closure)]
    let is_checked = use_memo(move || checked());
    use_context_provider(|| MenuItemCheckedCtx {
        checked: is_checked,
    });

    rsx! {
        RovingFocusGroupItem {
            focusable: !disabled,
            r#as: {
                let slot = slot.clone();
                let class = class.clone();
                let user_attrs = user_attrs.clone();
                let children = children.clone();
                move |slot_props: RovingFocusSlotProps| {
                    let item_attrs = attributes!(div {
                        role: "menuitemcheckbox",
                        "data-slot": slot.clone(),
                        "data-disabled": if disabled { Some("true") } else { None },
                        aria_checked: is_checked(),
                        aria_disabled: if disabled { Some("true") } else { None },
                        class: class.clone(),
                    });
                    let merged = merge_attributes(vec![slot_props.attributes, item_attrs, user_attrs.clone()]);

                    rsx! {
                        div {
                            onmounted: move |e| slot_props.on_mounted.call(e),
                            onfocus: move |e| slot_props.on_focus.call(e),
                            onmousedown: move |e| slot_props.on_mousedown.call(e),
                            onkeydown: {
                                move |event: Event<KeyboardData>| {
                                    let key = event.key();
                                    if key == Key::Enter || key == Key::Character(" ".to_string()) {
                                        if !disabled {
                                            on_checked_change.call(!checked());
                                            props.on_select.call(());
                                            on_close.call(());
                                        }
                                        event.prevent_default();
                                        event.stop_propagation();
                                    } else {
                                        slot_props.on_keydown.call(event);
                                    }
                                }
                            },
                            onclick: move |e: Event<MouseData>| {
                                e.stop_propagation();
                                if !disabled {
                                    on_checked_change.call(!checked());
                                    props.on_select.call(());
                                    on_close.call(());
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
// MenuRadioGroup
// ---------------------------------------------------------------------------

/// Props for [`MenuRadioGroup`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuRadioGroupProps {
    /// The current selected value.
    #[props(default)]
    pub value: ReadSignal<Option<String>>,

    /// Called when the value changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A radio group for menu items. Has `role="group"`.
#[component]
pub fn MenuRadioGroup(props: MenuRadioGroupProps) -> Element {
    let ctx: MenuCtx = use_context();
    let slot = format!("{}-radio-group", ctx.slot_prefix);
    let val = props.value;
    #[allow(clippy::redundant_closure)]
    let value = use_memo(move || val());

    use_context_provider(|| MenuRadioCtx {
        value,
        on_value_change: props.on_value_change,
    });

    rsx! {
        div {
            role: "group",
            "data-slot": slot,
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenuRadioItem
// ---------------------------------------------------------------------------

/// Props for [`MenuRadioItem`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuRadioItemProps {
    /// The value for this radio item.
    pub value: String,

    /// Whether the item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Called when the item is selected.
    #[props(default)]
    pub on_select: EventHandler<()>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A radio item within a MenuRadioGroup. Has `role="menuitemradio"`.
#[component]
pub fn MenuRadioItem(props: MenuRadioItemProps) -> Element {
    let ctx: MenuCtx = use_context();
    let radio_ctx: MenuRadioCtx = use_context();
    let slot = format!("{}-radio-item", ctx.slot_prefix);
    let disabled = props.disabled;
    let on_close = ctx.on_close;
    let item_value = props.value.clone();
    let class = props.class;
    let user_attrs = props.attributes;
    let children = props.children;

    let is_checked = {
        let item_value = item_value.clone();
        use_memo(move || (radio_ctx.value)().as_deref() == Some(item_value.as_str()))
    };
    use_context_provider(|| MenuItemCheckedCtx {
        checked: is_checked,
    });

    rsx! {
        RovingFocusGroupItem {
            focusable: !disabled,
            r#as: {
                let slot = slot.clone();
                let class = class.clone();
                let user_attrs = user_attrs.clone();
                let children = children.clone();
                let item_value = item_value.clone();
                move |slot_props: RovingFocusSlotProps| {
                    let item_attrs = attributes!(div {
                        role: "menuitemradio",
                        "data-slot": slot.clone(),
                        "data-disabled": if disabled { Some("true") } else { None },
                        aria_checked: is_checked(),
                        aria_disabled: if disabled { Some("true") } else { None },
                        class: class.clone(),
                    });
                    let merged = merge_attributes(vec![slot_props.attributes, item_attrs, user_attrs.clone()]);

                    let select_value = item_value.clone();
                    let select_value2 = item_value.clone();

                    rsx! {
                        div {
                            onmounted: move |e| slot_props.on_mounted.call(e),
                            onfocus: move |e| slot_props.on_focus.call(e),
                            onmousedown: move |e| slot_props.on_mousedown.call(e),
                            onkeydown: {
                                let select_value = select_value.clone();
                                move |event: Event<KeyboardData>| {
                                    let key = event.key();
                                    if key == Key::Enter || key == Key::Character(" ".to_string()) {
                                        if !disabled {
                                            radio_ctx.on_value_change.call(select_value.clone());
                                            props.on_select.call(());
                                            on_close.call(());
                                        }
                                        event.prevent_default();
                                        event.stop_propagation();
                                    } else {
                                        slot_props.on_keydown.call(event);
                                    }
                                }
                            },
                            onclick: {
                                let select_value2 = select_value2.clone();
                                move |e: Event<MouseData>| {
                                    e.stop_propagation();
                                    if !disabled {
                                        radio_ctx.on_value_change.call(select_value2.clone());
                                        props.on_select.call(());
                                        on_close.call(());
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
// MenuItemIndicator
// ---------------------------------------------------------------------------

/// Props for [`MenuItemIndicator`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuItemIndicatorProps {
    /// Keep indicator mounted even when unchecked.
    #[props(default)]
    pub force_mount: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (the check/radio icon).
    pub children: Element,
}

/// Renders children only when the parent CheckboxItem or RadioItem is checked.
#[component]
pub fn MenuItemIndicator(props: MenuItemIndicatorProps) -> Element {
    let checked_ctx: MenuItemCheckedCtx = use_context();
    let ctx: MenuCtx = use_context();
    let slot = format!("{}-item-indicator", ctx.slot_prefix);

    if !(checked_ctx.checked)() && !props.force_mount {
        return rsx! {};
    }

    rsx! {
        span {
            "data-slot": slot,
            "data-state": if (checked_ctx.checked)() { "checked" } else { "unchecked" },
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenuSub
// ---------------------------------------------------------------------------

/// Props for [`MenuSub`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuSubProps {
    /// Controlled open state.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// Default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Children.
    pub children: Element,
}

/// No-DOM context provider for a sub-menu.
#[component]
pub fn MenuSub(props: MenuSubProps) -> Element {
    let ctx: MenuCtx = use_context();
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let content_id = use_unique_id();
    let trigger_id = use_unique_id();

    // Upstream menu.tsx:982-985: close sub when parent closes
    {
        let parent_open = ctx.open;
        use_effect(move || {
            if !parent_open() {
                set_open.call(false);
            }
        });
    }

    use_context_provider(|| MenuSubCtx {
        open,
        set_open,
        content_id,
        trigger_id,
    });

    rsx! {
        Popper {
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenuSubTrigger
// ---------------------------------------------------------------------------

/// Props for [`MenuSubTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuSubTriggerProps {
    /// Whether the sub-trigger is disabled.
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

/// A menu item that opens a sub-menu. Has `role="menuitem"`, `aria-haspopup="menu"`.
///
/// Opens on hover (300ms delay) and ArrowRight. Closes on ArrowLeft.
///
/// ## Radix deviation
/// Radix uses a triangular "grace area" polygon between the trigger and
/// sub-content to allow diagonal mouse movement. This implementation uses
/// a simpler generation-counter delay approach instead, which provides
/// adequate UX for most cases. The `document::eval` timer approach from
/// the previous implementation has been replaced with `dioxus_sdk_time::sleep`
/// + generation counter for cancellation (no JS calls).
#[component]
pub fn MenuSubTrigger(props: MenuSubTriggerProps) -> Element {
    let ctx: MenuCtx = use_context();
    let sub_ctx: MenuSubCtx = use_context();
    let popper_ctx: PopperCtx = use_context();
    let slot = format!("{}-sub-trigger", ctx.slot_prefix);
    let disabled = props.disabled;
    let is_open = sub_ctx.open;
    let class = props.class;
    let user_attrs = props.attributes;
    let children = props.children;

    // Generation counter for cancelling stale hover timers (replaces document::eval timers)
    let mut open_gen = use_signal(|| 0u64);

    rsx! {
        RovingFocusGroupItem {
            focusable: !disabled,
            r#as: {
                let slot = slot.clone();
                let class = class.clone();
                let user_attrs = user_attrs.clone();
                let children = children.clone();
                move |slot_props: RovingFocusSlotProps| {
                    let item_attrs = attributes!(div {
                        role: "menuitem",
                        id: sub_ctx.trigger_id,
                        "data-slot": slot.clone(),
                        "data-state": if is_open() { "open" } else { "closed" },
                        "data-disabled": if disabled { Some("true") } else { None },
                        aria_haspopup: "menu",
                        aria_expanded: is_open(),
                        aria_controls: if is_open() { Some(sub_ctx.content_id.cloned()) } else { None },
                        aria_disabled: if disabled { Some("true") } else { None },
                        class: class.clone(),
                    });
                    let merged = merge_attributes(vec![slot_props.attributes, item_attrs, user_attrs.clone()]);

                    rsx! {
                        div {
                            onmounted: move |e: MountedEvent| {
                                slot_props.on_mounted.call(e.clone());
                                popper_ctx.set_anchor_ref(e.data());
                            },
                            onfocus: move |e| slot_props.on_focus.call(e),
                            onmousedown: move |e| slot_props.on_mousedown.call(e),
                            onkeydown: move |event: Event<KeyboardData>| {
                                match event.key() {
                                    Key::ArrowRight => {
                                        if !disabled {
                                            sub_ctx.set_open.call(true);
                                            event.prevent_default();
                                            event.stop_propagation();
                                        }
                                    }
                                    _ => {
                                        slot_props.on_keydown.call(event);
                                    }
                                }
                            },
                            onpointerenter: move |_| {
                                if !disabled {
                                    // Cancel any previous open timer
                                    let gen = *open_gen.peek() + 1;
                                    open_gen.set(gen);
                                    // Open after 100ms delay (matching Radix SELECTION_OPEN_DELAY)
                                    spawn(async move {
                                        dioxus_sdk_time::sleep(std::time::Duration::from_millis(100)).await;
                                        if *open_gen.peek() == gen {
                                            sub_ctx.set_open.call(true);
                                        }
                                    });
                                }
                            },
                            onpointerleave: move |event: Event<PointerData>| {
                                // Cancel pending open timer
                                let current = *open_gen.peek();
                                open_gen.set(current + 1);

                                // Compute grace area triangle from pointer to sub-content edges
                                // (upstream menu.tsx:1082-1122)
                                if is_open() {
                                    let px = event.data().client_coordinates().x;
                                    let py = event.data().client_coordinates().y;
                                    let content_id_str = (sub_ctx.content_id)();
                                    let mut grace = ctx.grace_intent;
                                    spawn(async move {
                                        let js = format!(
                                            "var e=document.getElementById('{content_id_str}');\
                                             if(e){{var r=e.getBoundingClientRect();\
                                             dioxus.send([r.left,r.top,r.right,r.bottom])}}else{{dioxus.send(null)}}"
                                        );
                                        let mut eval = document::eval(&js);
                                        if let Ok(Some([left, top, right, bottom])) = eval.recv::<Option<[f64; 4]>>().await {
                                            // Polygon: pointer position → sub-content bounding box corners
                                            let area = vec![
                                                Point { x: px, y: py },
                                                Point { x: left, y: top },
                                                Point { x: right, y: top },
                                                Point { x: right, y: bottom },
                                                Point { x: left, y: bottom },
                                            ];
                                            grace.set(Some(GraceIntent { area }));
                                        }
                                    });
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
// MenuSubContent
// ---------------------------------------------------------------------------

/// Props for [`MenuSubContent`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuSubContentProps {
    /// User-provided id override.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Keep content mounted even when closed.
    #[props(default)]
    pub force_mount: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Sub-menu content. Has `role="menu"`. Like `MenuContent` but for sub-menus.
///
/// ArrowLeft closes the sub-menu. Escape closes the sub-menu.
/// Positioned via [`PopperContent`] with `side: Right` (matching Radix).
#[component]
pub fn MenuSubContent(props: MenuSubContentProps) -> Element {
    let ctx: MenuCtx = use_context();
    let sub_ctx: MenuSubCtx = use_context();
    let id = use_id_or(sub_ctx.content_id, props.id);
    let mut presence = use_presence(sub_ctx.open, id);

    if !presence.is_present() && !props.force_mount {
        return rsx! {};
    }

    let slot = format!("{}-sub-content", ctx.slot_prefix);
    let data_state = presence.data_state();
    let children = props.children;

    let content_attrs = attributes!(div {
        id: id,
        "data-slot": slot,
        "data-state": data_state,
    });
    let merged = merge_attributes(vec![content_attrs, props.attributes]);

    rsx! {
        Portal {
            PopperContent {
                side: Side::Right,
                css_var_prefix: "menu",
                class: props.class,
                content_attributes: merged,
                on_animation_end: move |_: Event<AnimationData>| presence.on_animation_end(),

                RovingFocusGroup {
                    orientation: Signal::new(Some(Orientation::Vertical)),
                    r#loop: Signal::new(true),
                    r#as: {
                        let children = children.clone();
                        move |roving_attrs: Vec<Attribute>| {
                            let menu_attrs = attributes!(div {
                                role: "menu",
                                aria_orientation: "vertical",
                                aria_labelledby: sub_ctx.trigger_id.cloned(),
                            });
                            let merged = merge_attributes(vec![roving_attrs, menu_attrs]);

                            rsx! {
                                div {
                                    onkeydown: move |event: Event<KeyboardData>| {
                                        match event.key() {
                                            Key::ArrowLeft | Key::Escape => {
                                                sub_ctx.set_open.call(false);
                                                event.prevent_default();
                                                event.stop_propagation();
                                            }
                                            _ => {}
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
    }
}

// ---------------------------------------------------------------------------
// MenuSeparator
// ---------------------------------------------------------------------------

/// Props for [`MenuSeparator`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuSeparatorProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A visual separator between menu items. Has `role="separator"`.
#[component]
pub fn MenuSeparator(props: MenuSeparatorProps) -> Element {
    let ctx: MenuCtx = use_context();
    let slot = format!("{}-separator", ctx.slot_prefix);

    rsx! {
        div {
            role: "separator",
            "data-slot": slot,
            aria_orientation: "horizontal",
            class: props.class,
            ..props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// MenuLabel
// ---------------------------------------------------------------------------

/// Props for [`MenuLabel`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuLabelProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A non-interactive label within the menu.
#[component]
pub fn MenuLabel(props: MenuLabelProps) -> Element {
    let ctx: MenuCtx = use_context();
    let slot = format!("{}-label", ctx.slot_prefix);

    rsx! {
        div {
            "data-slot": slot,
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenuGroup
// ---------------------------------------------------------------------------

/// Props for [`MenuGroup`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuGroupProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A grouping element for menu items. Has `role="group"`.
#[component]
pub fn MenuGroup(props: MenuGroupProps) -> Element {
    let ctx: MenuCtx = use_context();
    let slot = format!("{}-group", ctx.slot_prefix);

    rsx! {
        div {
            role: "group",
            "data-slot": slot,
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenuShortcut
// ---------------------------------------------------------------------------

/// Props for [`MenuShortcut`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuShortcutProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (the shortcut text).
    pub children: Element,
}

/// A keyboard shortcut hint displayed alongside a menu item.
#[component]
pub fn MenuShortcut(props: MenuShortcutProps) -> Element {
    let ctx: MenuCtx = use_context();
    let slot = format!("{}-shortcut", ctx.slot_prefix);

    rsx! {
        span {
            "data-slot": slot,
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// Grace area utilities (upstream menu.tsx pointer grace area)
// ---------------------------------------------------------------------------

/// Ray-casting point-in-polygon test.
/// Returns true if point (px, py) is inside the polygon defined by `vertices`.
fn is_point_in_polygon(px: f64, py: f64, vertices: &[Point]) -> bool {
    let n = vertices.len();
    if n < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let vi = &vertices[i];
        let vj = &vertices[j];
        if (vi.y > py) != (vj.y > py) && px < (vj.x - vi.x) * (py - vi.y) / (vj.y - vi.y) + vi.x {
            inside = !inside;
        }
        j = i;
    }
    inside
}
