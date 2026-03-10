//! Resizable panel primitive — split views with draggable handles.
//!
//! Provides [`ResizablePanelGroup`], [`ResizablePanel`], and [`ResizableHandle`]
//! for building resizable split-panel layouts. Matches the API shape of
//! react-resizable-panels used by shadcn/ui.

use crate::direction::Orientation;
use dioxus::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

/// Shared context between panel group, panels, and handles.
#[derive(Clone, Copy)]
pub struct ResizablePanelGroupCtx {
    /// The orientation of the panel group layout.
    pub orientation: Orientation,
    /// Panel sizes as percentages (should sum to ~100).
    sizes: Signal<Vec<f64>>,
    /// Per-panel (min_size, max_size) constraints.
    constraints: Signal<Vec<(f64, f64)>>,
    /// Counter for handle registration.
    handle_count: Signal<usize>,
    /// The pixel size of the group element in the layout direction.
    group_size_px: Signal<Option<f64>>,
    /// Layout change callback.
    on_layout_change: Option<Callback<Vec<f64>>>,
}

impl ResizablePanelGroupCtx {
    /// Resize panels adjacent to `handle_index` by `delta_percent`.
    ///
    /// Positive delta grows the panel before the handle and shrinks the one after.
    pub fn resize(&mut self, handle_index: usize, delta_percent: f64) {
        let mut sizes = self.sizes.write();
        let constraints = self.constraints.read();

        let before = handle_index;
        let after = handle_index + 1;
        if after >= sizes.len() {
            return;
        }

        let total = sizes[before] + sizes[after];
        let (min_b, max_b) = constraints[before];
        let (min_a, max_a) = constraints[after];

        // Effective range for "before" panel considering both panels' constraints.
        let effective_min = min_b.max(total - max_a);
        let effective_max = max_b.min(total - min_a);

        let new_before = (sizes[before] + delta_percent).clamp(effective_min, effective_max);
        sizes[before] = new_before;
        sizes[after] = total - new_before;

        if let Some(cb) = self.on_layout_change {
            let layout = sizes.clone();
            drop(sizes);
            cb.call(layout);
        }
    }
}

/// Access the [`ResizablePanelGroupCtx`] from a child component.
pub fn use_resizable_panel_group() -> ResizablePanelGroupCtx {
    use_context::<ResizablePanelGroupCtx>()
}

// ---------------------------------------------------------------------------
// ResizablePanelGroup
// ---------------------------------------------------------------------------

/// Props for [`ResizablePanelGroup`].
#[derive(Props, Clone, PartialEq)]
pub struct ResizablePanelGroupProps {
    /// Layout direction. Defaults to `Horizontal`.
    #[props(default = Orientation::Horizontal)]
    pub orientation: Orientation,

    /// Called whenever the layout changes (panel sizes as percentages).
    #[props(default)]
    pub on_layout_change: Option<Callback<Vec<f64>>>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (panels and handles).
    pub children: Element,
}

