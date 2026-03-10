//! SSR snapshot tests for the styled DragAndDropList component.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::drag_and_drop_list::*;

/// Render a component to an HTML string via SSR, with normalized IDs.
fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

// ---------------------------------------------------------------------------
// DragAndDropList renders with data-slot="drag-and-drop-list"
// ---------------------------------------------------------------------------

#[test]
fn drag_and_drop_list_renders() {
    #[component]
    fn TestApp() -> Element {
        let items = vec![rsx! { "Item A" }, rsx! { "Item B" }, rsx! { "Item C" }];
        rsx! {
            DragAndDropList { items }
        }
    }

    let html = render(TestApp);
    eprintln!("=== drag_and_drop_list_renders ===\n{html}\n");

    assert!(
        html.contains(r#"data-slot="drag-and-drop-list""#),
        "DragAndDropList root should have data-slot=\"drag-and-drop-list\""
    );
    assert!(
        html.contains(r#"aria-roledescription="sortable list""#),
        "list should have aria-roledescription=\"sortable list\""
    );
    assert!(
        html.contains(r#"aria-label="Sortable list""#),
        "list should have default aria-label=\"Sortable list\""
    );
}

// ---------------------------------------------------------------------------
// DragAndDropItem renders with data-slot="drag-and-drop-list-item"
// ---------------------------------------------------------------------------

#[test]
fn drag_and_drop_item_renders() {
    #[component]
    fn TestApp() -> Element {
        let items = vec![rsx! { "First" }, rsx! { "Second" }];
        rsx! {
            DragAndDropList { items }
        }
    }

    let html = render(TestApp);
    eprintln!("=== drag_and_drop_item_renders ===\n{html}\n");

    assert!(
        html.contains(r#"data-slot="drag-and-drop-list-item""#),
        "List items should have data-slot=\"drag-and-drop-list-item\""
    );
    assert!(
        html.contains(r#"aria-roledescription="sortable item""#),
        "List items should have aria-roledescription=\"sortable item\""
    );
    assert!(
        html.contains(r#"draggable="true""#),
        "List items should be draggable"
    );
    assert!(
        html.contains(r#"data-slot="drag-and-drop-list-item-content""#),
        "Item content wrapper should have data-slot=\"drag-and-drop-list-item-content\""
    );
    assert!(
        html.contains("First"),
        "First item content should be rendered"
    );
    assert!(
        html.contains("Second"),
        "Second item content should be rendered"
    );
}

// ---------------------------------------------------------------------------
// Class merge on DragAndDropList root
// ---------------------------------------------------------------------------

#[test]
fn drag_and_drop_list_class_merge() {
    #[component]
    fn TestApp() -> Element {
        let items = vec![rsx! { "Item 1" }];
        rsx! {
            DragAndDropList {
                class: "my-custom-dnd-class",
                items,
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== drag_and_drop_list_class_merge ===\n{html}\n");

    assert!(
        html.contains("my-custom-dnd-class"),
        "Custom class should be present on DragAndDropList root"
    );
    assert!(
        html.contains(r#"data-slot="drag-and-drop-list""#),
        "data-slot should still be present after class merge"
    );
}
