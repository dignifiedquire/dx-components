//! SSR snapshot tests for the Combobox primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::combobox::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn root_renders_container() {
    fn App() -> Element {
        rsx! {
            Combobox {
                ComboboxInput {}
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"combobox\""),
        "root has data-slot: {html}"
    );
    assert!(
        html.contains("data-state=\"closed\""),
        "initially closed: {html}"
    );
}

#[test]
fn input_renders_with_role_combobox() {
    fn App() -> Element {
        rsx! {
            Combobox {
                ComboboxInput { placeholder: "Search..." }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("role=\"combobox\""),
        "input has combobox role: {html}"
    );
    assert!(
        html.contains("data-slot=\"combobox-input\""),
        "input has data-slot: {html}"
    );
    assert!(
        html.contains("placeholder=\"Search...\""),
        "has placeholder: {html}"
    );
    assert!(
        html.contains("aria-expanded=false"),
        "aria-expanded false: {html}"
    );
    assert!(
        html.contains("aria-autocomplete=\"list\""),
        "has aria-autocomplete: {html}"
    );
}

#[test]
fn content_hidden_when_closed() {
    fn App() -> Element {
        rsx! {
            Combobox {
                ComboboxInput {}
                ComboboxContent {
                    ComboboxList {
                        ComboboxItem { value: "test", "Test" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        !html.contains("data-slot=\"combobox-content\""),
        "content not rendered when closed: {html}"
    );
}

#[test]
fn list_renders_with_role_listbox() {
    fn App() -> Element {
        rsx! {
            Combobox {
                // Render list directly (without content wrapper) to test in SSR
                ComboboxList {
                    ComboboxItem { value: "test", "Test" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("role=\"listbox\""),
        "list has listbox role: {html}"
    );
    assert!(
        html.contains("data-slot=\"combobox-list\""),
        "list has data-slot: {html}"
    );
}

#[test]
fn item_renders_with_role_option() {
    fn App() -> Element {
        rsx! {
            Combobox {
                ComboboxList {
                    ComboboxItem { value: "react", "React" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("role=\"option\""),
        "item has option role: {html}"
    );
    assert!(
        html.contains("data-slot=\"combobox-item\""),
        "item has data-slot: {html}"
    );
    assert!(
        html.contains("data-value=\"react\""),
        "item has data-value: {html}"
    );
    assert!(
        html.contains("aria-selected=false"),
        "item not selected: {html}"
    );
}

#[test]
fn item_shows_selected_state() {
    fn App() -> Element {
        rsx! {
            Combobox { value: "react",
                ComboboxList {
                    ComboboxItem { value: "react", "React" }
                    ComboboxItem { value: "vue", "Vue" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("aria-selected=true"),
        "selected item has aria-selected: {html}"
    );
    assert!(
        html.contains("aria-selected=false"),
        "non-selected item: {html}"
    );
}

#[test]
fn empty_renders() {
    fn App() -> Element {
        rsx! {
            Combobox {
                ComboboxList {
                    ComboboxEmpty { "No results found." }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"combobox-empty\""),
        "empty has data-slot: {html}"
    );
    assert!(html.contains("No results found."), "empty text: {html}");
}

#[test]
fn group_renders_with_heading() {
    fn App() -> Element {
        rsx! {
            Combobox {
                ComboboxList {
                    ComboboxGroup { heading: "Frameworks",
                        ComboboxItem { value: "react", "React" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"combobox-group\""),
        "group has data-slot: {html}"
    );
    assert!(html.contains("role=\"group\""), "group has role: {html}");
    assert!(
        html.contains("data-slot=\"combobox-group-heading\""),
        "heading has data-slot: {html}"
    );
    assert!(html.contains("Frameworks"), "heading text: {html}");
}

#[test]
fn separator_renders() {
    fn App() -> Element {
        rsx! {
            Combobox {
                ComboboxList {
                    ComboboxItem { value: "a", "A" }
                    ComboboxSeparator {}
                    ComboboxItem { value: "b", "B" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-slot=\"combobox-separator\""),
        "separator has data-slot: {html}"
    );
    assert!(
        html.contains("role=\"separator\""),
        "separator has role: {html}"
    );
}

#[test]
fn disabled_state() {
    fn App() -> Element {
        rsx! {
            Combobox { disabled: true,
                ComboboxInput {}
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-disabled=\"true\""),
        "root has disabled: {html}"
    );
}

#[test]
fn full_composition() {
    fn App() -> Element {
        rsx! {
            Combobox { value: "react",
                ComboboxInput { placeholder: "Select..." }
                ComboboxList {
                    ComboboxGroup { heading: "Frameworks",
                        ComboboxItem { value: "react", "React" }
                        ComboboxItem { value: "vue", "Vue" }
                    }
                    ComboboxSeparator {}
                    ComboboxEmpty { "Nothing found." }
                }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("data-slot=\"combobox\""), "root: {html}");
    assert!(
        html.contains("data-slot=\"combobox-input\""),
        "input: {html}"
    );
    assert!(html.contains("data-slot=\"combobox-list\""), "list: {html}");
    assert!(html.contains("data-slot=\"combobox-item\""), "item: {html}");
    assert!(
        html.contains("data-slot=\"combobox-group\""),
        "group: {html}"
    );
    assert!(
        html.contains("data-slot=\"combobox-separator\""),
        "separator: {html}"
    );
    assert!(
        html.contains("data-slot=\"combobox-empty\""),
        "empty: {html}"
    );
}