/// Container for resizable panels.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::resizable::*;
/// rsx! {
///     ResizablePanelGroup {
///         ResizablePanel { default_size: 50.0, "Left" }
///         ResizableHandle {}
///         ResizablePanel { default_size: 50.0, "Right" }
///     }
/// };
/// ```
#[component]
pub fn ResizablePanelGroup(props: ResizablePanelGroupProps) -> Element {
    let orientation = props.orientation;
    let orientation_str = orientation.as_str();

    let sizes = use_signal(Vec::new);
    let constraints = use_signal(Vec::new);
    let handle_count = use_signal(|| 0usize);
    let mut group_size_px: Signal<Option<f64>> = use_signal(|| None);

    use_context_provider(|| ResizablePanelGroupCtx {
        orientation,
        sizes,
        constraints,
        handle_count,
        group_size_px,
        on_layout_change: props.on_layout_change,
    });

    let mut el_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);

    rsx! {
        div {
            "data-slot": "resizable-panel-group",
            role: "group",
            "data-orientation": orientation_str,
            aria_orientation: orientation_str,

            onmounted: move |evt| async move {
                if let Ok(r) = evt.data().get_client_rect().await {
                    let size = match orientation {
                        Orientation::Horizontal => r.width(),
                        Orientation::Vertical => r.height(),
                    };
                    group_size_px.set(Some(size));
                }
                el_ref.set(Some(evt.data()));
            },
            onresize: move |_| async move {
                if let Some(el) = el_ref() {
                    if let Ok(r) = el.get_client_rect().await {
                        let size = match orientation {
                            Orientation::Horizontal => r.width(),
                            Orientation::Vertical => r.height(),
                        };
                        group_size_px.set(Some(size));
                    }
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ResizablePanel
// ---------------------------------------------------------------------------

/// Props for [`ResizablePanel`].
#[derive(Props, Clone, PartialEq)]
pub struct ResizablePanelProps {
    /// Default size as a percentage (0–100). Panels in a group should sum to 100.
    #[props(default = 50.0)]
    pub default_size: f64,

    /// Minimum size as a percentage.
    #[props(default = 0.0)]
    pub min_size: f64,

    /// Maximum size as a percentage.
    #[props(default = 100.0)]
    pub max_size: f64,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// An individual resizable panel within a [`ResizablePanelGroup`].
#[component]
pub fn ResizablePanel(props: ResizablePanelProps) -> Element {
    let mut ctx = use_context::<ResizablePanelGroupCtx>();

    // Register this panel on first render; the index is stable across re-renders.
    let panel_index = use_hook(|| {
        let index = ctx.sizes.read().len();
        ctx.sizes.write().push(props.default_size);
        ctx.constraints
            .write()
            .push((props.min_size, props.max_size));
        index
    });

    let size = ctx.sizes.read().get(panel_index).copied().unwrap_or(50.0);
    let style = format!("flex: 0 0 {size}%; overflow: hidden;");

    rsx! {
        div {
            "data-slot": "resizable-panel",
            "data-panel-index": "{panel_index}",
            "data-panel-size": "{size}",
            style: "{style}",
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ResizableHandle
// ---------------------------------------------------------------------------

/// Props for [`ResizableHandle`].
#[derive(Props, Clone, PartialEq)]
pub struct ResizableHandleProps {
    /// Whether the handle is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (optional visual grip content).
    pub children: Element,
}

/// A draggable/keyboard-accessible handle between two [`ResizablePanel`]s.
///
/// Implements the WAI-ARIA window splitter pattern with `role="separator"`.
#[component]
pub fn ResizableHandle(props: ResizableHandleProps) -> Element {
    let mut ctx = use_context::<ResizablePanelGroupCtx>();

    // Register handle and get stable index.
    let handle_index = use_hook(|| {
        let index = *ctx.handle_count.read();
        *ctx.handle_count.write() = index + 1;
        index
    });

    // Unique ID for pointer capture.
    let handle_id = use_hook(|| {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    });

    // The handle orientation label is perpendicular to the group's orientation.
    let aria_orientation = match ctx.orientation {
        Orientation::Horizontal => "vertical",
        Orientation::Vertical => "horizontal",
    };

    let mut dragging = use_signal(|| false);
    let mut last_pos = use_signal(|| None::<f64>);

    // Current value for ARIA (size of the panel before this handle).
    let panel_size = ctx.sizes.read().get(handle_index).copied().unwrap_or(0.0);
    let (min_size, _max_size) = ctx
        .constraints
        .read()
        .get(handle_index)
        .copied()
        .unwrap_or((0.0, 100.0));

    let resize_step = 5.0;

    rsx! {
        div {
            "data-slot": "resizable-handle",
            "data-resize-handle-id": "{handle_id}",
            "data-handle-index": "{handle_index}",
            "data-dragging": if dragging() { "true" } else { "false" },
            role: "separator",
            aria_orientation,
            aria_valuenow: "{panel_size:.0}",
            aria_valuemin: "{min_size:.0}",
            aria_valuemax: "100",
            tabindex: if props.disabled { None::<&str> } else { Some("0") },

            onkeydown: move |evt| {
                if props.disabled {
                    return;
                }

                let key = evt.data().key();
                let delta = match (key, ctx.orientation) {
                    (Key::ArrowLeft, Orientation::Horizontal)
                    | (Key::ArrowUp, Orientation::Vertical) => -resize_step,
                    (Key::ArrowRight, Orientation::Horizontal)
                    | (Key::ArrowDown, Orientation::Vertical) => resize_step,
                    (Key::Home, _) => -100.0,
                    (Key::End, _) => 100.0,
                    _ => return,
                };

                evt.prevent_default();
                ctx.resize(handle_index, delta);
            },

            onpointerdown: move |evt| {
                if props.disabled {
                    return;
                }
                evt.prevent_default();

                let pos = match ctx.orientation {
                    Orientation::Horizontal => evt.client_coordinates().x,
                    Orientation::Vertical => evt.client_coordinates().y,
                };
                last_pos.set(Some(pos));
                dragging.set(true);

                // Set pointer capture so we get pointermove/up even outside the handle.
                let pointer_id = evt.data().pointer_id();
                dioxus::document::eval(&format!(
                    "let el=document.querySelector('[data-resize-handle-id=\"{handle_id}\"]');if(el)el.setPointerCapture({pointer_id});"
                ));
            },

            onpointermove: move |evt| {
                if !dragging() || props.disabled {
                    return;
                }

                let pos = match ctx.orientation {
                    Orientation::Horizontal => evt.client_coordinates().x,
                    Orientation::Vertical => evt.client_coordinates().y,
                };

                if let Some(prev) = last_pos() {
                    let delta_px = pos - prev;
                    if let Some(group_px) = (ctx.group_size_px)() {
                        if group_px > 0.0 {
                            let delta_pct = (delta_px / group_px) * 100.0;
                            ctx.resize(handle_index, delta_pct);
                        }
                    }
                }
                last_pos.set(Some(pos));
            },

            onpointerup: move |_| {
                dragging.set(false);
                last_pos.set(None);
            },

            ..props.attributes,
            {props.children}
        }
    }
}
