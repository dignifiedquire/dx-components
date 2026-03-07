//! ToggleGroup primitive — matches `@radix-ui/react-toggle-group`.
//!
//! A group of toggle buttons where selection depends on the type:
//! single (radio-like) or multiple (independent toggles).

use crate::direction::{Direction, Orientation};
use crate::merge_attributes;
use crate::roving_focus::{RovingFocusGroup, RovingFocusGroupItem, RovingFocusSlotProps};
use crate::use_controlled;
use dioxus::prelude::*;
use dioxus_attributes::attributes;

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// Whether the toggle group allows single or multiple selections.
///
/// Matches Radix's `type` prop.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ToggleGroupType {
    /// Only one item can be pressed at a time (radio-like).
    #[default]
    Single,
    /// Multiple items can be pressed independently.
    Multiple,
}

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct ToggleGroupCtx {
    type_: ToggleGroupType,
    value: Memo<Vec<String>>,
    on_item_activate: Callback<String>,
    on_item_deactivate: Callback<String>,
    disabled: bool,
    roving_focus: bool,
}

// ---------------------------------------------------------------------------
// ToggleGroup (root)
// ---------------------------------------------------------------------------

/// Props for [`ToggleGroup`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupProps {
    /// Whether this is a single or multiple selection group.
    #[props(default)]
    pub type_: ToggleGroupType,

    /// Controlled value. Pass `None` for uncontrolled.
    #[props(default)]
    pub value: ReadSignal<Option<Vec<String>>>,

    /// Default value when uncontrolled.
    #[props(default)]
    pub default_value: Vec<String>,

    /// Callback when the value changes.
    #[props(default)]
    pub on_value_change: Callback<Vec<String>>,

    /// Whether the entire group is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether RovingFocusGroup keyboard navigation is enabled.
    #[props(default = true)]
    pub roving_focus: bool,

    /// Layout orientation. Defaults to `Horizontal` (matching Radix).
    #[props(default = Orientation::Horizontal)]
    pub orientation: Orientation,

    /// Text direction for RTL support.
    #[props(default)]
    pub dir: Direction,

    /// Whether keyboard navigation loops. Defaults to `true`.
    #[props(default = true)]
    pub r#loop: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// A group of toggle buttons with single or multiple selection.
