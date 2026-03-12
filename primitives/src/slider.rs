//! Defines the [`Slider`] component and its sub-components, which provide a range input control for selecting a value within a specified range.

use crate::dioxus_core::{queue_effect, Runtime};
use crate::use_controlled;
use dioxus::html::geometry::euclid::Rect;
use dioxus::html::geometry::euclid::Vector2D;
use dioxus::html::geometry::ClientPoint;
use dioxus::html::geometry::Pixels;
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use std::rc::Rc;

/// The value of the slider. Supports single value or multiple values for
/// range sliders (matching Radix's `values: number[]` API).
#[derive(Debug, Clone, PartialEq)]
pub enum SliderValue {
    /// A single value for the slider.
    Single(f64),
    /// Multiple values for a range slider (multi-thumb).
    Multi(Vec<f64>),
}

impl SliderValue {
    /// Get the value at the given thumb index.
    pub fn get(&self, index: usize) -> Option<f64> {
        match self {
            SliderValue::Single(v) => {
                if index == 0 {
                    Some(*v)
                } else {
                    None
                }
            }
            SliderValue::Multi(vs) => vs.get(index).copied(),
        }
    }

    /// Get all values as a slice.
    pub fn values(&self) -> Vec<f64> {
        match self {
            SliderValue::Single(v) => vec![*v],
            SliderValue::Multi(vs) => vs.clone(),
        }
    }

    /// Number of values (thumbs).
    pub fn len(&self) -> usize {
        match self {
            SliderValue::Single(_) => 1,
            SliderValue::Multi(vs) => vs.len(),
        }
    }

    /// Returns true if there are no values.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Minimum value across all thumbs.
    pub fn min_value(&self) -> f64 {
        match self {
            SliderValue::Single(v) => *v,
            SliderValue::Multi(vs) => vs.iter().copied().fold(f64::INFINITY, f64::min),
        }
    }

    /// Maximum value across all thumbs.
    pub fn max_value(&self) -> f64 {
        match self {
            SliderValue::Single(v) => *v,
            SliderValue::Multi(vs) => vs.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        }
    }

    /// Set the value at the given index, returning a new SliderValue.
    fn with_value_at(&self, index: usize, new_val: f64) -> SliderValue {
        match self {
            SliderValue::Single(_) => {
                if index == 0 {
                    SliderValue::Single(new_val)
                } else {
                    self.clone()
                }
            }
            SliderValue::Multi(vs) => {
                let mut new = vs.clone();
                if index < new.len() {
                    new[index] = new_val;
                }
                SliderValue::Multi(new)
            }
        }
    }
}

impl From<f64> for SliderValue {
    fn from(v: f64) -> Self {
        SliderValue::Single(v)
    }
}

impl std::fmt::Display for SliderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SliderValue::Single(v) => write!(f, "{v}"),
            SliderValue::Multi(vs) => {
                let strs: Vec<String> = vs.iter().map(|v| v.to_string()).collect();
                write!(f, "{}", strs.join(", "))
            }
        }
    }
}

#[derive(Debug)]
struct Pointer {
    id: i32,
    position: ClientPoint,
}

static POINTERS: GlobalSignal<Vec<Pointer>> = Global::new(|| {
    let runtime = Runtime::current();
    queue_effect(move || {
        runtime.spawn(ScopeId::ROOT, async move {
            let mut pointer_updates = dioxus::document::eval(
                "window.addEventListener('pointerdown', (e) => {
                    dioxus.send(['down', [e.pointerId, e.pageX, e.pageY]]);
                });
                window.addEventListener('pointermove', (e) => {
                    dioxus.send(['move', [e.pointerId, e.pageX, e.pageY]]);
                });
                window.addEventListener('pointerup', (e) => {
                    dioxus.send(['up', [e.pointerId, e.pageX, e.pageY]]);
                });",
            );

            while let Ok((event_type, (pointer_id, x, y))) =
                pointer_updates.recv::<(String, (i32, f64, f64))>().await
            {
                let position = ClientPoint::new(x, y);

                match event_type.as_str() {
                    "down" => {
                        // Add a new pointer
                        POINTERS.write().push(Pointer {
                            id: pointer_id,
                            position,
                        });
                    }
                    "move" => {
                        // Update the position of an existing pointer
                        if let Some(pointer) =
                            POINTERS.write().iter_mut().find(|p| p.id == pointer_id)
                        {
                            pointer.position = position;
                        }
                    }
                    "up" => {
                        // Remove the pointer
                        POINTERS.write().retain(|p| p.id != pointer_id);
                    }
                    _ => {}
                }
            }
        });
    });

    Vec::new()
});

