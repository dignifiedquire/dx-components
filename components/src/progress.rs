//! Styled progress bar matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::progress` primitives with
//! Tailwind classes — matching the shadcn/ui progress component 1:1.
//!
//! Unlike the primitive, the styled [`Progress`] composes the indicator
//! internally with the transform style — the consumer does not need to
//! render the indicator manually.

use dioxus::prelude::*;
use dioxus_primitives::progress as primitives;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Progress (styled — composes indicator internally)
// ---------------------------------------------------------------------------

/// The props for the styled [`Progress`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// The current progress value (0–100). `None` for indeterminate.
    #[props(default)]
    pub value: ReadSignal<Option<f64>>,

    /// The maximum value. Defaults to `100`.
    #[props(default = 100.0)]
    pub max: f64,

    /// Custom function to generate the accessible value label.
    #[props(default)]
    pub get_value_label: Option<Callback<(f64, f64), String>>,

    /// Additional Tailwind classes to apply to the root.
    #[props(default)]
    pub class: Option<String>,

    /// Additional Tailwind classes to apply to the indicator.
    #[props(default)]
    pub indicator_class: Option<String>,

    /// Attributes to extend the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled Progress — matches shadcn exactly.
///
/// Composes the primitive progress root + indicator internally.
#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let class = tw_merge!(
        "relative h-2 w-full overflow-hidden rounded-full bg-primary/20",
        props.class,
    );

    let indicator_class = tw_merge!(
        "h-full w-full flex-1 bg-primary transition-all",
        props.indicator_class,
    );

    let max = props.max;
    let value = props.value;
    let translate_x = use_memo(move || {
        let pct = value().unwrap_or(0.0);
        let clamped = (pct / max).clamp(0.0, 1.0) * 100.0;
        format!("transform: translateX(-{}%);", 100.0 - clamped)
    });

    rsx! {
        primitives::Progress {
            value: props.value,
            max: props.max,
            get_value_label: props.get_value_label,
            class: class,
            attributes: props.attributes,

            primitives::ProgressIndicator {
                class: indicator_class,
                style: translate_x,
            }
        }
    }
}
