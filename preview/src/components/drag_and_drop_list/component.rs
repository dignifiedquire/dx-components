pub use dioxus_components::drag_and_drop_list::*;

use dioxus::prelude::*;

pub fn example_items() -> Vec<Element> {
    let animals = ["Cat 🐱", "Cow 🐮", "Dog 🐶", "Fox 🦊", "Pig 🐷"];

    animals
        .iter()
        .map(|&text| {
            rsx! {
                {text}
            }
        })
        .collect()
}
