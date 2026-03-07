use dioxus::prelude::*;
use dioxus_primitives::slider as slider;

pub use dioxus_primitives::slider::{
    SliderProps, SliderRangeProps, SliderThumbProps, SliderTrackProps, SliderValue,
};

/// Styled Slider root.
#[component]
pub fn Slider(props: SliderProps) -> Element {
    rsx! {
        slider::Slider {
            class: "relative flex w-full touch-none items-center select-none data-[disabled]:opacity-50 data-[orientation=horizontal]:flex-row data-[orientation=vertical]:flex-col",
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
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// Styled SliderTrack.
#[component]
pub fn SliderTrack(props: SliderTrackProps) -> Element {
    rsx! {
        slider::SliderTrack {
            class: "relative grow overflow-hidden rounded-full bg-muted data-[orientation=horizontal]:h-1.5 data-[orientation=horizontal]:w-full data-[orientation=vertical]:w-1.5 data-[orientation=vertical]:h-full",
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// Styled SliderRange.
#[component]
pub fn SliderRange(props: SliderRangeProps) -> Element {
    rsx! {
        slider::SliderRange {
            class: "absolute bg-primary data-[orientation=horizontal]:h-full data-[orientation=vertical]:w-full",
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// Styled SliderThumb.
#[component]
pub fn SliderThumb(props: SliderThumbProps) -> Element {
    rsx! {
        slider::SliderThumb {
            class: "block size-4 shrink-0 rounded-full border border-primary bg-background shadow-sm ring-ring/50 transition-[color,box-shadow] hover:ring-4 focus-visible:ring-4 focus-visible:outline-hidden data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}
