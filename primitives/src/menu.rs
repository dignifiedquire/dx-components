//! Internal shared menu base — matches `@radix-ui/react-menu`.
//!
//! Not exposed publicly. DropdownMenu, ContextMenu, and Menubar re-export
//! these components with prefixed names matching shadcn's data-slot convention.

use crate::direction::Orientation;
use crate::merge_attributes;
use crate::roving_focus::{RovingFocusGroup, RovingFocusGroupItem, RovingFocusSlotProps};
use crate::{use_controlled, use_global_escape_listener, use_id_or, use_presence, use_unique_id};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

// ---------------------------------------------------------------------------
// Contexts
// ---------------------------------------------------------------------------

/// Provided by each wrapper (DropdownMenu, ContextMenu, Menubar).
#[derive(Clone, Copy)]
pub(crate) struct MenuCtx {
    pub open: Memo<bool>,
    pub on_close: Callback<()>,
    pub content_id: Signal<String>,
    pub trigger_id: Signal<String>,
    pub slot_prefix: &'static str,
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

/// No-op pass-through for API compatibility with Radix Portal.
///
/// Dioxus does not have React's `createPortal`. This renders children in place.
#[component]
pub fn MenuPortal(props: MenuPortalProps) -> Element {
    rsx! { {props.children} }
}

// ---------------------------------------------------------------------------
// MenuContent
// ---------------------------------------------------------------------------

/// Props for [`MenuContent`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuContentProps {
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

    /// Extra attributes merged into the content div (used internally by wrappers
    /// like ContextMenuContent to inject position styling).
    #[props(default)]
    pub extra_attributes: Vec<Attribute>,
}

