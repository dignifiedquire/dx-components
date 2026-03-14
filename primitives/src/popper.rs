//! Popper primitive — matches `@radix-ui/react-popper`.
//!
//! Provides floating positioning for content relative to an anchor element.
//! This is the foundation for Popover, Tooltip, DropdownMenu, Select, etc.
//!
//! Uses the `floating-ui` crate for all positioning math and DOM measurement.
//! Zero JavaScript strings.

use std::rc::Rc;

use dioxus::prelude::*;

use floating_ui::utils::{get_alignment, get_side};

// ---------------------------------------------------------------------------
// Enums (re-export from floating-ui with Dioxus-friendly API)
// ---------------------------------------------------------------------------

/// Which side of the anchor the content appears on.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Side {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

impl Side {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }

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
    Start,
    #[default]
    Center,
    End,
}

impl Align {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

// Conversions between popper's Side/Align and floating-ui's Placement
fn to_floating_placement(side: Side, align: Align) -> floating_ui::Placement {
    use floating_ui::Placement;
    match (side, align) {
        (Side::Top, Align::Center) => Placement::Top,
        (Side::Top, Align::Start) => Placement::TopStart,
        (Side::Top, Align::End) => Placement::TopEnd,
        (Side::Right, Align::Center) => Placement::Right,
        (Side::Right, Align::Start) => Placement::RightStart,
        (Side::Right, Align::End) => Placement::RightEnd,
        (Side::Bottom, Align::Center) => Placement::Bottom,
        (Side::Bottom, Align::Start) => Placement::BottomStart,
        (Side::Bottom, Align::End) => Placement::BottomEnd,
        (Side::Left, Align::Center) => Placement::Left,
        (Side::Left, Align::Start) => Placement::LeftStart,
        (Side::Left, Align::End) => Placement::LeftEnd,
    }
}

fn from_floating_placement(placement: floating_ui::Placement) -> (Side, Align) {
    let side = match get_side(placement) {
        floating_ui::types::Side::Top => Side::Top,
        floating_ui::types::Side::Right => Side::Right,
        floating_ui::types::Side::Bottom => Side::Bottom,
        floating_ui::types::Side::Left => Side::Left,
    };
    let align = match get_alignment(placement) {
        None => Align::Center,
        Some(floating_ui::types::Alignment::Start) => Align::Start,
        Some(floating_ui::types::Alignment::End) => Align::End,
    };
    (side, align)
}

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

/// How the anchor is specified.
#[derive(Clone, Copy)]
pub(crate) enum PopperAnchorKind {
    Element(Signal<Option<Rc<MountedData>>>),
    Virtual { x: Signal<f64>, y: Signal<f64> },
}

/// Root context shared between anchor and content.
#[derive(Clone, Copy)]
pub(crate) struct PopperCtx {
    pub anchor: PopperAnchorKind,
}

impl PopperCtx {
    pub fn set_anchor_ref(&self, data: Rc<MountedData>) {
        if let PopperAnchorKind::Element(mut sig) = self.anchor {
            sig.set(Some(data));
        }
    }
}

/// Context provided by [`PopperContent`] to children.
#[derive(Clone, Copy)]
pub(crate) struct PopperContentCtx {
    pub placed_side: ReadSignal<Side>,
    pub placed_align: ReadSignal<Align>,
}

// ---------------------------------------------------------------------------
// Popper (root)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct PopperProps {
    pub children: Element,
}

#[component]
pub fn Popper(props: PopperProps) -> Element {
    let anchor_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_context_provider(|| PopperCtx {
        anchor: PopperAnchorKind::Element(anchor_ref),
    });
    props.children
}

