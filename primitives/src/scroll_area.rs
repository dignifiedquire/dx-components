//! ScrollArea primitive — matches `@radix-ui/react-scroll-area`.
//!
//! Provides a composable scrollable area with custom scrollbar support.
//! The simple API (children directly inside [`ScrollArea`]) works as a basic
//! CSS overflow wrapper. For full custom scrollbar support, use the
//! sub-component API:
//!
//! ```rust,no_run
//! # use dioxus::prelude::*;
//! # use dioxus_primitives::scroll_area::*;
//! fn Demo() -> Element {
//!     rsx! {
//!         ScrollArea {
//!             ScrollAreaViewport {
//!                 for i in 1..=50 {
//!                     p { "Item {i}" }
//!                 }
//!             }
//!             ScrollAreaScrollbar {
//!                 orientation: Orientation::Vertical,
//!                 ScrollAreaThumb {}
//!             }
//!             ScrollAreaCorner {}
//!         }
//!     }
//! }
//! ```

use std::time::Duration;

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// The direction in which scrolling is allowed.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ScrollDirection {
    /// Allow vertical scrolling only.
    Vertical,
    /// Allow horizontal scrolling only.
    Horizontal,
    /// Allow scrolling in both directions.
    #[default]
    Both,
}

/// The type of scrolling behavior (legacy API).
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ScrollType {
    /// Browser default scrolling.
    #[default]
    Auto,
    /// Always show scrollbars.
    Always,
    /// Hide scrollbars but enable scrolling.
    Hidden,
}

/// Scrollbar visibility mode (matching Radix's `type` prop).
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ScrollbarVisibility {
    /// Always visible.
    Always,
    /// Show only when content overflows (auto-detect).
    Auto,
    /// Show while scrolling, hide after delay.
    Scroll,
    /// Show on hover, hide after delay (default, matching Radix).
    #[default]
    Hover,
}

/// Scrollbar orientation.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum Orientation {
    /// Horizontal scrollbar.
    Horizontal,
    /// Vertical scrollbar.
    #[default]
    Vertical,
}

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

/// Internal context shared by ScrollArea sub-components.
///
/// ## Radix deviation
/// Radix uses element refs and ResizeObserver for measurement. We use
/// `document::eval` to read scrollWidth/scrollHeight/clientWidth/clientHeight
/// because Dioxus does not expose ResizeObserver or scrollable dimension
/// properties on MountedData.
#[derive(Clone, Copy)]
struct ScrollAreaCtx {
    /// Scroll position tracking.
    scroll_top: Signal<f64>,
    scroll_left: Signal<f64>,
    /// Content dimensions (scrollWidth, scrollHeight).
    content_width: Signal<f64>,
    content_height: Signal<f64>,
    /// Viewport dimensions (clientWidth, clientHeight).
    viewport_width: Signal<f64>,
    viewport_height: Signal<f64>,
    /// Whether each scrollbar axis is enabled (has content overflow).
    scrollbar_x_enabled: Signal<bool>,
    scrollbar_y_enabled: Signal<bool>,
    /// Visibility mode.
    visibility: ScrollbarVisibility,
    /// Delay before hiding scrollbar (ms).
    scroll_hide_delay: u64,
    /// For hover mode: whether pointer is over the scroll area.
    is_hovered: Signal<bool>,
    /// For scroll mode: whether actively scrolling.
    is_scrolling: Signal<bool>,
    /// Generation counter for hide timer cancellation.
    hide_gen: Signal<u64>,
    /// Corner dimensions for CSS custom properties.
    corner_width: Signal<f64>,
    corner_height: Signal<f64>,
    /// Unique ID for the viewport element (to avoid querySelectorAll conflicts
    /// when multiple ScrollAreas exist on the same page).
    viewport_id: Signal<String>,
}

// ---------------------------------------------------------------------------
// ScrollArea (root)
// ---------------------------------------------------------------------------

