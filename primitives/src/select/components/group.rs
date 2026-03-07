//! SelectGroup, SelectLabel (formerly SelectGroupLabel), and SelectSeparator component implementations.

use crate::{select::context::SelectListContext, use_effect, use_id_or, use_unique_id};
use dioxus::prelude::*;

use super::super::context::{SelectContext, SelectGroupContext};

/// The props for the [`SelectGroup`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectGroupProps {
    /// Whether the group is disabled
    #[props(default)]
    pub disabled: bool,

    /// Optional ID for the group
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the group
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the group
    pub children: Element,
}

/// A grouping element for select items. Has `role="group"`.
#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    let ctx = use_context::<SelectContext>();
    let disabled = ctx.disabled || props.disabled;

    let labeled_by = use_signal(|| None);

    use_context_provider(|| SelectGroupContext { labeled_by });
    let render = use_context::<SelectListContext>().render;

    rsx! {
        if render() {
            div {
                role: "group",
                "data-slot": "select-group",
                aria_disabled: if disabled { Some("true") } else { None },
                aria_labelledby: labeled_by,
                ..props.attributes,
                {props.children}
            }
        } else {
            {props.children}
        }
    }
}

/// The props for the [`SelectLabel`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectLabelProps {
    /// Optional ID for the label
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the label
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the label
    pub children: Element,
}

/// Backward-compatible alias.
pub type SelectGroupLabelProps = SelectLabelProps;

/// A non-interactive label for a group of select items.
#[component]
pub fn SelectLabel(props: SelectLabelProps) -> Element {
    let mut ctx: SelectGroupContext = use_context();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);

    use_effect(move || {
        ctx.labeled_by.set(Some(id()));
    });

    let render = use_context::<SelectListContext>().render;

    rsx! {
        if render() {
            div {
                "data-slot": "select-label",
                id,
                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// Backward-compatible alias for [`SelectLabel`].
#[component]
pub fn SelectGroupLabel(props: SelectLabelProps) -> Element {
    SelectLabel(props)
}

/// Props for [`SelectSeparator`].
#[derive(Props, Clone, PartialEq)]
pub struct SelectSeparatorProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A visual separator between select items. Has `aria-hidden`.
#[component]
pub fn SelectSeparator(props: SelectSeparatorProps) -> Element {
    rsx! {
        div {
            "data-slot": "select-separator",
            role: "separator",
            aria_hidden: "true",
            ..props.attributes,
        }
    }
}
