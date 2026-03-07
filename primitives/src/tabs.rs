//! Tabs primitive — matches `@radix-ui/react-tabs`.
//!
//! A set of layered sections of content, known as tab panels, that are displayed
//! one at a time.

use crate::direction::{Direction, Orientation};
use crate::merge_attributes;
use crate::roving_focus::{RovingFocusGroup, RovingFocusGroupItem, RovingFocusSlotProps};
use crate::{use_controlled, use_unique_id};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use dioxus_attributes::attributes;

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// Controls whether a tab is activated automatically on focus or manually.
///
/// Matches Radix's `activationMode` prop.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ActivationMode {
    /// Tab activates when its trigger receives focus (arrow key nav activates).
    #[default]
    Automatic,
    /// Tab activates only on explicit click / Enter / Space.
    Manual,
}

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct TabsCtx {
    base_id: String,
    value: Memo<String>,
    set_value: Callback<String>,
    orientation: Orientation,
    dir: Direction,
    activation_mode: ActivationMode,
}

// ---------------------------------------------------------------------------
// ID helpers (matches Radix's makeTriggerId / makeContentId)
// ---------------------------------------------------------------------------

fn make_trigger_id(base_id: &str, value: &str) -> String {
    format!("{base_id}-trigger-{value}")
}

fn make_content_id(base_id: &str, value: &str) -> String {
    format!("{base_id}-content-{value}")
}

// ---------------------------------------------------------------------------
// Tabs (root)
// ---------------------------------------------------------------------------

/// Props for [`Tabs`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// Controlled active tab value. Pass `None` for uncontrolled.
    #[props(default)]
    pub value: ReadSignal<Option<String>>,

    /// Default active tab when uncontrolled.
    #[props(default)]
    pub default_value: String,

    /// Callback when the active tab changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Layout orientation. Defaults to `Horizontal` (matching Radix).
    #[props(default = Orientation::Horizontal)]
    pub orientation: Orientation,

    /// Text direction for RTL support. Defaults to `Ltr`.
    #[props(default)]
    pub dir: Direction,

    /// Whether tabs activate on focus or only on click.
    #[props(default)]
    pub activation_mode: ActivationMode,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// A set of layered sections of content displayed one at a time.
