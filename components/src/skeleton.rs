//! Styled skeleton matching shadcn/ui.
//!
//! Pure HTML + Tailwind component.
//! No primitive dependency — renders a native `<div>`.

use dioxus::prelude::*;
use tailwind_fuse::*;

/// The props for the styled [`Skeleton`] component.
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled Skeleton — matches shadcn exactly.
#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    let class = tw_merge!("animate-pulse rounded-md bg-accent", props.class);

    rsx! {
        div {
            "data-slot": "skeleton",
            class: class,
            ..props.attributes,
        }
    }
}
