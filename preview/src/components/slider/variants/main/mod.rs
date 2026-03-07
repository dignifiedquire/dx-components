use super::super::component::{Slider, SliderRange, SliderThumb, SliderTrack, SliderValue};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut current_value = use_signal(|| 50.0);

    rsx! {
        div { class: "mb-4 text-sm font-bold", "{current_value:.0}%" }

        Slider {
            label: "Demo Slider",
            horizontal: true,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            default_value: SliderValue::Single(50.0),
            on_value_change: move |value: SliderValue| {
                let SliderValue::Single(v) = value;
                current_value.set(v);
            },
            SliderTrack {
                SliderRange {}
                SliderThumb {}
            }
        }
    }
}