///
/// Matches Radix's `Tabs`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::tabs::*;
/// rsx! {
///     Tabs { default_value: "tab1".to_string(),
///         TabsList {
///             TabsTrigger { value: "tab1".to_string(), "Tab 1" }
///             TabsTrigger { value: "tab2".to_string(), "Tab 2" }
///         }
///         TabsContent { value: "tab1".to_string(), "Content 1" }
///         TabsContent { value: "tab2".to_string(), "Content 2" }
///     }
/// };
/// ```
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let base_id_signal = use_unique_id();
    let base_id = use_hook(|| base_id_signal.cloned());

    use_context_provider(|| TabsCtx {
        base_id,
        value,
        set_value,
        orientation: props.orientation,
        dir: props.dir,
        activation_mode: props.activation_mode,
    });

    rsx! {
        div {
            "data-slot": "tabs",
            "data-orientation": props.orientation.as_str(),
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// TabsList
// ---------------------------------------------------------------------------

/// Props for [`TabsList`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct TabsListProps {
    /// Whether keyboard navigation loops. Defaults to `true`.
    #[props(default = true)]
    pub r#loop: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// Contains the triggers for a set of tabs.
///
/// Matches Radix's `TabsList`. Delegates keyboard navigation to
/// [`RovingFocusGroup`] via `r#as` (asChild pattern), matching Radix's
/// composition of `RovingFocusGroup.Root asChild` wrapping a `<div>`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::tabs::*;
/// rsx! {
///     Tabs { default_value: "tab1".to_string(),
///         TabsList {
///             TabsTrigger { value: "tab1".to_string(), "Tab 1" }
///             TabsTrigger { value: "tab2".to_string(), "Tab 2" }
///         }
///         TabsContent { value: "tab1".to_string(), "Content 1" }
///         TabsContent { value: "tab2".to_string(), "Content 2" }
///     }
/// };
/// ```
#[component]
pub fn TabsList(props: TabsListProps) -> Element {
    let ctx: TabsCtx = use_context();
    let children = props.children;
    let class = props.class;
    let user_attrs = props.attributes;

    rsx! {
        RovingFocusGroup {
            orientation: Signal::new(Some(ctx.orientation)),
            dir: Signal::new(ctx.dir),
            r#loop: Signal::new(props.r#loop),
            r#as: {
                let children = children.clone();
                let class = class.clone();
                let user_attrs = user_attrs.clone();
                move |roving_attrs: Vec<Attribute>| {
                    let list_attrs = attributes!(div {
                        role: "tablist",
                        "data-slot": "tabs-list",
                        aria_orientation: ctx.orientation.as_str(),
                        class: class.clone(),
                    });
                    let merged = merge_attributes(vec![roving_attrs, list_attrs, user_attrs.clone()]);

                    rsx! {
                        div { ..merged, {children.clone()} }
                    }
                }
            },
        }
    }
}

// ---------------------------------------------------------------------------
// TabsTrigger
// ---------------------------------------------------------------------------

/// Props for [`TabsTrigger`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct TabsTriggerProps {
    /// The value that associates this trigger with its content panel.
    pub value: String,

    /// Whether this trigger is disabled.
    #[props(default)]
    pub disabled: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// The button that activates a tab panel.
///
/// Matches Radix's `TabsTrigger`. Delegates focus management to
/// [`RovingFocusGroupItem`] via `r#as` (asChild pattern), composing
/// tab-specific event handlers with roving focus handlers.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::tabs::*;
/// rsx! {
///     Tabs { default_value: "tab1".to_string(),
///         TabsList {
///             TabsTrigger { value: "tab1".to_string(), "Tab 1" }
///             TabsTrigger { value: "tab2".to_string(), "Tab 2" }
///         }
///         TabsContent { value: "tab1".to_string(), "Content 1" }
///         TabsContent { value: "tab2".to_string(), "Content 2" }
///     }
/// };
/// ```
#[component]
pub fn TabsTrigger(props: TabsTriggerProps) -> Element {
    let ctx: TabsCtx = use_context();

    let trigger_value = props.value.clone();
    let is_selected = use_memo(move || (ctx.value)() == trigger_value);

    let trigger_id = make_trigger_id(&ctx.base_id, &props.value);
    let content_id = make_content_id(&ctx.base_id, &props.value);
    let disabled = props.disabled;
    let class = props.class;
    let user_attrs = props.attributes;
    let children = props.children;

    let mousedown_value = props.value.clone();
    let keydown_value = props.value.clone();
    let focus_value = props.value.clone();

    rsx! {
        RovingFocusGroupItem {
            focusable: !disabled,
            active: is_selected(),
            r#as: {
                let trigger_id = trigger_id.clone();
                let content_id = content_id.clone();
                let class = class.clone();
                let user_attrs = user_attrs.clone();
                let children = children.clone();
                let mousedown_value = mousedown_value.clone();
                let keydown_value = keydown_value.clone();
                let focus_value = focus_value.clone();
                move |slot: RovingFocusSlotProps| {
                    // Build tab-specific attributes (override roving defaults like data-slot)
                    let tab_attrs = attributes!(button {
                        r#type: "button",
                        role: "tab",
                        id: trigger_id.clone(),
                        "data-slot": "tabs-trigger",
                        "data-state": if is_selected() { "active" } else { "inactive" },
                        "data-disabled": if disabled { "" },
                        "data-orientation": ctx.orientation.as_str(),
                        aria_selected: is_selected(),
                        aria_controls: content_id.clone(),
                        disabled: disabled,
                        class: class.clone(),
                    });
                    // Roving attrs first, then tab overrides, then user attrs last
                    let merged = merge_attributes(vec![slot.attributes, tab_attrs, user_attrs.clone()]);

                    let mousedown_value = mousedown_value.clone();
                    let keydown_value = keydown_value.clone();
                    let focus_value = focus_value.clone();

                    rsx! {
                        button {
                            // Compose event handlers: tab-specific first, then roving focus
                            onmounted: move |e| slot.on_mounted.call(e),
                            onmousedown: {
                                let mousedown_value = mousedown_value.clone();
                                move |event: MouseEvent| {
                                    if !disabled
                                        && event.trigger_button() == Some(MouseButton::Primary)
                                        && !event.modifiers().ctrl()
                                    {
                                        ctx.set_value.call(mousedown_value.clone());
                                    } else if disabled {
                                        event.prevent_default();
                                    }
                                    slot.on_mousedown.call(event);
                                }
                            },
                            onkeydown: {
                                let keydown_value = keydown_value.clone();
                                move |event: KeyboardEvent| {
                                    if matches!(event.key(), Key::Character(ref c) if c == " ")
                                        || event.key() == Key::Enter
                                    {
                                        ctx.set_value.call(keydown_value.clone());
                                    }
                                    slot.on_keydown.call(event);
                                }
                            },
                            onfocus: {
                                let focus_value = focus_value.clone();
                                move |event: FocusEvent| {
                                    if ctx.activation_mode == ActivationMode::Automatic
                                        && !disabled
                                        && !is_selected()
                                    {
                                        ctx.set_value.call(focus_value.clone());
                                    }
                                    slot.on_focus.call(event);
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
// TabsContent
// ---------------------------------------------------------------------------

/// Props for [`TabsContent`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct TabsContentProps {
    /// The value that associates this content with its trigger.
    pub value: String,

    /// Keep content mounted even when inactive.
    #[props(default)]
    pub force_mount: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// The content panel for a tab.
///
/// Matches Radix's `TabsContent`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::tabs::*;
/// rsx! {
///     Tabs { default_value: "tab1".to_string(),
///         TabsList {
///             TabsTrigger { value: "tab1".to_string(), "Tab 1" }
///             TabsTrigger { value: "tab2".to_string(), "Tab 2" }
///         }
///         TabsContent { value: "tab1".to_string(), "Content 1" }
///         TabsContent { value: "tab2".to_string(), "Content 2" }
///     }
/// };
/// ```
#[component]
pub fn TabsContent(props: TabsContentProps) -> Element {
    let ctx: TabsCtx = use_context();

    let content_value = props.value.clone();
    let is_selected = use_memo(move || (ctx.value)() == content_value);

    let trigger_id = make_trigger_id(&ctx.base_id, &props.value);
    let content_id = make_content_id(&ctx.base_id, &props.value);

    let is_present = is_selected() || props.force_mount;

    rsx! {
        div {
            role: "tabpanel",
            id: content_id,
            "data-slot": "tabs-content",
            "data-state": if is_selected() { "active" } else { "inactive" },
            "data-orientation": ctx.orientation.as_str(),
            aria_labelledby: trigger_id,
            tabindex: "0",
            hidden: !is_selected(),
            class: props.class,
            ..props.attributes,

            if is_present {
                {props.children}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Backward-compat aliases
// ---------------------------------------------------------------------------

/// Alias for [`TabsList`] (old name).
pub use TabsList as TabList;

/// Alias for [`TabsTrigger`] (old name).
pub use TabsTrigger as TabTrigger;

/// Alias for [`TabsContent`] (old name).
pub use TabsContent as TabContent;
