//! Styled slider matching shadcn/ui.
//!
//! Wraps `dioxus_primitives::slider` with shadcn Tailwind classes.

use dioxus::prelude::*;
use dioxus_core::AttributeValue::Text;
use dioxus_primitives::slider as primitives;
pub use dioxus_primitives::slider::SliderValue;
use tailwind_fuse::*;

/// Push a `class` attribute onto an attribute vec.
fn push_class(attrs: &mut Vec<Attribute>, class: String) {
    attrs.push(Attribute {
        name: "class",
        value: Text(class),
        namespace: None,
        volatile: false,
    });
}

// ---------------------------------------------------------------------------
// Slider
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    pub value: ReadSignal<Option<SliderValue>>,

    #[props(default = SliderValue::Single(0.0))]
    pub default_value: SliderValue,

    #[props(default = 0.0)]
    pub min: ReadSignal<f64>,

    #[props(default = 100.0)]
    pub max: ReadSignal<f64>,

    #[props(default = 1.0)]
    pub step: ReadSignal<f64>,

    #[props(default)]
    pub disabled: bool,

    #[props(default = true)]
    pub horizontal: bool,

    #[props(default)]
    pub inverted: bool,

    #[props(default)]
    pub on_value_change: Callback<SliderValue>,

    pub label: ReadSignal<Option<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn Slider(props: SliderProps) -> Element {
    let class = tw_merge!(
        "relative flex w-full touch-none items-center select-none data-[disabled]:opacity-50 data-[orientation=vertical]:h-full data-[orientation=vertical]:min-h-44 data-[orientation=vertical]:w-auto data-[orientation=vertical]:flex-col",
        props.class,
    );

    let mut attrs = props.attributes;
    push_class(&mut attrs, class);

    rsx! {
        primitives::Slider {
            value: props.value,
            default_value: props.default_value,
            min: props.min,
            max: props.max,
            step: props.step,
            disabled: props.disabled,
            horizontal: props.horizontal,
            inverted: props.inverted,
            on_value_change: props.on_value_change,
            label: props.label,
            attributes: attrs,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// SliderTrack
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SliderTrackProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn SliderTrack(props: SliderTrackProps) -> Element {
    let class = tw_merge!(
        "relative grow overflow-hidden rounded-full bg-muted data-[orientation=horizontal]:h-1.5 data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-1.5",
        props.class,
    );

    let mut attrs = props.attributes;
    push_class(&mut attrs, class);

    rsx! {
        primitives::SliderTrack {
            attributes: attrs,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// SliderRange
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SliderRangeProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SliderRange(props: SliderRangeProps) -> Element {
    let class = tw_merge!(
        "absolute bg-primary data-[orientation=horizontal]:h-full data-[orientation=vertical]:w-full",
        props.class,
    );

    let mut attrs = props.attributes;
    push_class(&mut attrs, class);

    rsx! {
        primitives::SliderRange {
            attributes: attrs,
        }
    }
}

// ---------------------------------------------------------------------------
// SliderThumb
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SliderThumbProps {
    #[props(default)]
    pub index: Option<usize>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SliderThumb(props: SliderThumbProps) -> Element {
    let class = tw_merge!(
        "block size-4 shrink-0 rounded-full border border-primary bg-white shadow-sm ring-ring/50 transition-[color,box-shadow] hover:ring-4 focus-visible:ring-4 focus-visible:outline-hidden disabled:pointer-events-none disabled:opacity-50",
        props.class,
    );

    let mut attrs = props.attributes;
    push_class(&mut attrs, class);

    rsx! {
        primitives::SliderThumb {
            index: props.index,
            attributes: attrs,
        }
    }
}
