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
    /// Content above the anchor.
    Top,
    /// Content to the right of the anchor.
    Right,
    /// Content below the anchor (default).
    #[default]
    Bottom,
    /// Content to the left of the anchor.
    Left,
}

impl Side {
    /// Returns the CSS string representation.
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
    /// Align to the start edge.
    Start,
    /// Align to the center (default).
    #[default]
    Center,
    /// Align to the end edge.
    End,
}

impl Align {
    /// Returns the CSS string representation.
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
#[allow(dead_code)]
pub(crate) struct PopperContentCtx {
    pub placed_side: ReadSignal<Side>,
    pub placed_align: ReadSignal<Align>,
    /// Whether floating-ui has computed position. Matches Radix `isPositioned`.
    /// Children use this to suppress entry animations until placement is done.
    pub is_positioned: ReadSignal<bool>,
    /// Arrow x offset from arrow middleware.
    pub arrow_x: ReadSignal<Option<f64>>,
    /// Arrow y offset from arrow middleware.
    pub arrow_y: ReadSignal<Option<f64>>,
    /// Whether the arrow should be hidden (centerOffset !== 0).
    pub should_hide_arrow: ReadSignal<bool>,
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

/// Root context provider for popper positioning.
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

/// Props for [`PopperAnchor`].
#[derive(Props, Clone, PartialEq)]
pub struct PopperAnchorProps {
    /// CSS class.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// The anchor element that the floating content positions relative to.
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

/// Unique ID counter for wrapper elements (used to read computed styles via web-sys).
static POPPER_ID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

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

    /// CSS class forwarded to the inner content div (matching upstream Primitive.div).
    /// This is where consumers set their styling (e.g. `bg-popover z-50 ...`).
    #[props(default)]
    pub class: Option<String>,

    /// Attributes merged onto the inner content div (matching upstream `{...contentProps}`).
    /// Consumers build these with `attributes!` + `merge_attributes` to pass
    /// `data-state`, `role`, `id`, etc.
    #[props(default)]
    pub content_attributes: Vec<Attribute>,

    /// Called when an animation ends on the inner content div.
    /// Used by consumers for presence animation tracking.
    #[props(default)]
    pub on_animation_end: Option<EventHandler<Event<AnimationData>>>,

    /// Called when pointer enters the inner content div.
    #[props(default)]
    pub on_pointer_enter: Option<EventHandler<Event<PointerData>>>,

    /// Called when pointer leaves the inner content div.
    #[props(default)]
    pub on_pointer_leave: Option<EventHandler<Event<PointerData>>>,

    /// Called when a key is pressed on the inner content div.
    #[props(default)]
    pub on_keydown: Option<EventHandler<Event<KeyboardData>>>,

    /// Called when the inner content div loses focus.
    #[props(default)]
    pub on_blur: Option<EventHandler<Event<FocusData>>>,

    /// Called when the inner content div is mounted.
    #[props(default)]
    pub on_mounted: Option<EventHandler<Event<MountedData>>>,

    /// Width of the arrow element (along alignment axis).
    /// Pass the same value as `PopperArrow::width` when using an arrow.
    #[props(default)]
    pub arrow_width: f64,

    /// Height of the arrow element (perpendicular to alignment axis).
    /// Pass the same value as `PopperArrow::height` when using an arrow.
    #[props(default)]
    pub arrow_height: f64,

    /// Padding between arrow and floating element edges.
    /// Upstream: `arrowPadding` (default `0`).
    #[props(default)]
    pub arrow_padding: f64,

    /// Hide the content when the anchor is fully detached from viewport.
    /// Upstream: `hideWhenDetached` (default `false`).
    #[props(default)]
    pub hide_when_detached: bool,

    /// Called once when the content is first positioned.
    /// Upstream: `onPlaced` callback.
    #[props(default)]
    pub on_placed: Option<Callback<()>>,

    /// Position update strategy: `"optimized"` (default) or `"always"`.
    /// When `"always"`, uses `animation_frame: true` for continuous updates.
    #[props(default)]
    pub update_position_strategy: Option<&'static str>,

