//! Popper primitive — matches `@radix-ui/react-popper`.
//!
//! Provides floating positioning for content relative to an anchor element.
//! This is the foundation for Popover, Tooltip, DropdownMenu, Select, etc.
//!
//! Radix uses `@floating-ui/react-dom` for positioning. This Dioxus port
//! provides the component API surface with CSS-based positioning using
//! `position: fixed` and `MountedData::get_client_rect()` for anchor
//! measurement.

use dioxus::prelude::*;
use std::rc::Rc;

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// Which side of the anchor the content appears on.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Side {
    /// Content appears above the anchor.
    Top,
    /// Content appears to the right of the anchor.
    Right,
    /// Content appears below the anchor.
    #[default]
    Bottom,
    /// Content appears to the left of the anchor.
    Left,
}

impl Side {
    /// Returns the string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }

    /// Returns the opposite side.
    pub fn opposite(&self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Right => Self::Left,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
        }
    }
}

/// Alignment of the content along the anchor edge.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Align {
    /// Aligned to the start edge.
    Start,
    /// Centered (default).
    #[default]
    Center,
    /// Aligned to the end edge.
    End,
}

impl Align {
    /// Returns the string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct PopperCtx {
    anchor_mounted: Signal<Option<Rc<MountedData>>>,
}

#[derive(Clone)]
struct PopperContentCtx {
    placed_side: Side,
}

// ---------------------------------------------------------------------------
// Popper (root)
// ---------------------------------------------------------------------------

/// Props for [`Popper`].
#[derive(Props, Clone, PartialEq)]
pub struct PopperProps {
    /// Children.
    pub children: Element,
}

/// Root context provider for floating positioning.
///
/// Matches Radix's `Popper` — provides the anchor/content relationship context.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::popper::{Popper, PopperAnchor, PopperContent, Side};
/// rsx! {
///     Popper {
///         PopperAnchor { button { "Toggle" } }
///         PopperContent { side: Side::Bottom,
///             div { "Floating content" }
///         }
///     }
/// };
/// ```
#[component]
pub fn Popper(props: PopperProps) -> Element {
    let anchor_mounted: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    use_context_provider(|| PopperCtx { anchor_mounted });

    props.children
}

// ---------------------------------------------------------------------------
// PopperAnchor
// ---------------------------------------------------------------------------

