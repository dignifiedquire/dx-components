//! Progress primitive — matches `@radix-ui/react-progress`.
//!
//! Displays the completion progress of a task with proper ARIA attributes.

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct ProgressCtx {
    value: ReadSignal<Option<f64>>,
    max: f64,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn get_progress_state(value: Option<f64>, max: f64) -> &'static str {
    match value {
        None => "indeterminate",
        Some(v) if v >= max => "complete",
        Some(_) => "loading",
    }
}

fn default_get_value_label(value: f64, max: f64) -> String {
    format!("{}%", ((value / max) * 100.0).round() as i64)
}

// ---------------------------------------------------------------------------
// Progress
// ---------------------------------------------------------------------------

/// Props for [`Progress`].
#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// The current progress value. `None` for indeterminate.
    #[props(default)]
    pub value: ReadSignal<Option<f64>>,

    /// The maximum value. Defaults to `100`.
    #[props(default = 100.0)]
    pub max: f64,

    /// Custom function to generate the accessible value label.
    /// Receives `(value, max)`, should return a human-readable string.
    #[props(default)]
    pub get_value_label: Option<Callback<(f64, f64), String>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (typically a [`ProgressIndicator`]).
    pub children: Element,
}

/// Displays the completion progress of a task.
///
/// Matches Radix's `Progress`. Renders with `role="progressbar"` and full
/// ARIA value attributes.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::progress::{Progress, ProgressIndicator};
/// rsx! {
///     Progress { value: 50.0, max: 100.0,
///         ProgressIndicator {}
///     }
/// };
/// ```
#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let max = props.max;
    let value = props.value;

    use_context_provider(|| ProgressCtx { value, max });

    let state = use_memo(move || get_progress_state(value(), max));

    let value_label = use_memo(move || {
        value().map(|v| {
            if let Some(cb) = &props.get_value_label {
                cb.call((v, max))
            } else {
                default_get_value_label(v, max)
            }
        })
    });

    rsx! {
        div {
            "data-slot": "progress",
            role: "progressbar",
            "aria-valuemin": "0",
            "aria-valuemax": "{max}",
            "aria-valuenow": value().map(|v| format!("{v}")),
            "aria-valuetext": value_label(),
            "data-state": state,
            "data-value": value().map(|v| format!("{v}")),
            "data-max": "{max}",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ProgressIndicator
// ---------------------------------------------------------------------------

/// Props for [`ProgressIndicator`].
#[derive(Props, Clone, PartialEq)]
pub struct ProgressIndicatorProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// The visual indicator showing progress completion.
///
/// Must be used inside a [`Progress`] component. Inherits `data-state`,
/// `data-value`, and `data-max` from the parent context.
#[component]
pub fn ProgressIndicator(props: ProgressIndicatorProps) -> Element {
    let ctx: ProgressCtx = use_context();
    let value = (ctx.value)();
    let max = ctx.max;
    let state = get_progress_state(value, max);

    rsx! {
        div {
            "data-slot": "progress-indicator",
            "data-state": state,
            "data-value": value.map(|v| format!("{v}")),
            "data-max": "{max}",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}