/// The props for the [`Slider`] component
#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    /// The controlled value of the slider
    pub value: ReadSignal<Option<SliderValue>>,

    /// The default value when uncontrolled
    #[props(default = SliderValue::Single(0.0))]
    pub default_value: SliderValue,

    /// The minimum value
    #[props(default = 0.0)]
    pub min: ReadSignal<f64>,

    /// The maximum value
    #[props(default = 100.0)]
    pub max: ReadSignal<f64>,

    /// The step value
    #[props(default = 1.0)]
    pub step: ReadSignal<f64>,

    /// Whether the slider is disabled
    #[props(default)]
    pub disabled: bool,

    /// Orientation of the slider
    #[props(default = true)]
    pub horizontal: bool,

    /// Inverts the order of the values
    #[props(default)]
    pub inverted: bool,

    /// Minimum number of steps between thumbs (for multi-thumb sliders).
    /// Matching Radix's `minStepsBetweenThumbs` prop. Default 0.
    #[props(default = 0.0)]
    pub min_steps_between_thumbs: f64,

    /// Callback when value changes
    #[props(default)]
    pub on_value_change: Callback<SliderValue>,

    /// The label for the slider (for accessibility)
    pub label: ReadSignal<Option<String>>,

    /// Additional attributes for the slider
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the slider
    pub children: Element,
}

