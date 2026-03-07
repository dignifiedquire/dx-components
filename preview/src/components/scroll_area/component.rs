use dioxus::prelude::*;
use dioxus_primitives::scroll_area as scroll_area;

pub use dioxus_primitives::scroll_area::{ScrollAreaProps, ScrollDirection, ScrollType};

/// Styled ScrollArea with position:relative applied by default.
#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    rsx! {
        scroll_area::ScrollArea {
            class: "relative",
            direction: props.direction,
            scroll_type: props.scroll_type,
            attributes: props.attributes,
            {props.children}
        }
    }
}