/// The props for the [`ScrollArea`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaProps {
    /// The scroll direction (legacy API).
    #[props(default)]
    pub direction: ScrollDirection,

    /// The scroll type (legacy API).
    #[props(default)]
    pub scroll_type: ScrollType,

    /// Scrollbar visibility mode (new sub-component API, matching Radix).
    #[props(default)]
    pub r#type: ScrollbarVisibility,

    /// Delay in ms before hiding scrollbar (for hover/scroll modes).
    /// Default 600ms, matching Radix.
    #[props(default = 600)]
    pub scroll_hide_delay: u64,

    /// Additional attributes to apply to the scroll area element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the scroll area component.
    pub children: Element,
}

/// # ScrollArea
///
/// The `ScrollArea` component creates a scrollable area. When used with
/// direct children (legacy API), it renders a scrollable div. When used
/// with sub-components ([`ScrollAreaViewport`], [`ScrollAreaScrollbar`],
/// [`ScrollAreaThumb`], [`ScrollAreaCorner`]), it provides a full custom
/// scrollbar experience matching Radix UI.
///
/// ## Example (simple)
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::scroll_area::{ScrollArea, ScrollDirection};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ScrollArea {
///             width: "10em",
///             height: "10em",
///             direction: ScrollDirection::Vertical,
///             tabindex: "0",
///             div { class: "scroll-content",
///                 for i in 1..=20 {
///                     p {
///                         "Scrollable content item {i}"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`ScrollArea`] component defines the following data attributes you can use to control styling:
/// - `data-scroll-direction`: Indicates the scroll direction. Values are `vertical`, `horizontal`, or `both`.
#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    let direction = props.direction;
    let scroll_type = props.scroll_type;

    let viewport_id = crate::use_unique_id();

    // Provide context for sub-components
    use_context_provider(|| ScrollAreaCtx {
        scroll_top: Signal::new(0.0),
        scroll_left: Signal::new(0.0),
        content_width: Signal::new(0.0),
        content_height: Signal::new(0.0),
        viewport_width: Signal::new(0.0),
        viewport_height: Signal::new(0.0),
        scrollbar_x_enabled: Signal::new(false),
        scrollbar_y_enabled: Signal::new(false),
        visibility: props.r#type,
        scroll_hide_delay: props.scroll_hide_delay,
        is_hovered: Signal::new(false),
        is_scrolling: Signal::new(false),
        hide_gen: Signal::new(0u64),
        corner_width: Signal::new(0.0),
        corner_height: Signal::new(0.0),
        viewport_id,
    });

    let mut state = use_context::<ScrollAreaCtx>();

    // Legacy rendering: apply overflow styles directly
    let (overflow_x, overflow_y, scrollbar_width) = match scroll_type {
        ScrollType::Auto => match direction {
            ScrollDirection::Vertical => (Some("hidden"), Some("auto"), None),
            ScrollDirection::Horizontal => (Some("auto"), Some("hidden"), None),
            ScrollDirection::Both => (Some("auto"), Some("auto"), None),
        },
        ScrollType::Always => match direction {
            ScrollDirection::Vertical => (Some("hidden"), Some("scroll"), None),
            ScrollDirection::Horizontal => (Some("scroll"), Some("hidden"), None),
            ScrollDirection::Both => (Some("scroll"), Some("scroll"), None),
        },
        ScrollType::Hidden => match direction {
            ScrollDirection::Vertical => (Some("hidden"), Some("scroll"), Some("none")),
            ScrollDirection::Horizontal => (Some("scroll"), Some("hidden"), Some("none")),
            ScrollDirection::Both => (Some("scroll"), Some("scroll"), Some("none")),
        },
    };

    rsx! {
        div {
            "data-slot": "scroll-area",
            overflow_x,
            overflow_y,
            "scrollbar-width": scrollbar_width,
            "data-scroll-direction": match direction {
                ScrollDirection::Vertical => "vertical",
                ScrollDirection::Horizontal => "horizontal",
                ScrollDirection::Both => "both",
            },
            style: "position: relative; --radix-scroll-area-corner-width: 0px; --radix-scroll-area-corner-height: 0px;",
            onpointerenter: move |_| {
                state.is_hovered.set(true);
                // Cancel any pending hide timer
                let current = *state.hide_gen.peek();
                state.hide_gen.set(current + 1);
            },
            onpointerleave: move |_| {
                state.is_hovered.set(false);
                // Start hide timer
                let gen = *state.hide_gen.peek() + 1;
                state.hide_gen.set(gen);
                let delay = state.scroll_hide_delay;
                let hide_gen = state.hide_gen;
                let mut is_scrolling = state.is_scrolling;
                spawn(async move {
                    dioxus_sdk_time::sleep(Duration::from_millis(delay)).await;
                    if *hide_gen.peek() == gen {
                        is_scrolling.set(false);
                    }
                });
            },
            ..props.attributes,

            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ScrollAreaViewport
// ---------------------------------------------------------------------------

/// Props for [`ScrollAreaViewport`].
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaViewportProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (scrollable content).
    pub children: Element,
}

/// The scrollable viewport container.
///
/// Hides native scrollbars using CSS and tracks scroll position and
/// dimensions for custom scrollbar sub-components.
///
/// ## Radix deviation
/// Radix injects a `<style>` tag with `dangerouslySetInnerHTML` to hide
/// native scrollbars cross-browser. We use inline `scrollbar-width: none`
/// (modern browsers) and inject a webkit rule via `document::eval` because
/// Dioxus has no API for injecting style elements.
#[component]
pub fn ScrollAreaViewport(props: ScrollAreaViewportProps) -> Element {
    let mut ctx = use_context::<ScrollAreaCtx>();
    let viewport_id = ctx.viewport_id;

    // Inject CSS to hide webkit scrollbars (runs once)
    // Radix deviation: Radix uses dangerouslySetInnerHTML for a <style> tag.
    // We use document::eval because Dioxus has no style injection API.
    use_effect(move || {
        document::eval(
            r#"
            if (!document.getElementById('__dxc_scroll_area_style')) {
                var style = document.createElement('style');
                style.id = '__dxc_scroll_area_style';
                style.textContent = '[data-slot="scroll-area-viewport"]::-webkit-scrollbar{display:none}';
                document.head.appendChild(style);
            }
            "#,
        );
    });

    // Measure dimensions on mount
    {
        let id = viewport_id.clone();
        use_effect(move || {
            let id = id.clone();
            spawn(async move {
                dioxus_sdk_time::sleep(Duration::from_millis(16)).await;
                let id_val = id();
                let mut reader = document::eval(&format!(
                    r#"
                    var el = document.getElementById('{id_val}');
                    if (el) {{
                        dioxus.send([el.scrollWidth, el.scrollHeight, el.clientWidth, el.clientHeight]);
                    }} else {{
                        dioxus.send([0, 0, 0, 0]);
                    }}
                    "#
                ));
                if let Ok(values) = reader.recv::<(f64, f64, f64, f64)>().await {
                    ctx.content_width.set(values.0);
                    ctx.content_height.set(values.1);
                    ctx.viewport_width.set(values.2);
                    ctx.viewport_height.set(values.3);
                    ctx.scrollbar_x_enabled.set(values.0 > values.2);
                    ctx.scrollbar_y_enabled.set(values.1 > values.3);
                }
            });
        });
    }

    let overflow_x = if (ctx.scrollbar_x_enabled)() {
        "scroll"
    } else {
        "hidden"
    };
    let overflow_y = if (ctx.scrollbar_y_enabled)() {
        "scroll"
    } else {
        "hidden"
    };

    rsx! {
        div {
            id: viewport_id,
            "data-slot": "scroll-area-viewport",
            style: "overflow-x: {overflow_x}; overflow-y: {overflow_y}; scrollbar-width: none; -ms-overflow-style: none; width: 100%; height: 100%;",
            class: props.class,
            // Track scroll position via onscroll
            // Radix deviation: Radix uses requestAnimationFrame-based scroll
            // tracking to avoid scroll-linked performance issues. We use the
            // onscroll event because Dioxus provides it natively.
            onscroll: move |_: Event<ScrollData>| {
                ctx.is_scrolling.set(true);

                // Start scroll-end hide timer
                let gen = *ctx.hide_gen.peek() + 1;
                ctx.hide_gen.set(gen);
                let delay = ctx.scroll_hide_delay;
                let hide_gen = ctx.hide_gen;
                let mut is_scrolling = ctx.is_scrolling;
                spawn(async move {
                    dioxus_sdk_time::sleep(Duration::from_millis(delay)).await;
                    if *hide_gen.peek() == gen {
                        is_scrolling.set(false);
                    }
                });

                // Read scroll position from the viewport element
                let id_val = viewport_id();
                spawn(async move {
                    let mut reader = document::eval(&format!(
                        r#"
                        var el = document.getElementById('{id_val}');
                        if (el) {{
                            dioxus.send([el.scrollTop, el.scrollLeft, el.scrollWidth, el.scrollHeight, el.clientWidth, el.clientHeight]);
                        }} else {{
                            dioxus.send([0, 0, 0, 0, 0, 0]);
                        }}
                        "#
                    ));
                    if let Ok(values) = reader.recv::<(f64, f64, f64, f64, f64, f64)>().await {
                        ctx.scroll_top.set(values.0);
                        ctx.scroll_left.set(values.1);
                        ctx.content_width.set(values.2);
                        ctx.content_height.set(values.3);
                        ctx.viewport_width.set(values.4);
                        ctx.viewport_height.set(values.5);
                        ctx.scrollbar_x_enabled.set(values.2 > values.4);
                        ctx.scrollbar_y_enabled.set(values.3 > values.5);
                    }
                });
            },
            ..props.attributes,
            // Radix wraps content in a div with display:table to match
            // content dimensions. We use min-width: 100% for the same effect.
            div {
                style: "min-width: 100%; display: table;",
                {props.children}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ScrollAreaScrollbar
// ---------------------------------------------------------------------------

/// Props for [`ScrollAreaScrollbar`].
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaScrollbarProps {
    /// Scrollbar orientation.
    #[props(default)]
    pub orientation: Orientation,

    /// Force the scrollbar to always mount.
    #[props(default)]
    pub force_mount: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (should contain a [`ScrollAreaThumb`]).
    pub children: Element,
}

/// Custom scrollbar track.
///
/// Contains a [`ScrollAreaThumb`] and handles click-to-scroll on the track.
/// Visibility is controlled by the root [`ScrollArea`]'s `type` prop.
///
/// Sets CSS custom properties for thumb sizing:
/// - Vertical: `--radix-scroll-area-thumb-height`
/// - Horizontal: `--radix-scroll-area-thumb-width`
#[component]
pub fn ScrollAreaScrollbar(props: ScrollAreaScrollbarProps) -> Element {
    let ctx = use_context::<ScrollAreaCtx>();
    let orientation = props.orientation;

    // Determine if this scrollbar should be visible
    let is_enabled = match orientation {
        Orientation::Horizontal => (ctx.scrollbar_x_enabled)(),
        Orientation::Vertical => (ctx.scrollbar_y_enabled)(),
    };

    let is_visible = match ctx.visibility {
        ScrollbarVisibility::Always => is_enabled,
        ScrollbarVisibility::Auto => is_enabled,
        ScrollbarVisibility::Hover => is_enabled && ((ctx.is_hovered)() || (ctx.is_scrolling)()),
        ScrollbarVisibility::Scroll => is_enabled && (ctx.is_scrolling)(),
    };

    if !is_visible && !props.force_mount {
        return rsx! {};
    }

    // Calculate thumb size ratio
    let (content_size, viewport_size) = match orientation {
        Orientation::Horizontal => ((ctx.content_width)(), (ctx.viewport_width)()),
        Orientation::Vertical => ((ctx.content_height)(), (ctx.viewport_height)()),
    };

    let thumb_ratio = if content_size > 0.0 {
        (viewport_size / content_size).min(1.0)
    } else {
        0.0
    };

    // CSS custom property for thumb size (matching Radix)
    let thumb_css_var = match orientation {
        Orientation::Horizontal => {
            format!(
                "--radix-scroll-area-thumb-width: {:.1}%",
                thumb_ratio * 100.0
            )
        }
        Orientation::Vertical => {
            format!(
                "--radix-scroll-area-thumb-height: {:.1}%",
                thumb_ratio * 100.0
            )
        }
    };

    let data_state = if is_visible { "visible" } else { "hidden" };
    let data_orientation = match orientation {
        Orientation::Horizontal => "horizontal",
        Orientation::Vertical => "vertical",
    };

    // Provide scrollbar context for thumb
    use_context_provider(|| ScrollbarCtx {
        orientation,
        thumb_ratio,
    });

    rsx! {
        div {
            "data-slot": "scroll-area-scrollbar",
            "data-state": data_state,
            "data-orientation": data_orientation,
            style: "{thumb_css_var};",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

/// Internal context for scrollbar → thumb communication.
#[derive(Clone, Copy)]
struct ScrollbarCtx {
    orientation: Orientation,
    thumb_ratio: f64,
}

// ---------------------------------------------------------------------------
// ScrollAreaThumb
// ---------------------------------------------------------------------------

/// Props for [`ScrollAreaThumb`].
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaThumbProps {
    /// Force mount the thumb.
    #[props(default)]
    pub force_mount: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Draggable scrollbar thumb.
///
/// Sized proportionally to the viewport/content ratio, positioned based
/// on the current scroll offset. Supports pointer drag interaction.
///
/// ## Radix deviation
/// Radix uses `element.setPointerCapture()` for drag tracking and
/// `transform: translate3d()` for positioning. We use `document::eval`
/// for pointer capture (Dioxus doesn't expose this API) and CSS
/// `transform` for positioning.
#[component]
pub fn ScrollAreaThumb(props: ScrollAreaThumbProps) -> Element {
    let ctx = use_context::<ScrollAreaCtx>();
    let scrollbar_ctx = use_context::<ScrollbarCtx>();

    let orientation = scrollbar_ctx.orientation;
    let thumb_ratio = scrollbar_ctx.thumb_ratio;

    // Don't render if thumb would fill the entire track (nothing to scroll)
    let has_thumb = thumb_ratio > 0.0 && thumb_ratio < 1.0;
    if !has_thumb && !props.force_mount {
        return rsx! {};
    }

    // Calculate thumb position as percentage of track
    let (scroll_offset, content_size, viewport_size) = match orientation {
        Orientation::Horizontal => (
            (ctx.scroll_left)(),
            (ctx.content_width)(),
            (ctx.viewport_width)(),
        ),
        Orientation::Vertical => (
            (ctx.scroll_top)(),
            (ctx.content_height)(),
            (ctx.viewport_height)(),
        ),
    };

    let max_scroll = (content_size - viewport_size).max(0.0);
    let scroll_fraction = if max_scroll > 0.0 {
        (scroll_offset / max_scroll).clamp(0.0, 1.0)
    } else {
        0.0
    };

    // Thumb position: fraction of (track_size - thumb_size)
    // Since thumb_size = track_size * thumb_ratio,
    // thumb_pos = scroll_fraction * track_size * (1 - thumb_ratio)
    // As percentage of track: scroll_fraction * (1 - thumb_ratio) * 100%
    let thumb_offset_pct = scroll_fraction * (1.0 - thumb_ratio) * 100.0;

    let (size_style, transform) = match orientation {
        Orientation::Horizontal => (
            format!(
                "width: var(--radix-scroll-area-thumb-width, {:.1}%); height: 100%;",
                thumb_ratio * 100.0
            ),
            format!("translate3d({thumb_offset_pct:.2}%, 0, 0)"),
        ),
        Orientation::Vertical => (
            format!(
                "height: var(--radix-scroll-area-thumb-height, {:.1}%); width: 100%;",
                thumb_ratio * 100.0
            ),
            format!("translate3d(0, {thumb_offset_pct:.2}%, 0)"),
        ),
    };

    // Drag state
    let mut is_dragging = use_signal(|| false);
    let mut drag_start_scroll = use_signal(|| 0.0);
    let mut drag_start_pointer = use_signal(|| 0.0);

    let thumb_id = crate::use_unique_id();

    let style = format!("{size_style} transform: {transform}; border-radius: inherit;",);

    rsx! {
        div {
            id: thumb_id,
            "data-slot": "scroll-area-thumb",
            "data-state": if has_thumb { "visible" } else { "hidden" },
            style: style,
            class: props.class,
            onpointerdown: move |e: PointerEvent| {
                e.prevent_default();
                is_dragging.set(true);

                // Store starting positions
                match orientation {
                    Orientation::Horizontal => {
                        drag_start_scroll.set((ctx.scroll_left)());
                        drag_start_pointer.set(e.client_coordinates().x);
                    }
                    Orientation::Vertical => {
                        drag_start_scroll.set((ctx.scroll_top)());
                        drag_start_pointer.set(e.client_coordinates().y);
                    }
                }

                // Capture pointer for drag tracking
                // Radix deviation: Radix calls element.setPointerCapture().
                // We use document::eval because Dioxus doesn't expose this API.
                let pointer_id = e.data().pointer_id();
                let id_val = thumb_id();
                document::eval(&format!(
                    r#"
                    var thumb = document.getElementById('{id_val}');
                    if (thumb) thumb.setPointerCapture({pointer_id});
                    "#
                ));
            },
            onpointermove: move |e: PointerEvent| {
                if !is_dragging() {
                    return;
                }

                let current_pos = match orientation {
                    Orientation::Horizontal => e.client_coordinates().x,
                    Orientation::Vertical => e.client_coordinates().y,
                };

                let delta = current_pos - drag_start_pointer();

                // Convert pointer delta to scroll delta.
                // The thumb moves within the track; its movement maps to
                // the full scroll range. scroll_delta = pointer_delta / thumb_ratio
                let scroll_multiplier = if thumb_ratio > 0.0 {
                    1.0 / thumb_ratio
                } else {
                    1.0
                };

                let new_scroll = drag_start_scroll() + delta * scroll_multiplier;
                let clamped = new_scroll.clamp(0.0, max_scroll);

                // Apply scroll via eval
                let prop = match orientation {
                    Orientation::Horizontal => "scrollLeft",
                    Orientation::Vertical => "scrollTop",
                };
                let viewport_id_val = (ctx.viewport_id)();
                document::eval(&format!(
                    r#"
                    var el = document.getElementById('{viewport_id_val}');
                    if (el) el.{prop} = {clamped};
                    "#
                ));
            },
            onpointerup: move |_| {
                is_dragging.set(false);
            },
            ..props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// ScrollAreaCorner
// ---------------------------------------------------------------------------

/// Props for [`ScrollAreaCorner`].
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaCornerProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    #[props(default)]
    pub children: Element,
}

/// Corner element rendered when both horizontal and vertical scrollbars
/// are visible.
///
/// Fills the space where the two scrollbars meet, positioned at the
/// bottom-right corner.
///
/// ## Radix deviation
/// Radix uses ResizeObserver on scrollbar elements to measure corner
/// dimensions. We use a fixed approach since scrollbar dimensions are
/// determined by CSS rather than measured dynamically.
#[component]
pub fn ScrollAreaCorner(props: ScrollAreaCornerProps) -> Element {
    let ctx = use_context::<ScrollAreaCtx>();

    // Only render when both scrollbars are present and type != scroll
    let both_visible = (ctx.scrollbar_x_enabled)() && (ctx.scrollbar_y_enabled)();
    let show = both_visible && ctx.visibility != ScrollbarVisibility::Scroll;

    if !show {
        return rsx! {};
    }

    rsx! {
        div {
            "data-slot": "scroll-area-corner",
            style: "position: absolute; right: 0; bottom: 0;",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}