///
/// Matches Radix's `ToggleGroup`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::toggle_group::*;
/// rsx! {
///     ToggleGroup { type_: ToggleGroupType::Multiple,
///         ToggleGroupItem { value: "bold", "B" }
///         ToggleGroupItem { value: "italic", "I" }
///         ToggleGroupItem { value: "underline", "U" }
///     }
/// };
/// ```
#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let type_ = props.type_;

    let on_item_activate = use_callback(move |item_value: String| match type_ {
        ToggleGroupType::Single => {
            set_value.call(vec![item_value]);
        }
        ToggleGroupType::Multiple => {
            let mut current = (value)();
            current.push(item_value);
            set_value.call(current);
        }
    });

    let on_item_deactivate = use_callback(move |item_value: String| match type_ {
        ToggleGroupType::Single => {
            set_value.call(vec![]);
        }
        ToggleGroupType::Multiple => {
            let current = (value)();
            let filtered: Vec<String> = current.into_iter().filter(|v| v != &item_value).collect();
            set_value.call(filtered);
        }
    });

    use_context_provider(|| ToggleGroupCtx {
        type_,
        value,
        on_item_activate,
        on_item_deactivate,
        disabled: props.disabled,
        roving_focus: props.roving_focus,
    });

    let class = props.class;
    let user_attrs = props.attributes;
    let children = props.children;
    let orientation = props.orientation;
    let disabled = props.disabled;

    if props.roving_focus {
        rsx! {
            RovingFocusGroup {
                orientation: Signal::new(Some(orientation)),
                dir: Signal::new(props.dir),
                r#loop: Signal::new(props.r#loop),
                r#as: {
                    let class = class.clone();
                    let user_attrs = user_attrs.clone();
                    let children = children.clone();
                    move |roving_attrs: Vec<Attribute>| {
                        let group_attrs = attributes!(div {
                            role: "group",
                            "data-slot": "toggle-group",
                            "data-orientation": orientation.as_str(),
                            "data-disabled": if disabled { "" },
                            class: class.clone(),
                        });
                        let merged = merge_attributes(vec![roving_attrs, group_attrs, user_attrs.clone()]);

                        rsx! {
                            div { ..merged, {children.clone()} }
                        }
                    }
                },
            }
        }
    } else {
        rsx! {
            div {
                role: "group",
                "data-slot": "toggle-group",
                "data-orientation": orientation.as_str(),
                "data-disabled": if disabled { "" },
                class: class,
                ..user_attrs,
                {children}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ToggleGroupItem
// ---------------------------------------------------------------------------

/// Props for [`ToggleGroupItem`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupItemProps {
    /// The value that identifies this item.
    pub value: String,

    /// Whether this item is disabled.
    #[props(default)]
    pub disabled: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// An individual toggle button within a [`ToggleGroup`].
///
/// Matches Radix's `ToggleGroupItem`. In single mode uses `role="radio"` +
/// `aria-checked`; in multiple mode uses `aria-pressed`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::toggle_group::*;
/// rsx! {
///     ToggleGroup { type_: ToggleGroupType::Multiple,
///         ToggleGroupItem { value: "bold", "B" }
///         ToggleGroupItem { value: "italic", "I" }
///     }
/// };
/// ```
#[component]
pub fn ToggleGroupItem(props: ToggleGroupItemProps) -> Element {
    let ctx: ToggleGroupCtx = use_context();
    let is_disabled = ctx.disabled || props.disabled;

    let item_value = props.value.clone();
    let pressed = use_memo(move || (ctx.value)().contains(&item_value));

    let class = props.class;
    let user_attrs = props.attributes;
    let children = props.children;
    let click_value = props.value.clone();

    // Build ARIA attrs based on single vs. multiple mode
    let build_item_attrs = {
        let class = class.clone();
        move || match ctx.type_ {
            ToggleGroupType::Single => attributes!(button {
                r#type: "button",
                role: "radio",
                "data-slot": "toggle-group-item",
                "data-state": if pressed() { "on" } else { "off" },
                "data-disabled": if is_disabled { "" },
                aria_checked: pressed(),
                disabled: is_disabled,
                class: class.clone(),
            }),
            ToggleGroupType::Multiple => attributes!(button {
                r#type: "button",
                "data-slot": "toggle-group-item",
                "data-state": if pressed() { "on" } else { "off" },
                "data-disabled": if is_disabled { "" },
                aria_pressed: pressed(),
                disabled: is_disabled,
                class: class.clone(),
            }),
        }
    };

    let on_click = use_callback({
        let click_value = click_value.clone();
        move |_: Event<MouseData>| {
            if !is_disabled {
                if pressed() {
                    ctx.on_item_deactivate.call(click_value.clone());
                } else {
                    ctx.on_item_activate.call(click_value.clone());
                }
            }
        }
    });

    if ctx.roving_focus {
        // With RovingFocusGroup — compose event handlers
        rsx! {
            RovingFocusGroupItem {
                focusable: !is_disabled,
                active: pressed(),
                r#as: {
                    let user_attrs = user_attrs.clone();
                    let children = children.clone();
                    let build_item_attrs = build_item_attrs.clone();
                    move |slot: RovingFocusSlotProps| {
                        let item_attrs = build_item_attrs();
                        let merged = merge_attributes(vec![slot.attributes.clone(), item_attrs, user_attrs.clone()]);

                        rsx! {
                            button {
                                onmounted: move |e| slot.on_mounted.call(e),
                                onmousedown: move |event: MouseEvent| {
                                    slot.on_mousedown.call(event);
                                },
                                onkeydown: move |event: KeyboardEvent| {
                                    slot.on_keydown.call(event);
                                },
                                onfocus: move |event: FocusEvent| {
                                    slot.on_focus.call(event);
                                },
                                onclick: on_click,
                                ..merged,
                                {children.clone()}
                            }
                        }
                    }
                },
            }
        }
    } else {
        // Without RovingFocusGroup — plain button (e.g., inside Toolbar)
        let item_attrs = build_item_attrs();
        let merged = merge_attributes(vec![item_attrs, user_attrs]);

        rsx! {
            button {
                onclick: on_click,
                ..merged,
                {children}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Backward-compat aliases
// ---------------------------------------------------------------------------

/// Alias for [`ToggleGroupItem`] (old name).
pub use ToggleGroupItem as ToggleItem;
