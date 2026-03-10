//! SSR snapshot tests for the styled Combobox component.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::combobox::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn combobox_root_renders() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Combobox {
                ComboboxInput {}
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("data-slot=\"combobox\""), "root slot: {html}");
}

#[test]
fn combobox_input_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Combobox {
                ComboboxInput { placeholder: "Search..." }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("w-auto"), "input has w-auto: {html}");
    assert!(html.contains("role=\"combobox\""), "input role: {html}");
}

#[test]
fn combobox_content_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            // Content won't render because combobox is closed in SSR.
            // So we test the list directly.
            Combobox {
                ComboboxList {
                    ComboboxItem { value: "test", "Test" }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("overflow-y-auto"),
        "list has overflow-y-auto: {html}"
    );
}

#[test]
fn combobox_item_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Combobox {
                ComboboxList {
                    ComboboxItem { value: "react", "React" }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("cursor-default"), "item class: {html}");
    assert!(html.contains("rounded-sm"), "item rounded: {html}");
}

#[test]
fn combobox_empty_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Combobox {
                ComboboxList {
                    ComboboxEmpty { "No results." }
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(
        html.contains("text-muted-foreground"),
        "empty has muted: {html}"
    );
}

#[test]
fn combobox_separator_has_shadcn_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Combobox {
                ComboboxList {
                    ComboboxSeparator {}
                }
            }
        }
    }

    let html = render(TestApp);
    assert!(html.contains("bg-border"), "separator: {html}");
    assert!(html.contains("h-px"), "separator height: {html}");
}

#[test]
fn full_styled_combobox_composition() {
    #[component]
    fn TestApp() -> Element {
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

    let html = render(TestApp);
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
