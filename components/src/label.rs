//! Styled label matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::label` primitive with
//! Tailwind classes — matching the shadcn/ui label component 1:1.

use dioxus::prelude::*;
use dioxus_primitives::label as primitives;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Label (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`Label`] component.
#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    /// The `for` attribute linking to a form control ID.
    #[props(default)]
    pub html_for: Option<String>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the label element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the label.
    pub children: Element,
}

/// Styled Label — matches shadcn exactly.
#[component]
pub fn Label(props: LabelProps) -> Element {
    let class = tw_merge!(
        "flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50",
        props.class,
    );

    rsx! {
        primitives::Label {
            html_for: props.html_for,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