/// # Slider
///
/// The `Slider` component is a range input control that allows users to select a value along a
/// [`SliderTrack`] by dragging a [`SliderThumb`] with the pointer or using the arrow keys.
///
/// Supports multiple thumbs for range selection (matching Radix's multi-value API).
/// Use `SliderValue::Multi(vec![...])` as the default value and place multiple
/// `SliderThumb` components with different `index` values.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::slider::{Slider, SliderRange, SliderThumb, SliderTrack};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Slider {
///             label: "Demo Slider",
///             horizontal: true,
///             SliderTrack {
///                 SliderRange {}
///                 SliderThumb {}
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Slider`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Present when the slider is disabled (attribute present with empty value or absent).
/// - `data-orientation`: Indicates the orientation of the slider. Values are `horizontal` or `vertical`.
#[component]
pub fn Slider(props: SliderProps) -> Element {
    let (value, set_value) = use_controlled(
        props.value,
        props.default_value.clone(),
        props.on_value_change,
    );

    let orientation = if props.horizontal {
        "horizontal"
    } else {
        "vertical"
    };

    let mut dragging = use_signal(|| false);

    let is_disabled = props.disabled;

    let mut ctx = use_context_provider(|| SliderContext {
        value,
        set_value,
        min: props.min,
        max: props.max,
        step: props.step,
        disabled: is_disabled,
        horizontal: props.horizontal,
        inverted: props.inverted,
        dragging: dragging.into(),
        label: props.label,
        active_thumb_index: Signal::new(None),
        min_steps_between_thumbs: props.min_steps_between_thumbs,
    });

    let mut rect = use_signal(|| None);
    let mut div_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let mut granular_value = use_hook(|| CopyValue::new(props.default_value.clone()));

    let size = rect().map(|r: Rect<f64, Pixels>| {
        if props.horizontal {
            r.width()
        } else {
            r.height()
        }
    });

    let mut current_pointer_id: Signal<Option<i32>> = use_signal(|| None);
    let mut last_processed_pos = use_hook(|| CopyValue::new(None));

    use_effect(move || {
        let pointers = POINTERS.read();

        if !dragging() {
            return;
        }

        let Some(size) = size else {
            return;
        };

        let Some(active_pointer_id) = current_pointer_id() else {
            return;
        };

        let Some(pointer) = pointers.iter().find(|p| p.id == active_pointer_id) else {
            current_pointer_id.take();
            last_processed_pos.set(None);
            return;
        };

        let delta = if let Some(last_pos) = last_processed_pos.replace(Some(pointer.position)) {
            pointer.position - last_pos
        } else {
            Vector2D::zero()
        };

        let delta_pos = if ctx.horizontal { delta.x } else { delta.y } as f64;

        let delta = delta_pos / size * ctx.range_size();

        // Get the active thumb index (default to 0 for single-value)
        let thumb_idx = ctx.active_thumb_index.peek().unwrap_or(0);
        let granular = granular_value.cloned();
        let current_val = granular.get(thumb_idx).unwrap_or(0.0);
        let new = current_val + delta;
        granular_value.set(granular.with_value_at(thumb_idx, new));

        let clamped = ctx.clamp_and_snap_for_thumb(new, thumb_idx);
        ctx.set_value
            .call(ctx.value.cloned().with_value_at(thumb_idx, clamped));
    });

    rsx! {
        span {
            "data-slot": "slider",
            "data-disabled": if is_disabled { "" } else { None::<&str> },
            "data-orientation": orientation,
            aria_disabled: if is_disabled { Some("true") } else { None },

            onmounted: move |evt| async move {
                // Get the bounding rect of the slider
                if let Ok(r) = evt.data().get_client_rect().await {
                    rect.set(Some(r));
                }
                div_element.set(Some(evt.data()));
            },
            onresize: move |_| async move {
                // Update the rect on resize
                let Some(div_element) = div_element() else {
                    return;
                };
                if let Ok(r) = div_element.get_client_rect().await {
                    rect.set(Some(r));
                }
            },
            onpointerdown: move |evt| {
                if ctx.disabled {
                    return;
                }

                // Prevent default to avoid loosing focus on the range
                evt.prevent_default();
                evt.stop_propagation();

                if current_pointer_id.read().is_some() || evt.trigger_button() != Some(MouseButton::Primary) {
                    return;
                }

                current_pointer_id.set(Some(evt.data().pointer_id()));
                POINTERS.write().push(Pointer {
                    id: evt.data().pointer_id(),
                    position: evt.client_coordinates(),
                });

                // Handle pointer interaction
                spawn(async move {
                    let Some(div_element) = div_element() else {
                        return;
                    };

                    // Update the bounding rect of the slider in case it moved
                    if let Ok(r) = div_element.get_client_rect().await {
                        rect.set(Some(r));

                        let size = if props.horizontal {
                            r.width()
                        } else {
                            r.height()
                        };

                        // Get the mouse position relative to the slider
                        let top_left = r.origin;
                        let relative_pos = evt.client_coordinates() - top_left.cast_unit();

                        let offset = if ctx.horizontal {
                            relative_pos.x
                        } else {
                            relative_pos.y
                        };
                        let click_value = (offset / size) * ctx.range_size() + (ctx.min)();

                        // Find the closest thumb to the click position
                        // (matching Radix's behavior for multi-thumb sliders)
                        let current = ctx.value.cloned();
                        let closest_idx = find_closest_thumb(&current, click_value);
                        ctx.active_thumb_index.set(Some(closest_idx));

                        let snapped = ctx.clamp_and_snap_for_thumb(click_value, closest_idx);
                        granular_value.set(current.with_value_at(closest_idx, click_value));
                        ctx.set_value.call(current.with_value_at(closest_idx, snapped));
                    }

                    dragging.set(true);
                });
            },

            ..props.attributes,

            {props.children}
        }
    }
}

/// Find the thumb index closest to the given value.
fn find_closest_thumb(slider_value: &SliderValue, target: f64) -> usize {
    let values = slider_value.values();
    if values.len() <= 1 {
        return 0;
    }
    values
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            (target - *a)
                .abs()
                .partial_cmp(&(target - *b).abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(i, _)| i)
        .unwrap_or(0)
}

/// The props for the [`SliderTrack`] component
#[derive(Props, Clone, PartialEq)]
pub struct SliderTrackProps {
    /// Additional attributes to apply to the track element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the track which should include a [`SliderThumb`]
    pub children: Element,
}

/// # SliderTrack
///
/// The track component for the [`Slider`] that represents the full range of the slider. This serves as the
/// container for the [`SliderRange`] and provides the background track. Clicking along the track will update
/// the value of the slider and move the [`SliderThumb`] to the new position.
///
/// This must be used inside a [`Slider`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::slider::{Slider, SliderRange, SliderThumb, SliderTrack};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Slider {
///             label: "Demo Slider",
///             horizontal: true,
///             SliderTrack {
///                 SliderRange {}
///                 SliderThumb {}
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`SliderTrack`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Present when the slider is disabled.
/// - `data-orientation`: Indicates the orientation of the slider. Values are `horizontal` or `vertical`.
#[component]
pub fn SliderTrack(props: SliderTrackProps) -> Element {
    let ctx = use_context::<SliderContext>();
    let orientation = if ctx.horizontal {
        "horizontal"
    } else {
        "vertical"
    };

    rsx! {
        span {
            "data-slot": "slider-track",
            "data-disabled": if ctx.disabled { "" } else { None::<&str> },
            "data-orientation": orientation,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`SliderRange`] component
#[derive(Props, Clone, PartialEq)]
pub struct SliderRangeProps {
    /// Additional attributes to apply to the range element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the range element
    pub children: Element,
}

/// # SliderRange
///
/// The range component for the [`Slider`] that visually represents the selected portion of the slider track.
///
/// For single-value sliders, the range spans from `min` to the value.
/// For multi-thumb sliders, the range spans from the minimum to maximum
/// of all thumb values (matching Radix's behavior).
///
/// This must be used inside a [`SliderTrack`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::slider::{Slider, SliderRange, SliderThumb, SliderTrack};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Slider {
///             label: "Demo Slider",
///             horizontal: true,
///             SliderTrack {
///                 SliderRange {}
///                 SliderThumb {}
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`SliderRange`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Present when the slider is disabled.
/// - `data-orientation`: Indicates the orientation of the slider. Values are `horizontal` or `vertical`.
///
/// It automatically has the percentage based size and position styles applied based on the current slider value.
#[component]
pub fn SliderRange(props: SliderRangeProps) -> Element {
    let ctx = use_context::<SliderContext>();
    let orientation = if ctx.horizontal {
        "horizontal"
    } else {
        "vertical"
    };

    let style = use_memo(move || {
        let val = (ctx.value)();

        // For single-value sliders: range from min to value.
        // For multi-thumb sliders: range from min(values) to max(values).
        // This matches Radix's SliderRange behavior.
        let (start, end) = match &val {
            SliderValue::Single(v) => ((ctx.min)(), *v),
            SliderValue::Multi(_) => (val.min_value(), val.max_value()),
        };

        let start_percent = ctx.as_percent(start);
        let end_percent = ctx.as_percent(end);

        if ctx.horizontal {
            format!("left: {}%; right: {}%", start_percent, 100.0 - end_percent)
        } else {
            format!("bottom: {}%; top: {}%", start_percent, 100.0 - end_percent)
        }
    });

    rsx! {
        span {
            "data-slot": "slider-range",
            "data-disabled": if ctx.disabled { "" } else { None::<&str> },
            "data-orientation": orientation,
            style,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`SliderThumb`] component
#[derive(Props, Clone, PartialEq)]
pub struct SliderThumbProps {
    /// Which thumb this is in a range slider (0-indexed).
    /// For single-value sliders, defaults to 0.
    #[props(default)]
    pub index: Option<usize>,

    /// Additional attributes to apply to the thumb element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the thumb element
    pub children: Element,
}

/// # SliderThumb
///
/// The thumb component for the [`Slider`] that users can drag to change the slider value. It supports
/// both mouse/touch interaction and keyboard navigation with arrow keys. Arrow keys will move the thumb
/// by the step value by default, or by 10x the step value if the shift key is held down.
///
/// For multi-thumb sliders, set the `index` prop to specify which value this thumb controls.
/// Keyboard navigation respects `min_steps_between_thumbs` to prevent thumbs from overlapping.
///
/// This must be used inside a [`SliderTrack`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::slider::{Slider, SliderRange, SliderThumb, SliderTrack};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Slider {
///             label: "Demo Slider",
///             horizontal: true,
///             SliderTrack {
///                 SliderRange {}
///                 SliderThumb {}
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`SliderThumb`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Present when the slider is disabled.
/// - `data-orientation`: Indicates the orientation of the slider. Values are `horizontal` or `vertical`.
/// - `data-dragging`: Indicates if the thumb is currently being dragged. Values are `true` or `false`.
///
/// It automatically has the percentage based position styles applied based on the current slider value.
#[component]
pub fn SliderThumb(props: SliderThumbProps) -> Element {
    let mut ctx = use_context::<SliderContext>();
    let orientation = if ctx.horizontal {
        "horizontal"
    } else {
        "vertical"
    };

    let thumb_idx = props.index.unwrap_or(0);

    let value = use_memo(move || (ctx.value)().get(thumb_idx).unwrap_or(0.0));

    let percent = ctx.as_percent(value());
    let style = if ctx.horizontal {
        format!("left: {percent}%")
    } else {
        format!("bottom: {percent}%")
    };

    let mut thumb_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    use_effect(move || {
        let thumb_ref = thumb_ref();
        if let Some(thumb) = thumb_ref {
            // Focus the thumb while dragging
            let dragging = ctx.dragging.cloned();
            let is_active = ctx.active_thumb_index.peek().unwrap_or(0) == thumb_idx;
            if !ctx.disabled && dragging && is_active {
                spawn(async move {
                    _ = thumb.set_focus(true).await;
                });
            }
        }
    });

    let aria_label = ctx.label;

    rsx! {
        span {
            role: "slider",
            aria_valuemin: (ctx.min)(),
            aria_valuemax: (ctx.max)(),
            aria_valuenow: value,
            aria_orientation: orientation,
            aria_disabled: if ctx.disabled { Some("true") } else { None },
            aria_label,
            "data-slot": "slider-thumb",
            "data-disabled": if ctx.disabled { "" } else { None::<&str> },
            "data-orientation": orientation,
            "data-dragging": ctx.dragging,
            style,
            tabindex: if ctx.disabled { None::<&str> } else { Some("0") },
            onmounted: move |evt| {
                // Store the mounted data for focus management
                thumb_ref.set(Some(evt.data()));
            },
            onmousedown: move |evt| {
                // Don't focus the thumb. The dragging state will handle focus
                evt.prevent_default();
            },
            ontouchstart: move |evt| {
                // Don't focus the thumb. The dragging state will handle focus
                evt.prevent_default();
            },
            onfocus: move |_| {
                // Set this as the active thumb when focused
                ctx.active_thumb_index.set(Some(thumb_idx));
            },
            onkeydown: move |evt| async move {
                if ctx.disabled {
                    return;
                }

                let key = evt.data().key();
                let mut step = (ctx.step)();
                if evt.data().modifiers().shift() {
                    // If shift is pressed, increase the step size
                    step *= 10.0;
                }

                // Handle keyboard navigation (orientation-aware)
                let new_value = match (key, ctx.horizontal) {
                    (Key::ArrowRight, true) | (Key::ArrowUp, false) => {
                        value() + step
                    }
                    (Key::ArrowLeft, true) | (Key::ArrowDown, false) => {
                        value() - step
                    }
                    _ => return,
                };

                // Clamp, snap, and enforce minimum spacing between thumbs
                let clamped = ctx.clamp_and_snap_for_thumb(new_value, thumb_idx);
                ctx.set_value.call(ctx.value.cloned().with_value_at(thumb_idx, clamped));
            },
            ..props.attributes,
            {props.children}
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
struct SliderContext {
    value: Memo<SliderValue>,
    set_value: Callback<SliderValue>,
    min: ReadSignal<f64>,
    max: ReadSignal<f64>,
    step: ReadSignal<f64>,
    disabled: bool,
    horizontal: bool,
    inverted: bool,
    dragging: ReadSignal<bool>,
    label: ReadSignal<Option<String>>,
    /// Which thumb is currently being interacted with.
    active_thumb_index: Signal<Option<usize>>,
    /// Minimum spacing between thumbs in step units (matching Radix's minStepsBetweenThumbs).
    min_steps_between_thumbs: f64,
}

impl SliderContext {
    fn range(&self) -> [f64; 2] {
        if !self.inverted {
            [(self.min)(), (self.max)()]
        } else {
            [(self.max)(), (self.min)()]
        }
    }

    fn range_size(&self) -> f64 {
        let [range_min, range_max] = self.range();
        range_max - range_min
    }

    fn snap(&self, value: f64) -> f64 {
        let step = (self.step)();
        (value / step).round() * step
    }

    /// Clamp, snap, and enforce minimum spacing from neighbor thumbs.
    /// Matching Radix's `minStepsBetweenThumbs` constraint.
    fn clamp_and_snap_for_thumb(&self, value: f64, thumb_idx: usize) -> f64 {
        let step = (self.step)();
        let min_gap = self.min_steps_between_thumbs * step;
        let current = self.value.cloned();
        let values = current.values();

        let mut low = (self.min)();
        let mut high = (self.max)();

        // Constrain by neighboring thumbs' values + minimum gap
        if thumb_idx > 0 {
            if let Some(prev_val) = values.get(thumb_idx - 1) {
                low = low.max(prev_val + min_gap);
            }
        }
        if thumb_idx + 1 < values.len() {
            if let Some(next_val) = values.get(thumb_idx + 1) {
                high = high.min(next_val - min_gap);
            }
        }

        let clamped = value.clamp(low, high);
        self.snap(clamped)
    }

    fn as_percent(&self, value: f64) -> f64 {
        let min = (self.min)();
        let max = (self.max)();
        ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0)
    }
}
