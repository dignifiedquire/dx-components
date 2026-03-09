//! Styled toolbar component.
//!
//! Wraps the unstyled `dioxus_primitives::toolbar` primitives with
//! Tailwind classes. There is no shadcn toolbar equivalent — these
//! styles are Dioxus-specific.

use dioxus::prelude::*;
pub use dioxus_primitives::direction::{Direction, Orientation};
pub use dioxus_primitives::toggle_group::{ToggleGroupItem, ToggleGroupType};
use dioxus_primitives::toolbar as primitives;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Toolbar (styled root)
// ---------------------------------------------------------------------------

/// The props for the styled [`Toolbar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarProps {
    /// The orientation of the toolbar.
    #[props(default = Orientation::Horizontal)]
    pub orientation: Orientation,

    /// Text direction for RTL support.
    #[props(default)]
    pub dir: Direction,

    /// Whether keyboard navigation loops.
    #[props(default = true)]
    pub r#loop: bool,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the toolbar.
    pub children: Element,
}

/// Styled Toolbar root.
#[component]
pub fn Toolbar(props: ToolbarProps) -> Element {
    let class = tw_merge!(
        "flex items-center gap-1 rounded-md border bg-background p-1",
        props.class,
    );

    rsx! {
        primitives::Toolbar {
            orientation: props.orientation,
            dir: props.dir,
            r#loop: props.r#loop,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ToolbarButton (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`ToolbarButton`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarButtonProps {
    /// Whether the button is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the button element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the button.
    pub children: Element,
}

/// Styled ToolbarButton — passthrough wrapper.
#[component]
pub fn ToolbarButton(props: ToolbarButtonProps) -> Element {
    rsx! {
        primitives::ToolbarButton {
            disabled: props.disabled,
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ToolbarLink (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`ToolbarLink`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarLinkProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the link element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the link.
    pub children: Element,
}

/// Styled ToolbarLink — passthrough wrapper.
#[component]
pub fn ToolbarLink(props: ToolbarLinkProps) -> Element {
    rsx! {
        primitives::ToolbarLink {
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ToolbarSeparator (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`ToolbarSeparator`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarSeparatorProps {
    /// Whether the separator is purely decorative.
    #[props(default)]
    pub decorative: bool,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the separator element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled ToolbarSeparator — adds border styling.
#[component]
pub fn ToolbarSeparator(props: ToolbarSeparatorProps) -> Element {
    let class = tw_merge!("shrink-0 bg-border w-px h-5", props.class);

    rsx! {
        primitives::ToolbarSeparator {
            decorative: props.decorative,
            class: class,
            attributes: props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// ToolbarToggleGroup (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`ToolbarToggleGroup`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarToggleGroupProps {
    /// Selection mode — single or multiple.
    #[props(default)]
    pub type_: ToggleGroupType,

    /// The controlled value of selected items.
    #[props(default)]
    pub value: ReadSignal<Option<Vec<String>>>,

    /// The default selected values when uncontrolled.
    #[props(default)]
    pub default_value: Vec<String>,

    /// Callback fired when the selected values change.
    #[props(default)]
    pub on_value_change: Callback<Vec<String>>,

    /// Whether the entire group is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the group element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the toggle group.
    pub children: Element,
}

/// Styled ToolbarToggleGroup — passthrough wrapper.
#[component]
pub fn ToolbarToggleGroup(props: ToolbarToggleGroupProps) -> Element {
    rsx! {
        primitives::ToolbarToggleGroup {
            type_: props.type_,
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
