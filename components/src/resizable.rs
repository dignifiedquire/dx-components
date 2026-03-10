//! Styled Resizable component — matches shadcn/ui `resizable.tsx`.
//!
//! Wraps `dioxus_primitives::resizable` with shadcn Tailwind classes.

use dioxus::prelude::*;
use dioxus_primitives::resizable as primitives;
use tailwind_fuse::*;

// Re-export context and types
pub use primitives::{ResizablePanelGroupCtx, use_resizable_panel_group};

// ---------------------------------------------------------------------------
// ResizablePanelGroup
// ---------------------------------------------------------------------------

/// Props for the styled [`ResizablePanelGroup`].
#[derive(Props, Clone, PartialEq)]
pub struct ResizablePanelGroupProps {
    /// Layout direction. Defaults to `Horizontal`.
    #[props(default = dioxus_primitives::direction::Orientation::Horizontal)]
    pub orientation: dioxus_primitives::direction::Orientation,

    /// Called whenever the layout changes.
    #[props(default)]
    pub on_layout_change: Option<Callback<Vec<f64>>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled panel group — `flex h-full w-full` with vertical flex-col.
#[component]
pub fn ResizablePanelGroup(props: ResizablePanelGroupProps) -> Element {
    let orient_class = match props.orientation {
        dioxus_primitives::direction::Orientation::Vertical => "flex-col",
        _ => "",
    };

    let class = tw_merge!("flex h-full w-full", orient_class, props.class);

    rsx! {
        primitives::ResizablePanelGroup {
            orientation: props.orientation,
            on_layout_change: props.on_layout_change,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ResizablePanel
// ---------------------------------------------------------------------------

/// Props for the styled [`ResizablePanel`].
#[derive(Props, Clone, PartialEq)]
pub struct ResizablePanelProps {
    /// Default size as a percentage (0–100).
    #[props(default = 50.0)]
    pub default_size: f64,

    /// Minimum size as a percentage.
    #[props(default = 0.0)]
    pub min_size: f64,

    /// Maximum size as a percentage.
    #[props(default = 100.0)]
    pub max_size: f64,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled resizable panel — thin passthrough (shadcn adds no extra classes).
#[component]
pub fn ResizablePanel(props: ResizablePanelProps) -> Element {
    rsx! {
        primitives::ResizablePanel {
            default_size: props.default_size,
            min_size: props.min_size,
            max_size: props.max_size,
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ResizableHandle
// ---------------------------------------------------------------------------

/// Props for the styled [`ResizableHandle`].
#[derive(Props, Clone, PartialEq)]
pub struct ResizableHandleProps {
    /// Whether the handle is disabled.
    #[props(default)]
    pub disabled: bool,

    /// When true, renders a visible grip icon in the handle.
    #[props(default)]
    pub with_handle: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled resize handle — matches shadcn's ResizableHandle.
///
/// Renders a 1px separator line with optional grip indicator.
#[component]
pub fn ResizableHandle(props: ResizableHandleProps) -> Element {
    // Read orientation from the parent panel group context.
    let ctx = use_context::<ResizablePanelGroupCtx>();
    let is_vertical_group = matches!(
        ctx.orientation,
        dioxus_primitives::direction::Orientation::Vertical
    );

    // Handle between vertical panels = horizontal separator line;
    // handle between horizontal panels = vertical separator line.
    let (orientation_classes, after_classes, child_rotate, cursor_class) = if is_vertical_group {
        (
            "h-px w-full",
            "after:left-0 after:h-1 after:w-full after:translate-x-0 after:-translate-y-1/2",
            "[&>div]:rotate-90",
            "cursor-row-resize",
        )
    } else {
        (
            "w-px",
            "after:inset-y-0 after:left-1/2 after:w-1 after:-translate-x-1/2",
            "",
            "cursor-col-resize",
        )
    };

    let class = tw_merge!(
        "relative flex shrink-0 items-center justify-center bg-border",
        orientation_classes,
        cursor_class,
        "after:absolute",
        after_classes,
        "focus-visible:ring-1 focus-visible:ring-ring focus-visible:ring-offset-1 focus-visible:outline-hidden",
        child_rotate,
        props.class,
    );

    let has_children = props.children != Ok(VNode::placeholder());

    rsx! {
        primitives::ResizableHandle {
            disabled: props.disabled,
            class: class,
            attributes: props.attributes,

            if props.with_handle && !has_children {
                div { class: "z-10 flex h-4 w-3 items-center justify-center rounded-xs border bg-border",
                    // Grip dots icon (simplified SVG matching GripVerticalIcon)
                    svg {
                        class: "size-2.5",
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        circle { cx: "9", cy: "12", r: "1" }
                        circle { cx: "9", cy: "5", r: "1" }
                        circle { cx: "9", cy: "19", r: "1" }
                        circle { cx: "15", cy: "12", r: "1" }
                        circle { cx: "15", cy: "5", r: "1" }
                        circle { cx: "15", cy: "19", r: "1" }
                    }
                }
            }

            {props.children}
        }
    }
}
