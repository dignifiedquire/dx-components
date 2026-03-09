//! Styled collapsible matching shadcn/ui.
//!
//! Thin wrapper around `dioxus_primitives::collapsible` — shadcn's collapsible
//! adds only `data-slot` attributes (which our primitive already provides).

use dioxus::prelude::*;
use dioxus_primitives::collapsible as primitives;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Collapsible (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`Collapsible`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleProps {
    /// The controlled `open` state. Pass `None` for uncontrolled.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// The default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Whether the collapsible is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Callback fired when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled Collapsible root — thin passthrough matching shadcn.
#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
    rsx! {
        primitives::Collapsible {
            open: props.open,
            default_open: props.default_open,
            disabled: props.disabled,
            on_open_change: props.on_open_change,
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CollapsibleTrigger (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`CollapsibleTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleTriggerProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled CollapsibleTrigger — thin passthrough matching shadcn.
#[component]
pub fn CollapsibleTrigger(props: CollapsibleTriggerProps) -> Element {
    rsx! {
        primitives::CollapsibleTrigger {
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CollapsibleContent (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`CollapsibleContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleContentProps {
    /// When `true`, children stay mounted in the DOM even when collapsed.
    #[props(default)]
    pub force_mount: bool,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled CollapsibleContent — thin passthrough matching shadcn.
#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element {
    let class = tw_merge!(props.class);

    rsx! {
        primitives::CollapsibleContent {
            force_mount: props.force_mount,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
