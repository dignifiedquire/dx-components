//! RadioGroup primitive — matches `@radix-ui/react-radio-group`.
//!
//! A set of checkable buttons (radios) where only one can be checked at a time.

use crate::direction::{Direction, Orientation};
use crate::merge_attributes;
use crate::roving_focus::{RovingFocusGroup, RovingFocusGroupItem, RovingFocusSlotProps};
use crate::use_controlled;
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use dioxus_attributes::attributes;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct RadioGroupCtx {
    value: Memo<String>,
    set_value: Callback<String>,
    disabled: bool,
    required: bool,
    name: Option<String>,
}

#[derive(Clone)]
struct RadioCtx {
    checked: Memo<bool>,
    disabled: bool,
}

// ---------------------------------------------------------------------------
// RadioGroup (root)
// ---------------------------------------------------------------------------

/// Props for [`RadioGroup`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
    /// Controlled value. Pass `None` for uncontrolled.
    #[props(default)]
    pub value: ReadSignal<Option<String>>,

    /// Default value when uncontrolled.
    #[props(default)]
    pub default_value: String,

    /// Callback when the value changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Whether the entire group is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether a selection is required for form submission.
    #[props(default)]
    pub required: bool,

    /// The `name` attribute for hidden form inputs.
    #[props(default)]
    pub name: Option<String>,

    /// Layout orientation. Defaults to `Vertical` (matching Radix).
    #[props(default)]
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

