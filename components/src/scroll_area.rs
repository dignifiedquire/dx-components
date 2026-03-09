//! Styled scroll area matching shadcn/ui.
//!
//! Wraps `dioxus_primitives::scroll_area` with shadcn Tailwind classes.

use dioxus::prelude::*;
use dioxus_core::AttributeValue::Text;
use dioxus_primitives::scroll_area as primitives;
pub use dioxus_primitives::scroll_area::{ScrollDirection, ScrollType};
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// ScrollArea
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaProps {
    #[props(default)]
    pub direction: ScrollDirection,

    #[props(default)]
    pub scroll_type: ScrollType,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    let class = tw_merge!("relative", props.class);

    let mut attrs = props.attributes;
    attrs.push(Attribute {
        name: "class",
        value: Text(class),
        namespace: None,
        volatile: false,
    });

    rsx! {
        primitives::ScrollArea {
            direction: props.direction,
            scroll_type: props.scroll_type,
            attributes: attrs,
            {props.children}
        }
    }
}
