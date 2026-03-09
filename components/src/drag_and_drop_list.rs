//! Styled drag-and-drop list.
//!
//! Wraps the unstyled `dioxus_primitives::drag_and_drop_list` primitive and
//! includes a CSS stylesheet that targets `data-slot` attributes.

use dioxus::prelude::*;
use dioxus_primitives::drag_and_drop_list as primitives;

/// The props for the styled [`DragAndDropList`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DragAndDropListProps {
    /// Items (labels) to be rendered.
    pub items: Vec<Element>,

    /// Set if the list items should be removable.
    #[props(default)]
    pub is_removable: bool,

    /// Accessible label for the list.
    #[props(default)]
    pub aria_label: Option<String>,

    /// Attributes to extend the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the list component.
    pub children: Element,
}

/// Styled DragAndDropList — includes CSS for data-slot styling.
#[component]
pub fn DragAndDropList(props: DragAndDropListProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./drag_and_drop_list.css") }
        primitives::DragAndDropList {
            items: props.items,
            is_removable: props.is_removable,
            aria_label: props.aria_label,
            attributes: props.attributes,
            {props.children}
        }
    }
}