    pub children: Element,
}

/// Floating content positioned relative to the anchor.
///
/// Renders two elements matching upstream `@radix-ui/react-popper`:
/// 1. **Wrapper div** (`data-radix-popper-content-wrapper`): positioned by floating-ui
///    with `position: fixed`, floatingStyles, `zIndex` mirrored from content,
///    transform-origin CSS var, and available-size CSS vars.
/// 2. **Inner content div** (`data-side`, `data-align`): receives consumer's class,
///    attributes, and suppresses animations until positioned.
#[component]
pub fn PopperContent(props: PopperContentProps) -> Element {
    let ctx: PopperCtx = use_context();

    // Wrapper ref — the floating element measured for positioning
    let mut wrapper_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    // Unique ID for the wrapper so we can query it via web-sys for contentZIndex
    let wrapper_id = use_memo(|| {
        format!(
            "__popper_w_{}",
            POPPER_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        )
    });

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

    // Derived from position values — true once floating-ui computes position.
    // Used to suppress entry animations on inner content div (upstream line 281).
    // Derived rather than set separately to avoid signal batching race conditions
    // in async spawn (pos_x/pos_y set before is_positioned would cause a render
    // with position values but is_positioned=false).
    let is_positioned = use_memo(move || pos_x().is_some() && pos_y().is_some());

    // Mirrors content's computed z-index to wrapper (matching upstream lines 232-235, 245).
    #[allow(unused_mut)]
    let mut content_z_index = use_signal(|| Option::<String>::None);

    // Hide-when-detached state
    let mut reference_hidden = use_signal(|| false);

    // Arrow middleware state
    let mut arrow_x = use_signal(|| None::<f64>);
    let mut arrow_y = use_signal(|| None::<f64>);
    let mut should_hide_arrow = use_signal(|| false);

    let side = props.side;
    let side_offset = props.side_offset;
    let align = props.align;
    let align_offset = props.align_offset;
    let avoid_collisions = props.avoid_collisions;
    let collision_padding = props.collision_padding;
    let arrow_w = props.arrow_width;
    let arrow_h = props.arrow_height;
    let arrow_padding = props.arrow_padding;
    let has_arrow = arrow_w > 0.0 || arrow_h > 0.0;
    let hide_when_detached = props.hide_when_detached;
    let on_placed = props.on_placed;
    #[cfg(target_arch = "wasm32")]
    let animation_frame = props.update_position_strategy == Some("always");
    let anchor = ctx.anchor;

    // Tick counter — incremented by auto_update scroll/resize listeners to trigger re-measurement
    let tick = use_signal(|| 0u64);

    // Read content's computed z-index after inner div mounts (upstream lines 232-235).
    // Uses wrapper ID to find the wrapper, then firstElementChild for the inner content div.
    #[cfg(target_arch = "wasm32")]
    {
        let wrapper_id = wrapper_id.clone();
        use_effect(move || {
            // Subscribe to wrapper_ref so we run after mount
            let _ = wrapper_ref();
            let id = wrapper_id();
            let Some(window) = web_sys::window() else {
                return;
            };
            let Some(doc) = window.document() else {
                return;
            };
            if let Some(wrapper_el) = doc.get_element_by_id(&id) {
                if let Some(inner_el) = wrapper_el.first_element_child() {
                    if let Ok(Some(style)) = window.get_computed_style(&inner_el) {
                        if let Ok(z) = style.get_property_value("z-index") {
                            if z != "auto" && !z.is_empty() {
                                content_z_index.set(Some(z));
                            }
                        }
                    }
                }
            }
        });
    }

    // Positioning effect: fires on tick changes (scroll/resize) and ref changes.
    use_effect(move || {
        let _tick = tick(); // Subscribe to scroll/resize updates
        let Some(wrapper_md) = wrapper_ref.cloned() else {
            return;
        };

        // Subscribe to anchor signal changes so we reposition when anchor mounts
        let anchor_kind = anchor;
        match anchor_kind {
            PopperAnchorKind::Element(sig) => {
                let _ = sig();
            }
            PopperAnchorKind::Virtual { x, y } => {
                let _ = (x(), y());
            }
        }

        spawn(async move {
            // Wait for browser layout to complete before measuring.
            #[cfg(target_arch = "wasm32")]
            {
                let promise = js_sys::Promise::new(&mut |resolve, _| {
                    let _ = web_sys::window().unwrap().request_animation_frame(&resolve);
                });
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
            }

            // Measure anchor (viewport-relative via getBoundingClientRect)
            let anchor_rect = match anchor_kind {
                PopperAnchorKind::Element(sig) => {
                    let Some(md) = sig.cloned() else { return };
                    let Ok(r) = md.get_client_rect().await else {
                        return;
                    };
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

            // Measure floating element (wrapper)
            let Ok(cr) = wrapper_md.get_client_rect().await else {
                return;
            };
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

            // Build middleware chain (matching upstream popper.tsx lines 192-216)
            let placement = to_floating_placement(side, align);
            let padding = floating_ui::Padding::Uniform(collision_padding);

            // Upstream: offset({ mainAxis: sideOffset + arrowHeight, alignmentAxis: alignOffset })
            let mut middleware = vec![floating_ui::Middleware::Offset(
                floating_ui::OffsetOptions {
                    main_axis: side_offset + arrow_h,
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

            // Upstream: arrow && floatingUIarrow({ element: arrow, padding: arrowPadding })
            if has_arrow {
                middleware.push(floating_ui::Middleware::Arrow(floating_ui::ArrowOptions {
                    width: arrow_w,
                    height: arrow_h,
                    padding: floating_ui::Padding::Uniform(arrow_padding),
                }));
            }

            // Upstream: hideWhenDetached && hide({ strategy: 'referenceHidden', ...detectOverflowOptions })
            if hide_when_detached {
                middleware.push(floating_ui::Middleware::Hide(floating_ui::HideOptions {
                    strategy: floating_ui::HideStrategy::ReferenceHidden,
                    detect_overflow: floating_ui::DetectOverflowOptions {
                        padding,
                        ..Default::default()
                    },
                }));
            }

            let detect_overflow_fn =
                |state: &floating_ui::MiddlewareState,
                 opts: &floating_ui::DetectOverflowOptions| {
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

            // Compute transform origin (matching upstream transformOrigin middleware)
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

            // Read hide middleware data
            let is_ref_hidden = result
                .middleware_data
                .hide
                .is_some_and(|h| h.reference_hidden);

            // Read arrow middleware data
            let (ax, ay, center_off) = result
                .middleware_data
                .arrow
                .map(|a| (a.x, a.y, a.center_offset))
                .unwrap_or((None, None, 0.0));

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
            reference_hidden.set(is_ref_hidden);
            arrow_x.set(ax);
            arrow_y.set(ay);
            should_hide_arrow.set(center_off != 0.0);
        });
    });

    // Auto-update: set up scroll/resize listeners that bump `tick`
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
                    animation_frame,
                },
            );

            move || {
                cleanup();
            }
        });
    }

    // Upstream: onPlaced fires once when isPositioned becomes true
    {
        let prev_positioned = crate::use_previous(is_positioned.into());
        use_effect(move || {
            if is_positioned() && !prev_positioned() {
                if let Some(cb) = on_placed {
                    cb.call(());
                }
            }
        });
    }

    let prefix = props.css_var_prefix;

    // --- Wrapper style (matching upstream lines 241-258) ---
    // Upstream useFloating returns floatingStyles = { position: 'fixed', left: 0, top: 0,
    // transform: 'translate(x, y)' }. Position is always done via transform, not left/top.
    // When !isPositioned, transform is overridden with 'translate(0, -200%)' to hide off-page.
    let wrapper_style = {
        let z_index = content_z_index();
        let is_pos = is_positioned();

        let z_part = match &z_index {
            Some(z) => format!("z-index: {z};"),
            None => String::new(),
        };

        // transform: isPositioned ? floatingStyles.transform : 'translate(0, -200%)'
        let transform = if is_pos {
            if let (Some(x), Some(y)) = (pos_x(), pos_y()) {
                format!("translate({x}px, {y}px)")
            } else {
                "translate(0, -200%)".to_string()
            }
        } else {
            "translate(0, -200%)".to_string()
        };

        let mut style = format!(
            "position: fixed; left: 0px; top: 0px; \
             transform: {transform}; \
             min-width: max-content; {z_part}"
        );

        // Upstream: hide when reference is detached
        if reference_hidden() {
            style.push_str(" visibility: hidden; pointer-events: none;");
        }

        if is_pos {
            let to = transform_origin();
            let aw = avail_w();
            let ah = avail_h();
            let anw = anchor_w();
            let anh = anchor_h();

            use std::fmt::Write;
            let _ = write!(
                style,
                " --radix-popper-transform-origin: {to}; \
                 --radix-popper-available-width: {aw}px; \
                 --radix-popper-available-height: {ah}px; \
                 --radix-popper-anchor-width: {anw}px; \
                 --radix-popper-anchor-height: {anh}px;"
            );

            // Component-specific aliases (e.g. --radix-dropdown-menu-*)
            if let Some(p) = prefix {
                let _ = write!(
                    style,
                    " --radix-{p}-content-transform-origin: {to}; \
                     --radix-{p}-content-available-width: {aw}px; \
                     --radix-{p}-content-available-height: {ah}px; \
                     --radix-{p}-trigger-width: {anw}px; \
                     --radix-{p}-trigger-height: {anh}px;"
                );
            }
        }

        style
    };

    // --- Inner content style (matching upstream line 277-282) ---
    // Suppresses animation until positioned so entry animations don't fire with wrong placement.
    let content_style = if !is_positioned() {
        "animation: none;"
    } else {
        ""
    };

    let ctx_side: ReadSignal<Side> = placed_side.into();
    let ctx_align: ReadSignal<Align> = placed_align.into();
    let ctx_is_positioned: ReadSignal<bool> = is_positioned.into();
    let ctx_arrow_x: ReadSignal<Option<f64>> = arrow_x.into();
    let ctx_arrow_y: ReadSignal<Option<f64>> = arrow_y.into();
    let ctx_should_hide_arrow: ReadSignal<bool> = should_hide_arrow.into();

    let wrapper = rsx! {
        PopperContentCtxProvider {
            placed_side: ctx_side,
            placed_align: ctx_align,
            is_positioned: ctx_is_positioned,
            arrow_x: ctx_arrow_x,
            arrow_y: ctx_arrow_y,
            should_hide_arrow: ctx_should_hide_arrow,

            div {
                id: wrapper_id(),
                onmounted: move |evt| wrapper_ref.set(Some(evt.data())),
                "data-radix-popper-content-wrapper": "",
                style: "{wrapper_style}",

                // Inner content div — matches upstream Primitive.div (lines 272-283).
                // Receives consumer's class, content_attributes, data-side, data-align.
                div {
                    "data-side": placed_side().as_str(),
                    "data-align": placed_align().as_str(),
                    class: props.class,
                    style: "{content_style}",
                    onanimationend: move |e| {
                        if let Some(ref h) = props.on_animation_end {
                            h.call(e);
                        }
                    },
                    onpointerenter: move |e| {
                        if let Some(ref h) = props.on_pointer_enter {
                            h.call(e);
                        }
                    },
                    onpointerleave: move |e| {
                        if let Some(ref h) = props.on_pointer_leave {
                            h.call(e);
                        }
                    },
                    onkeydown: move |e| {
                        if let Some(ref h) = props.on_keydown {
                            h.call(e);
                        }
                    },
                    onblur: move |e| {
                        if let Some(ref h) = props.on_blur {
                            h.call(e);
                        }
                    },
                    onmounted: move |e| {
                        if let Some(ref h) = props.on_mounted {
                            h.call(e);
                        }
                    },
                    ..props.content_attributes,
                    {props.children}
                }
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
    is_positioned: ReadSignal<bool>,
    arrow_x: ReadSignal<Option<f64>>,
    arrow_y: ReadSignal<Option<f64>>,
    should_hide_arrow: ReadSignal<bool>,
    children: Element,
}

#[component]
fn PopperContentCtxProvider(props: PopperContentCtxProviderProps) -> Element {
    use_context_provider(|| PopperContentCtx {
        placed_side: props.placed_side,
        placed_align: props.placed_align,
        is_positioned: props.is_positioned,
        arrow_x: props.arrow_x,
        arrow_y: props.arrow_y,
        should_hide_arrow: props.should_hide_arrow,
    });
    props.children
}

// ---------------------------------------------------------------------------
// PopperArrow
// ---------------------------------------------------------------------------

/// Props for [`PopperArrow`].
#[derive(Props, Clone, PartialEq)]
pub struct PopperArrowProps {
    /// Arrow width in pixels.
    #[props(default = 10.0)]
    pub width: f64,

    /// Arrow height in pixels.
    #[props(default = 5.0)]
    pub height: f64,

    /// CSS class.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Upstream `PopperArrow` — positioned by arrow middleware data from context.
#[component]
pub fn PopperArrow(props: PopperArrowProps) -> Element {
    let content_ctx: PopperContentCtx = use_context();
    let side = (content_ctx.placed_side)();
    let base_side = side.opposite();
    let ax = (content_ctx.arrow_x)();
    let ay = (content_ctx.arrow_y)();
    let hide = (content_ctx.should_hide_arrow)();

    let transform = match side {
        Side::Top => "translateY(100%)",
        Side::Right => "translateY(50%) rotate(90deg) translateX(-50%)",
        Side::Bottom => "rotate(180deg)",
        Side::Left => "translateY(50%) rotate(-90deg) translateX(50%)",
    };

    // Upstream: positions via arrow middleware x/y, falls back to centered
    let left_part = match ax {
        Some(x) => format!("left: {x}px;"),
        None => "left: 50%; transform-origin: center;".to_string(),
    };
    let top_part = match ay {
        Some(y) => format!("top: {y}px;"),
        None => String::new(),
    };
    let visibility = if hide { " visibility: hidden;" } else { "" };

    let style = format!(
        "position: absolute; {left_part} {top_part} {base}: 0; transform: {transform};{visibility}",
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
