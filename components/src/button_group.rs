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
    // Upstream ships a separate `ui-rtl/button-group.tsx` that swaps the
    // physical `-l-`/`-r-` rules for logical `-s-`/`-e-` ones. Logical
    // properties render identically in LTR and correctly in RTL, so we use
    // them here to unify the single component (matches `ui-rtl`; LTR output
    // is identical to the default `ui/button-group.tsx`).
    let orientation_class = match props.orientation {
        Orientation::Horizontal => {
            "[&>*:not(:first-child)]:rounded-s-none [&>*:not(:first-child)]:border-s-0 [&>*:not(:last-child)]:rounded-e-none [&>[data-slot]:not(:has(~[data-slot]))]:rounded-e-lg!"
        }
        Orientation::Vertical => {
            "flex-col [&>*:not(:first-child)]:rounded-t-none [&>*:not(:first-child)]:border-t-0 [&>*:not(:last-child)]:rounded-b-none [&>[data-slot]:not(:has(~[data-slot]))]:rounded-b-lg!"
        }
    };

    let data_orientation = match props.orientation {
        Orientation::Horizontal => "horizontal",
        Orientation::Vertical => "vertical",
    };

    let class = tw_merge!(
        "flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 has-[>[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&>[data-slot=select-trigger]:last-of-type]:rounded-e-lg [&>[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&>input]:flex-1",
        orientation_class,
        props.class,
    );

    rsx! {
        div {
            "data-slot": "button-group",
            role: "group",
            "data-orientation": data_orientation,
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
        "flex items-center gap-2 rounded-lg border bg-muted px-2.5 text-sm font-medium [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4",
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
    /// Separator orientation. Upstream defaults to vertical.
    #[props(default = Orientation::Vertical)]
    pub orientation: Orientation,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Mirrors shadcn's `ButtonGroupSeparator`. Upstream composes the styled
/// `Separator`; the net rendered element is a decorative divider, so we
/// emit it directly with the merged Separator + button-group classes.
/// Upstream uses `data-horizontal:`/`data-vertical:` variants; our
/// convention is `data-[orientation=*]:`.
#[component]
pub fn ButtonGroupSeparator(props: ButtonGroupSeparatorProps) -> Element {
    let data_orientation = match props.orientation {
        Orientation::Horizontal => "horizontal",
        Orientation::Vertical => "vertical",
    };

    let class = tw_merge!(
        "shrink-0 relative self-stretch bg-input data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-auto data-[orientation=horizontal]:mx-px data-[orientation=vertical]:w-px data-[orientation=vertical]:h-auto data-[orientation=vertical]:my-px",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "button-group-separator",
            role: "none",
            "data-orientation": data_orientation,
            class: class,
            ..props.attributes,
        }
    }
}