// ---------------------------------------------------------------------------
// PopperAnchor
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct PopperAnchorProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn PopperAnchor(props: PopperAnchorProps) -> Element {
    let ctx: PopperCtx = use_context();

    let onmounted = move |evt: Event<MountedData>| {
        if let PopperAnchorKind::Element(mut sig) = ctx.anchor {
            sig.set(Some(evt.data()));
        }
    };

    rsx! {
        div {
            onmounted: onmounted,
            "data-slot": "popper-anchor",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// PopperContent
// ---------------------------------------------------------------------------

#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct PopperContentProps {
    #[props(default)]
    pub side: Side,

    #[props(default)]
    pub side_offset: f64,

    #[props(default)]
    pub align: Align,

    #[props(default)]
    pub align_offset: f64,

    #[props(default = true)]
    pub avoid_collisions: bool,

    #[props(default)]
    pub collision_padding: f64,

    #[props(default)]
    pub css_var_prefix: Option<&'static str>,

    /// Whether to render through a Portal (escapes parent overflow).
    /// Default: false. Set to true for components like DropdownMenu, ContextMenu.
    #[props(default)]
    pub portal: bool,

    pub children: Element,
}

/// Floating content positioned relative to the anchor.
///
/// Uses `floating-ui` crate for positioning (offset, flip, shift, size middleware).
/// Auto-updates on scroll/resize via web-sys event listeners.
/// Sets CSS custom properties for transform-origin and available space.
#[component]
pub fn PopperContent(props: PopperContentProps) -> Element {
    let ctx: PopperCtx = use_context();

    let mut content_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    // Position state
    let mut pos_x = use_signal(|| None::<f64>);
    let mut pos_y = use_signal(|| None::<f64>);
    let mut placed_side = use_signal(|| props.side);
    let mut placed_align = use_signal(|| props.align);
    let mut avail_w = use_signal(|| 0.0f64);
    let mut avail_h = use_signal(|| 0.0f64);
    let mut anchor_w = use_signal(|| 0.0f64);
    let mut anchor_h = use_signal(|| 0.0f64);
    let mut transform_origin = use_signal(String::new);

    // Context is provided by PopperContentCtxProvider inside the wrapper
    // (works correctly even through Portal since it re-provides at render site)

    let side = props.side;
    let side_offset = props.side_offset;
    let align = props.align;
    let align_offset = props.align_offset;
    let avoid_collisions = props.avoid_collisions;
    let collision_padding = props.collision_padding;
    let anchor = ctx.anchor;

    // Tick counter — incremented by auto_update scroll/resize listeners to trigger re-measurement
    let tick = use_signal(|| 0u64);

    // Positioning effect: fires on tick changes (scroll/resize) and ref changes.
    // Subscribes to content_ref and anchor signals so it re-runs when elements mount.
    use_effect(move || {
        let _tick = tick(); // Subscribe to scroll/resize updates
        let Some(content_md) = content_ref.cloned() else {
            return;
        };

        // Subscribe to anchor signal changes so we reposition when anchor mounts
        let anchor_kind = anchor;
        match anchor_kind {
            PopperAnchorKind::Element(sig) => { let _ = sig(); }  // Subscribe
            PopperAnchorKind::Virtual { x, y } => { let _ = (x(), y()); }  // Subscribe
        }

        spawn(async move {
            // Wait for browser layout to complete before measuring.
            // Without this, getBoundingClientRect returns stale coordinates
            // because the browser hasn't performed layout yet.
            #[cfg(target_arch = "wasm32")]
            {
                let promise = js_sys::Promise::new(&mut |resolve, _| {
                    let _ = web_sys::window()
                        .unwrap()
                        .request_animation_frame(&resolve);
                });
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
            }

            // Measure anchor (viewport-relative via getBoundingClientRect)
            let anchor_rect = match anchor_kind {
                PopperAnchorKind::Element(sig) => {
                    let Some(md) = sig.cloned() else { return };
                    let Ok(r) = md.get_client_rect().await else { return };
                    floating_ui::Rect {
                        x: r.origin.x,
                        y: r.origin.y,
                        width: r.size.width,
                        height: r.size.height,
                    }
                }
                PopperAnchorKind::Virtual { x, y } => floating_ui::Rect {
                    x: x(),
                    y: y(),
                    width: 0.0,
                    height: 0.0,
                },
            };

            // Measure content
            let Ok(cr) = content_md.get_client_rect().await else { return };
            let content_rect = floating_ui::Rect {
                x: 0.0,
                y: 0.0,
                width: cr.size.width,
                height: cr.size.height,
            };

            let rects = floating_ui::ElementRects {
                reference: anchor_rect,
                floating: content_rect,
            };

            // Build middleware chain matching Radix Popper.tsx
            let placement = to_floating_placement(side, align);
            let padding = floating_ui::Padding::Uniform(collision_padding);

            let mut middleware = vec![floating_ui::Middleware::Offset(
                floating_ui::OffsetOptions {
                    main_axis: side_offset,
                    alignment_axis: if align_offset != 0.0 {
                        Some(align_offset)
                    } else {
                        None
                    },
                    ..Default::default()
                },
            )];

            if avoid_collisions {
                middleware.push(floating_ui::Middleware::Shift(floating_ui::ShiftOptions {
                    main_axis: true,
                    cross_axis: false,
                    limiter: Some(floating_ui::LimitShift::default()),
                    detect_overflow: floating_ui::DetectOverflowOptions {
                        padding,
                        ..Default::default()
                    },
                }));
                middleware.push(floating_ui::Middleware::Flip(floating_ui::FlipOptions {
                    detect_overflow: floating_ui::DetectOverflowOptions {
                        padding,
                        ..Default::default()
                    },
                    ..Default::default()
                }));
            }

            middleware.push(floating_ui::Middleware::Size(floating_ui::SizeOptions {
                detect_overflow: floating_ui::DetectOverflowOptions {
                    padding,
                    ..Default::default()
                },
            }));

            // Simple viewport-based overflow detection
            // On wasm32, we'd use floating_ui::dom for full clipping rect.
            // For now, use a viewport rect from content measurement.
            let detect_overflow_fn =
                |state: &floating_ui::MiddlewareState,
                 opts: &floating_ui::DetectOverflowOptions| {
                    // Viewport = the window dimensions.
                    // Since we use strategy: fixed, getBoundingClientRect coords are viewport-relative.
                    // We get viewport size from the content's measurement context.
                    #[cfg(target_arch = "wasm32")]
                    let viewport = {
                        let vp = floating_ui::dom::get_viewport_rect();
                        floating_ui::utils::rect_to_client_rect(vp)
                    };
                    #[cfg(not(target_arch = "wasm32"))]
                    let viewport = floating_ui::utils::rect_to_client_rect(floating_ui::Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 1024.0,
                        height: 768.0,
                    });

                    floating_ui::core::detect_overflow::detect_overflow(state, viewport, opts)
                };

            let result = floating_ui::compute_position(
                rects,
                placement,
                floating_ui::Strategy::Fixed,
                &middleware,
                &detect_overflow_fn,
            );

            let (result_side, result_align) = from_floating_placement(result.placement);

            // Compute transform origin (matching Radix's custom transformOrigin middleware)
            let align_pct = match result_align {
                Align::Start => "0%",
                Align::Center => "50%",
                Align::End => "100%",
            };
            let to = match result_side {
                Side::Bottom => format!("{align_pct} 0px"),
                Side::Top => format!("{align_pct} {}px", content_rect.height),
                Side::Right => format!("0px {align_pct}"),
                Side::Left => format!("{}px {align_pct}", content_rect.width),
            };

            let (aw, ah) = result
                .middleware_data
                .size
                .map(|s| (s.available_width, s.available_height))
                .unwrap_or((0.0, 0.0));

            // Update signals
            pos_x.set(Some(result.x));
            pos_y.set(Some(result.y));
            placed_side.set(result_side);
            placed_align.set(result_align);
            avail_w.set(aw);
            avail_h.set(ah);
            anchor_w.set(anchor_rect.width);
            anchor_h.set(anchor_rect.height);
            transform_origin.set(to);
        });
    });

    // Auto-update: set up scroll/resize listeners that bump `tick`
    // This is only available on wasm32 (uses web-sys)
    #[cfg(target_arch = "wasm32")]
    {
        use crate::use_effect_with_cleanup;
        use_effect_with_cleanup(move || {
            let doc_el = floating_ui::dom::get_document_element();
            let cleanup = floating_ui::dom::auto_update(
                &doc_el,
                &doc_el,
                {
                    let mut tick = tick;
                    move || {
                        tick += 1;
                    }
                },
                floating_ui::dom::AutoUpdateOptions {
                    ancestor_scroll: true,
                    ancestor_resize: true,
                    element_resize: false,
                    layout_shift: false,
                    animation_frame: false,
                },
            );

            move || {
                cleanup();
            }
        });
    }

    let prefix = props.css_var_prefix;

    let style = if let (Some(x), Some(y)) = (pos_x(), pos_y()) {
        let (to_var, aw_var, ah_var, anw_var, anh_var) = if let Some(p) = prefix {
            (
                format!("--radix-{p}-content-transform-origin"),
                format!("--radix-{p}-content-available-width"),
                format!("--radix-{p}-content-available-height"),
                format!("--radix-{p}-trigger-width"),
                format!("--radix-{p}-trigger-height"),
            )
        } else {
            (
                "--radix-popper-transform-origin".to_string(),
                "--radix-popper-available-width".to_string(),
                "--radix-popper-available-height".to_string(),
                "--radix-popper-anchor-width".to_string(),
                "--radix-popper-anchor-height".to_string(),
            )
        };

        format!(
            "position: fixed; left: {x}px; top: {y}px; \
             transform: none; \
             min-width: max-content; \
             {to_var}: {to}; \
             {aw_var}: {aw}px; \
             {ah_var}: {ah}px; \
             {anw_var}: {anw}px; \
             {anh_var}: {anh}px;",
            to = transform_origin(),
            aw = avail_w(),
            ah = avail_h(),
            anw = anchor_w(),
            anh = anchor_h(),
        )
    } else {
        // Off-screen while measuring
        "position: fixed; left: 0; top: 0; transform: translate(0, -200%); min-width: max-content;"
            .to_string()
    };

    // Capture context values as signals to re-provide through Portal
    let ctx_side: ReadSignal<Side> = placed_side.into();
    let ctx_align: ReadSignal<Align> = placed_align.into();

    let wrapper = rsx! {
        PopperContentCtxProvider {
            placed_side: ctx_side,
            placed_align: ctx_align,
            div {
                onmounted: move |evt| content_ref.set(Some(evt.data())),
                "data-radix-popper-content-wrapper": "",
                style: "{style}",
                {props.children}
            }
        }
    };

    if props.portal {
        rsx! {
            crate::portal::Portal { {wrapper} }
        }
    } else {
        wrapper
    }
}

/// Re-provides PopperContentCtx for children that may be inside a Portal.
#[derive(Props, Clone, PartialEq)]
struct PopperContentCtxProviderProps {
    placed_side: ReadSignal<Side>,
    placed_align: ReadSignal<Align>,
    children: Element,
}

#[component]
fn PopperContentCtxProvider(props: PopperContentCtxProviderProps) -> Element {
    use_context_provider(|| PopperContentCtx {
        placed_side: props.placed_side,
        placed_align: props.placed_align,
    });
    props.children
}

// ---------------------------------------------------------------------------
// PopperArrow
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct PopperArrowProps {
    #[props(default = 10.0)]
    pub width: f64,

    #[props(default = 5.0)]
    pub height: f64,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn PopperArrow(props: PopperArrowProps) -> Element {
    let content_ctx: PopperContentCtx = use_context();
    let side = (content_ctx.placed_side)();
    let base_side = side.opposite();

    let transform = match side {
        Side::Top => "translateY(100%)",
        Side::Right => "translateY(50%) rotate(90deg) translateX(-50%)",
        Side::Bottom => "rotate(180deg)",
        Side::Left => "translateY(50%) rotate(-90deg) translateX(50%)",
    };

    let style = format!(
        "position: absolute; {base}: 0; left: 50%; transform: translateX(-50%) {transform};",
        base = base_side.as_str(),
    );

    rsx! {
        span {
            "data-slot": "popper-arrow",
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
