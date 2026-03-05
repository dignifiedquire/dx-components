use dioxus_primitives::drag_and_drop_list::*;
use super::super::component::example_items;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let items = example_items();
    rsx! {
        DragAndDropList { items }
    }
}
