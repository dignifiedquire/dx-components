//! Styled button group matching shadcn/ui.
//!
//! Pure HTML + Tailwind — no Radix primitive needed.

use dioxus::prelude::*;
pub use dioxus_primitives::direction::Orientation;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// ButtonGroup
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupProps {
    /// Orientation of the button group. Defaults to horizontal.
    #[props(default = Orientation::Horizontal)]
    pub orientation: Orientation,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let orientation_class = match props.orientation {
        Orientation::Horizontal => {
            "[&>*:not(:first-child)]:rounded-l-none [&>*:not(:first-child)]:border-l-0 [&>*:not(:last-child)]:rounded-r-none"
        }
        Orientation::Vertical => {
            "flex-col [&>*:not(:first-child)]:rounded-t-none [&>*:not(:first-child)]:border-t-0 [&>*:not(:last-child)]:rounded-b-none"
        }
    };

    let class = tw_merge!(
        "flex w-fit items-stretch has-[>[data-slot=button-group]]:gap-2 [&>*]:focus-visible:relative [&>*]:focus-visible:z-10 [&>input]:flex-1",
        orientation_class,
        props.class,
    );

    rsx! {
        div {
            "data-slot": "button-group",
            role: "group",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ButtonGroupText
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupTextProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ButtonGroupText(props: ButtonGroupTextProps) -> Element {
    let class = tw_merge!(
        "flex items-center gap-2 rounded-md border bg-muted px-4 text-sm font-medium shadow-xs [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "button-group-text",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ButtonGroupSeparator
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupSeparatorProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ButtonGroupSeparator(props: ButtonGroupSeparatorProps) -> Element {
    let class = tw_merge!(
        "relative m-0 self-stretch bg-input shrink-0 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-auto data-[orientation=vertical]:w-px",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "button-group-separator",
            role: "none",
            "data-orientation": "vertical",
            class: class,
            ..props.attributes,
        }
    }
}
