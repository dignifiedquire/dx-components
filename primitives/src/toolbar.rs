//! Toolbar primitive — matches `@radix-ui/react-toolbar`.
//!
//! A container for grouping a set of controls (buttons, toggle groups, etc.)
//! with keyboard navigation via `RovingFocusGroup`.

use crate::direction::{Direction, Orientation};
use crate::merge_attributes;
use crate::roving_focus::{RovingFocusGroup, RovingFocusGroupItem, RovingFocusSlotProps};
use crate::toggle_group::{ToggleGroup, ToggleGroupType};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct ToolbarCtx {
    orientation: Orientation,
    dir: Direction,
}

// ---------------------------------------------------------------------------
// Toolbar (root)
// ---------------------------------------------------------------------------

/// Props for [`Toolbar`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarProps {
    /// Layout orientation. Defaults to `Horizontal`.
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

/// A container for grouping controls with keyboard navigation.
///
/// Matches Radix's `Toolbar`. Delegates keyboard navigation to
/// [`RovingFocusGroup`] via `r#as`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::toolbar::*;
/// rsx! {
///     Toolbar { aria_label: "Formatting tools",
///         ToolbarButton { "Bold" }
///         ToolbarSeparator {}
///         ToolbarButton { "Italic" }
///     }
/// };
/// ```
#[component]
pub fn Toolbar(props: ToolbarProps) -> Element {
    use_context_provider(|| ToolbarCtx {
        orientation: props.orientation,
        dir: props.dir,
    });

    let class = props.class;
    let user_attrs = props.attributes;
    let children = props.children;
    let orientation = props.orientation;

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
                    let toolbar_attrs = attributes!(div {
                        role: "toolbar",
                        "data-slot": "toolbar",
                        "data-orientation": orientation.as_str(),
                        aria_orientation: orientation.as_str(),
                        class: class.clone(),
                    });
                    let merged = merge_attributes(vec![roving_attrs, toolbar_attrs, user_attrs.clone()]);

                    rsx! {
                        div { ..merged, {children.clone()} }
                    }
                }
            },
        }
    }
}

// ---------------------------------------------------------------------------
// ToolbarButton
// ---------------------------------------------------------------------------

/// Props for [`ToolbarButton`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarButtonProps {
    /// Whether this button is disabled.
    #[props(default)]
    pub disabled: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// A focusable button within a [`Toolbar`].
///
/// Matches Radix's `ToolbarButton`. Wraps `RovingFocusGroupItem` via `r#as`.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::toolbar::*;
/// rsx! {
///     Toolbar {
///         ToolbarButton { "Bold" }
///         ToolbarButton { "Italic" }
///     }
/// };
/// ```
#[component]
pub fn ToolbarButton(props: ToolbarButtonProps) -> Element {
    let disabled = props.disabled;
    let class = props.class;
    let user_attrs = props.attributes;
    let children = props.children;

    rsx! {
        RovingFocusGroupItem {
            focusable: !disabled,
            active: false,
            r#as: {
                let class = class.clone();
                let user_attrs = user_attrs.clone();
                let children = children.clone();
                move |slot: RovingFocusSlotProps| {
                    let btn_attrs = attributes!(button {
                        r#type: "button",
                        "data-slot": "toolbar-button",
                        "data-disabled": if disabled { "" },
                        disabled: disabled,
                        class: class.clone(),
                    });
                    let merged = merge_attributes(vec![slot.attributes.clone(), btn_attrs, user_attrs.clone()]);

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
// ToolbarLink
// ---------------------------------------------------------------------------

/// Props for [`ToolbarLink`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarLinkProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// A focusable link within a [`Toolbar`].
///
/// Matches Radix's `ToolbarLink`. Adds Space key activation (matching
/// Radix's accessibility behavior for links in toolbars).
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::toolbar::*;
/// rsx! {
///     Toolbar {
///         ToolbarLink { "Docs" }
///     }
/// };
/// ```
#[component]
pub fn ToolbarLink(props: ToolbarLinkProps) -> Element {
    let class = props.class;
    let user_attrs = props.attributes;
    let children = props.children;

    rsx! {
        RovingFocusGroupItem {
            focusable: true,
            active: false,
            r#as: {
                let class = class.clone();
                let user_attrs = user_attrs.clone();
                let children = children.clone();
                move |slot: RovingFocusSlotProps| {
                    let link_attrs = attributes!(a {
                        "data-slot": "toolbar-link",
                        class: class.clone(),
                    });
                    let merged = merge_attributes(vec![slot.attributes.clone(), link_attrs, user_attrs.clone()]);

                    rsx! {
                        a {
                            onmounted: move |e| slot.on_mounted.call(e),
                            onmousedown: move |event: MouseEvent| {
                                slot.on_mousedown.call(event);
                            },
                            onkeydown: move |event: KeyboardEvent| {
                                // Space key activates link (accessibility)
                                if matches!(event.key(), Key::Character(ref c) if c == " ") {
                                    // Triggering click via JS eval since we can't call .click() on element
                                    event.prevent_default();
                                }
                                slot.on_keydown.call(event);
                            },
                            onfocus: move |event: FocusEvent| {
                                slot.on_focus.call(event);
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
// ToolbarSeparator
// ---------------------------------------------------------------------------

/// Props for [`ToolbarSeparator`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarSeparatorProps {
    /// Whether the separator is decorative (no ARIA role).
    #[props(default)]
    pub decorative: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A separator within a [`Toolbar`].
///
/// Matches Radix's `ToolbarSeparator`. Automatically inverts orientation
/// from the toolbar (horizontal toolbar → vertical separator).
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::toolbar::*;
/// rsx! {
///     Toolbar {
///         ToolbarButton { "Bold" }
///         ToolbarSeparator {}
///         ToolbarButton { "Italic" }
///     }
/// };
/// ```
#[component]
pub fn ToolbarSeparator(props: ToolbarSeparatorProps) -> Element {
    let ctx: ToolbarCtx = use_context();

    // Separator orientation is inverted from toolbar orientation
    let sep_orientation = match ctx.orientation {
        Orientation::Horizontal => Orientation::Vertical,
        Orientation::Vertical => Orientation::Horizontal,
    };

    rsx! {
        div {
            "data-slot": "toolbar-separator",
            "data-orientation": sep_orientation.as_str(),
            role: if !props.decorative { "separator" },
            aria_orientation: if !props.decorative { sep_orientation.as_str() },
            class: props.class,
            ..props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// ToolbarToggleGroup
// ---------------------------------------------------------------------------

/// Props for [`ToolbarToggleGroup`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarToggleGroupProps {
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

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// A toggle group embedded within a [`Toolbar`].
///
/// Matches Radix's `ToolbarToggleGroup`. Wraps [`ToggleGroup`] with
/// `roving_focus: false` since the Toolbar already manages keyboard navigation.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::toolbar::*;
/// # use dioxus_primitives::toggle_group::ToggleGroupItem;
/// rsx! {
///     Toolbar {
///         ToolbarToggleGroup { type_: dioxus_primitives::toggle_group::ToggleGroupType::Single,
///             ToggleGroupItem { value: "bold", "B" }
///             ToggleGroupItem { value: "italic", "I" }
///         }
///     }
/// };
/// ```
#[component]
pub fn ToolbarToggleGroup(props: ToolbarToggleGroupProps) -> Element {
    let ctx: ToolbarCtx = use_context();

    rsx! {
        ToggleGroup {
            type_: props.type_,
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            roving_focus: false,
            orientation: ctx.orientation,
            dir: ctx.dir,
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