/// A set of checkable buttons where only one can be checked at a time.
///
/// Matches Radix's `RadioGroup`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::radio_group::*;
/// rsx! {
///     RadioGroup { default_value: "a".to_string(),
///         RadioGroupItem { value: "a".to_string(), "Option A" }
///         RadioGroupItem { value: "b".to_string(), "Option B" }
///     }
/// };
/// ```
#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    use_context_provider(|| RadioGroupCtx {
        value,
        set_value,
        disabled: props.disabled,
        required: props.required,
        name: props.name.clone(),
    });

    let class = props.class;
    let user_attrs = props.attributes;
    let children = props.children;

    rsx! {
        RovingFocusGroup {
            orientation: Signal::new(Some(props.orientation)),
            dir: Signal::new(props.dir),
            r#loop: Signal::new(props.r#loop),
            r#as: {
                let class = class.clone();
                let user_attrs = user_attrs.clone();
                let children = children.clone();
                move |roving_attrs: Vec<Attribute>| {
                    let group_attrs = attributes!(div {
                        role: "radiogroup",
                        "data-slot": "radio-group",
                        "data-orientation": props.orientation.as_str(),
                        "data-disabled": if props.disabled { "" },
                        aria_required: props.required,
                        aria_orientation: props.orientation.as_str(),
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
}

// ---------------------------------------------------------------------------
// RadioGroupItem
// ---------------------------------------------------------------------------

/// Props for [`RadioGroupItem`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupItemProps {
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

/// An individual radio button within a [`RadioGroup`].
///
/// Matches Radix's `RadioGroupItem`. Wraps `RovingFocusGroupItem` via `r#as`
/// to compose keyboard navigation handlers.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::radio_group::*;
/// rsx! {
///     RadioGroup { default_value: "a".to_string(),
///         RadioGroupItem { value: "a".to_string(), "Option A" }
///         RadioGroupItem { value: "b".to_string(), "Option B" }
///     }
/// };
/// ```
#[component]
pub fn RadioGroupItem(props: RadioGroupItemProps) -> Element {
    let ctx: RadioGroupCtx = use_context();
    let is_disabled = ctx.disabled || props.disabled;

    let item_value = props.value.clone();
    let checked = use_memo(move || (ctx.value)() == item_value);

    use_context_provider(|| RadioCtx {
        checked,
        disabled: is_disabled,
    });

    let class = props.class;
    let user_attrs = props.attributes;
    let children = props.children;

    let click_value = props.value.clone();
    let focus_value = props.value.clone();

    rsx! {
        RovingFocusGroupItem {
            focusable: !is_disabled,
            active: checked(),
            r#as: {
                let class = class.clone();
                let user_attrs = user_attrs.clone();
                let children = children.clone();
                let click_value = click_value.clone();
                let focus_value = focus_value.clone();
                move |slot: RovingFocusSlotProps| {
                    let radio_attrs = attributes!(button {
                        r#type: "button",
                        role: "radio",
                        "data-slot": "radio-group-item",
                        "data-state": if checked() { "checked" } else { "unchecked" },
                        "data-disabled": if is_disabled { "" },
                        aria_checked: checked(),
                        disabled: is_disabled,
                        class: class.clone(),
                    });
                    let merged = merge_attributes(vec![slot.attributes.clone(), radio_attrs, user_attrs.clone()]);

                    let click_value = click_value.clone();
                    let focus_value = focus_value.clone();

                    rsx! {
                        button {
                            onmounted: move |e| slot.on_mounted.call(e),
                            onmousedown: {
                                let click_value = click_value.clone();
                                move |event: MouseEvent| {
                                    if !is_disabled
                                        && event.trigger_button() == Some(MouseButton::Primary)
                                        && !event.modifiers().ctrl()
                                    {
                                        ctx.set_value.call(click_value.clone());
                                    }
                                    slot.on_mousedown.call(event);
                                }
                            },
                            onkeydown: move |event: KeyboardEvent| {
                                // Radios don't activate on Enter (WAI-ARIA)
                                if event.key() == Key::Enter {
                                    event.prevent_default();
                                }
                                slot.on_keydown.call(event);
                            },
                            onfocus: {
                                let focus_value = focus_value.clone();
                                move |event: FocusEvent| {
                                    // Auto-check on focus via arrow keys (matching Radix behavior)
                                    if !is_disabled && !checked() {
                                        ctx.set_value.call(focus_value.clone());
                                    }
                                    slot.on_focus.call(event);
                                }
                            },
                            ..merged,
                            {children.clone()}
                        }

                        // Hidden input for form submission
                        if ctx.name.is_some() {
                            input {
                                r#type: "radio",
                                aria_hidden: true,
                                tabindex: "-1",
                                name: ctx.name.clone(),
                                value: click_value.clone(),
                                checked: checked(),
                                disabled: is_disabled,
                                required: ctx.required,
                                style: "position: absolute; pointer-events: none; opacity: 0; margin: 0; transform: translateX(-100%);",
                            }
                        }
                    }
                }
            },
        }
    }
}

// ---------------------------------------------------------------------------
// RadioGroupIndicator
// ---------------------------------------------------------------------------

/// Props for [`RadioGroupIndicator`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupIndicatorProps {
    /// When `true`, the indicator is always mounted regardless of checked state.
    #[props(default)]
    pub force_mount: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    #[props(default)]
    pub children: Element,
}

/// Visual indicator rendered inside a [`RadioGroupItem`] when checked.
///
/// Matches Radix's `RadioGroupIndicator`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::radio_group::*;
/// rsx! {
///     RadioGroup { default_value: "a".to_string(),
///         RadioGroupItem { value: "a".to_string(),
///             RadioGroupIndicator {}
///             "Option A"
///         }
///     }
/// };
/// ```
#[component]
pub fn RadioGroupIndicator(props: RadioGroupIndicatorProps) -> Element {
    let radio_ctx: RadioCtx = use_context();
    let is_present = (radio_ctx.checked)() || props.force_mount;

    rsx! {
        span {
            "data-slot": "radio-group-indicator",
            "data-state": if (radio_ctx.checked)() { "checked" } else { "unchecked" },
            "data-disabled": if radio_ctx.disabled { "" },
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

/// Alias for [`RadioGroupItem`] (old name).
pub use RadioGroupItem as RadioItem;