/// The menu content container. Has `role="menu"`.
///
/// Wraps children in a vertical `RovingFocusGroup` for keyboard navigation.
/// Uses `use_presence` for animation-aware mount/unmount.
#[component]
pub fn MenuContent(props: MenuContentProps) -> Element {
    let ctx: MenuCtx = use_context();
    let id = use_id_or(ctx.content_id, props.id);
    let mut presence = use_presence(ctx.open, id);

    // Document-level Escape listener so the menu closes even when
    // focus is not inside the content div.
    {
        let on_close = ctx.on_close;
        let on_escape_override = props.on_escape_override;
        let open = ctx.open;
        use_global_escape_listener(move || {
            if *open.peek() {
                if let Some(on_esc) = on_escape_override {
                    on_esc.call(());
                } else {
                    on_close.call(());
                }
            }
        });
    }

    if !presence.is_present() && !props.force_mount {
        return rsx! {};
    }

    let slot = format!("{}-content", ctx.slot_prefix);
    let on_close = ctx.on_close;
    let on_escape_override = props.on_escape_override;
    let on_arrow_left = props.on_arrow_left;
    let on_arrow_right = props.on_arrow_right;
    let children = props.children;
    let class = props.class;
    let user_attrs = props.attributes;
    let extra_attrs = props.extra_attributes;
    let trigger_id = ctx.trigger_id;

    rsx! {
        RovingFocusGroup {
            orientation: Signal::new(Some(Orientation::Vertical)),
            r#loop: Signal::new(true),
            r#as: {
                let children = children.clone();
                let class = class.clone();
                let user_attrs = user_attrs.clone();
                let extra_attrs = extra_attrs.clone();
                let slot = slot.clone();
                move |roving_attrs: Vec<Attribute>| {
                    let content_attrs = attributes!(div {
                        id: id,
                        role: "menu",
                        "data-slot": slot.clone(),
                        "data-state": presence.data_state(),
                        aria_orientation: "vertical",
                        aria_labelledby: (trigger_id)(),
                        class: class.clone(),
                    });
                    let merged = merge_attributes(vec![roving_attrs, content_attrs, extra_attrs.clone(), user_attrs.clone()]);

                    rsx! {
                        div {
                            onanimationend: move |_| presence.on_animation_end(),
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
                                    _ => {}
                                }
                            },
                            onfocusout: move |_| {
                                let id_str = id();
                                let open = ctx.open;
                                let on_close = ctx.on_close;
                                spawn(async move {
                                    // Wait a frame for focus to settle
                                    let js = format!(
                                        "var e=document.getElementById('{id_str}');\
                                         dioxus.send(e?e.contains(document.activeElement):false)"
                                    );
                                    let mut eval = document::eval(&js);
                                    if let Ok(still_inside) = eval.recv::<bool>().await {
                                        if !still_inside && *open.peek() {
                                            on_close.call(());
                                        }
                                    }
                                });
                            },
                            onpointerdown: move |event| {
                                event.prevent_default();
                                event.stop_propagation();
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
// MenuItem
// ---------------------------------------------------------------------------

/// Props for [`MenuItem`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuItemProps {
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
    let ctx: MenuCtx = use_context();
    let slot = format!("{}-item", ctx.slot_prefix);
    let disabled = props.disabled;
    let on_close = ctx.on_close;
    let class = props.class;
    let user_attrs = props.attributes;
    let children = props.children;

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
                            onmounted: move |e| slot_props.on_mounted.call(e),
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
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let content_id = use_unique_id();
    let trigger_id = use_unique_id();

    use_context_provider(|| MenuSubCtx {
        open,
        set_open,
        content_id,
        trigger_id,
    });

    rsx! { {props.children} }
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
#[component]
pub fn MenuSubTrigger(props: MenuSubTriggerProps) -> Element {
    let ctx: MenuCtx = use_context();
    let sub_ctx: MenuSubCtx = use_context();
    let slot = format!("{}-sub-trigger", ctx.slot_prefix);
    let disabled = props.disabled;
    let is_open = sub_ctx.open;
    let class = props.class;
    let user_attrs = props.attributes;
    let children = props.children;

    let mut hover_timer = use_signal(|| None::<i32>);

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
                            onmounted: move |e| slot_props.on_mounted.call(e),
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
                                    // Clear any pending timer
                                    if let Some(timer_id) = hover_timer() {
                                        let js = format!("clearTimeout({timer_id})");
                                        document::eval(&js);
                                    }
                                    // Open after 300ms delay
                                    let mut eval = document::eval(
                                        "dioxus.send(setTimeout(() => {}, 300))"
                                    );
                                    spawn(async move {
                                        if let Ok(timer_id) = eval.recv::<i32>().await {
                                            hover_timer.set(Some(timer_id));
                                            // Wait the actual delay
                                            let mut delay = document::eval(
                                                "setTimeout(() => dioxus.send(true), 300)"
                                            );
                                            if delay.recv::<bool>().await.is_ok() {
                                                sub_ctx.set_open.call(true);
                                            }
                                        }
                                    });
                                }
                            },
                            onpointerleave: move |_| {
                                if let Some(timer_id) = hover_timer() {
                                    let js = format!("clearTimeout({timer_id})");
                                    document::eval(&js);
                                    hover_timer.set(None);
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
    let children = props.children;
    let class = props.class;
    let user_attrs = props.attributes;

    rsx! {
        RovingFocusGroup {
            orientation: Signal::new(Some(Orientation::Vertical)),
            r#loop: Signal::new(true),
            r#as: {
                let children = children.clone();
                let class = class.clone();
                let user_attrs = user_attrs.clone();
                let slot = slot.clone();
                move |roving_attrs: Vec<Attribute>| {
                    let content_attrs = attributes!(div {
                        id: id,
                        role: "menu",
                        "data-slot": slot.clone(),
                        "data-state": presence.data_state(),
                        aria_orientation: "vertical",
                        aria_labelledby: sub_ctx.trigger_id.cloned(),
                        class: class.clone(),
                    });
                    let merged = merge_attributes(vec![roving_attrs, content_attrs, user_attrs.clone()]);

                    rsx! {
                        div {
                            onanimationend: move |_| presence.on_animation_end(),
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