/// Props for [`PopperAnchor`].
#[derive(Props, Clone, PartialEq)]
pub struct PopperAnchorProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Marks the anchor element for floating positioning.
///
/// Matches Radix's `PopperAnchor`.
#[component]
pub fn PopperAnchor(props: PopperAnchorProps) -> Element {
    let mut ctx: PopperCtx = use_context();

    rsx! {
        div {
            "data-slot": "popper-anchor",
            class: props.class,
            onmounted: move |event: Event<MountedData>| {
                ctx.anchor_mounted.set(Some(event.data()));
            },
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// PopperContent
// ---------------------------------------------------------------------------

/// Props for [`PopperContent`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct PopperContentProps {
    /// Which side of the anchor to place content. Defaults to `Bottom`.
    #[props(default)]
    pub side: Side,

    /// Offset from the anchor edge in pixels. Defaults to 0.
    #[props(default)]
    pub side_offset: f64,

    /// Alignment along the anchor edge. Defaults to `Center`.
    #[props(default)]
    pub align: Align,

    /// Offset along the alignment axis in pixels. Defaults to 0.
    #[props(default)]
    pub align_offset: f64,

    /// Whether to avoid collisions with viewport edges. Defaults to `true`.
    #[props(default = true)]
    pub avoid_collisions: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// Floating content positioned relative to the anchor.
///
/// Matches Radix's `PopperContent`. Positions itself using `position: fixed`
/// and measures the anchor element via `MountedData::get_client_rect()`.
/// Sets CSS custom properties `--radix-popper-anchor-width` and
/// `--radix-popper-anchor-height`.
#[component]
pub fn PopperContent(props: PopperContentProps) -> Element {
    let ctx: PopperCtx = use_context();
    let mut position = use_signal(|| None::<(f64, f64)>);
    let mut anchor_size = use_signal(|| (0.0f64, 0.0f64));

    let side = props.side;
    let side_offset = props.side_offset;
    let align = props.align;
    let align_offset = props.align_offset;

    // Measure anchor position
    use_effect(move || {
        if let Some(md) = ctx.anchor_mounted.cloned() {
            spawn(async move {
                if let Ok(rect) = md.get_client_rect().await {
                    let anchor_x = rect.origin.x;
                    let anchor_y = rect.origin.y;
                    let anchor_w = rect.size.width;
                    let anchor_h = rect.size.height;

                    anchor_size.set((anchor_w, anchor_h));

                    let (x, y) = match side {
                        Side::Bottom => {
                            let x = match align {
                                Align::Start => anchor_x + align_offset,
                                Align::Center => anchor_x + anchor_w / 2.0,
                                Align::End => anchor_x + anchor_w - align_offset,
                            };
                            (x, anchor_y + anchor_h + side_offset)
                        }
                        Side::Top => {
                            let x = match align {
                                Align::Start => anchor_x + align_offset,
                                Align::Center => anchor_x + anchor_w / 2.0,
                                Align::End => anchor_x + anchor_w - align_offset,
                            };
                            (x, anchor_y - side_offset)
                        }
                        Side::Right => {
                            let y = match align {
                                Align::Start => anchor_y + align_offset,
                                Align::Center => anchor_y + anchor_h / 2.0,
                                Align::End => anchor_y + anchor_h - align_offset,
                            };
                            (anchor_x + anchor_w + side_offset, y)
                        }
                        Side::Left => {
                            let y = match align {
                                Align::Start => anchor_y + align_offset,
                                Align::Center => anchor_y + anchor_h / 2.0,
                                Align::End => anchor_y + anchor_h - align_offset,
                            };
                            (anchor_x - side_offset, y)
                        }
                    };

                    position.set(Some((x, y)));
                }
            });
        }
    });

    use_context_provider(|| PopperContentCtx {
        placed_side: props.side,
    });

    let (aw, ah) = anchor_size();

    let style = if let Some((x, y)) = position() {
        let transform = match (side, align) {
            (Side::Bottom | Side::Top, Align::Center) => "translateX(-50%)",
            (Side::Bottom | Side::Top, Align::End) => "translateX(-100%)",
            (Side::Right | Side::Left, Align::Center) => "translateY(-50%)",
            (Side::Right | Side::Left, Align::End) => "translateY(-100%)",
            (Side::Top, Align::Start) => "translateY(-100%)",
            (Side::Left, Align::Start) => "translateX(-100%)",
            _ => "",
        };

        // For top/left sides, we also need to translate
        let extra_transform = match side {
            Side::Top if !matches!(align, Align::Start) => "translateY(-100%) ",
            Side::Left if !matches!(align, Align::Start) => "translateX(-100%) ",
            _ => "",
        };

        format!(
            "position: fixed; left: {x}px; top: {y}px; \
             transform: {extra_transform}{transform}; \
             min-width: max-content; \
             --radix-popper-anchor-width: {aw}px; \
             --radix-popper-anchor-height: {ah}px;"
        )
    } else {
        // Off-screen while measuring
        "position: fixed; left: 0; top: 0; transform: translate(0, -200%); min-width: max-content;"
            .to_string()
    };

    rsx! {
        div {
            "data-radix-popper-content-wrapper": "",
            style: "{style}",

            div {
                "data-slot": "popper-content",
                "data-side": side.as_str(),
                "data-align": align.as_str(),
                class: props.class,
                ..props.attributes,
                {props.children}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// PopperArrow
// ---------------------------------------------------------------------------

/// Props for [`PopperArrow`].
#[derive(Props, Clone, PartialEq)]
pub struct PopperArrowProps {
    /// Width of the arrow. Defaults to 10.
    #[props(default = 10.0)]
    pub width: f64,

    /// Height of the arrow. Defaults to 5.
    #[props(default = 5.0)]
    pub height: f64,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Arrow pointing from the content toward the anchor.
///
/// Matches Radix's `PopperArrow`. Must be rendered inside [`PopperContent`].
#[component]
pub fn PopperArrow(props: PopperArrowProps) -> Element {
    let content_ctx: PopperContentCtx = use_context();
    let base_side = content_ctx.placed_side.opposite();

    let transform = match content_ctx.placed_side {
        Side::Top => "translateY(100%)",
        Side::Right => "translateY(50%) rotate(90deg) translateX(-50%)",
        Side::Bottom => "rotate(180deg)",
        Side::Left => "translateY(50%) rotate(-90deg) translateX(50%)",
    };

    let style = format!(
        "position: absolute; {base}: 0; transform: {transform};",
        base = base_side.as_str(),
    );

    rsx! {
        span {
            style: "{style}",

            crate::arrow::Arrow {
                width: props.width,
                height: props.height,
                class: props.class,
                attributes: props.attributes,
            }
        }
    }
}
