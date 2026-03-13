//! SSR snapshot tests for the select primitive.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::select::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

// ---------------------------------------------------------------------------
// Select root renders no DOM
// ---------------------------------------------------------------------------

#[test]
fn root_renders_no_dom() {
    fn App() -> Element {
        rsx! {
            Select {
                span { "sentinel" }
            }
        }
    }

    let html = render(App);
    assert!(html.contains("sentinel"), "children render: {html}");
    // Root should NOT add a wrapper div with data-state
    assert!(
        !html.contains("data-state"),
        "root should not render DOM: {html}"
    );
}

// ---------------------------------------------------------------------------
// SelectTrigger
// ---------------------------------------------------------------------------

#[test]
fn trigger_attributes() {
    fn App() -> Element {
        rsx! {
            Select {
                SelectTrigger { "Open" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="select-trigger""#),
        "trigger has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="combobox""#),
        "trigger has role=combobox: {html}"
    );
    assert!(
        html.contains(r#"type="button""#),
        "trigger has type=button: {html}"
    );
    assert!(
        html.contains("aria-expanded=false"),
        "trigger has aria-expanded: {html}"
    );
    assert!(
        html.contains(r#"aria-autocomplete="none""#),
        "trigger has aria-autocomplete: {html}"
    );
    assert!(
        html.contains(r#"data-state="closed""#),
        "trigger shows closed: {html}"
    );
    assert!(html.contains("Open"), "trigger has children: {html}");
}

#[test]
fn trigger_disabled() {
    fn App() -> Element {
        rsx! {
            Select {
                disabled: true,
                SelectTrigger { "Open" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-disabled="""#),
        "trigger has data-disabled: {html}"
    );
    assert!(
        html.contains("disabled"),
        "trigger has disabled attribute: {html}"
    );
}

#[test]
fn trigger_has_data_placeholder_when_no_value() {
    fn App() -> Element {
        rsx! {
            Select {
                SelectTrigger { "Open" }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("data-placeholder"),
        "trigger has data-placeholder when no value: {html}"
    );
}

// ---------------------------------------------------------------------------
// SelectValue
// ---------------------------------------------------------------------------

#[test]
fn value_attributes() {
    fn App() -> Element {
        rsx! {
            Select {
                placeholder: "Pick one",
                SelectTrigger {
                    SelectValue {}
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="select-value""#),
        "value has data-slot: {html}"
    );
    assert!(
        html.contains("data-placeholder"),
        "value has data-placeholder when no selection: {html}"
    );
    assert!(html.contains("Pick one"), "value shows placeholder: {html}");
}

// ---------------------------------------------------------------------------
// SelectContent (formerly SelectList)
// ---------------------------------------------------------------------------

#[test]
fn content_hidden_when_closed() {
    fn App() -> Element {
        rsx! {
            Select {
                SelectTrigger { "Open" }
                SelectContent {
                    p { "Hidden" }
                }
            }
        }
    }

    let html = render(App);
    // Content should not render with data-slot when closed
    assert!(
        !html.contains(r#"data-slot="select-content""#),
        "content not rendered when closed: {html}"
    );
}

// ---------------------------------------------------------------------------
// SelectGroup
// ---------------------------------------------------------------------------

#[test]
fn group_attributes() {
    fn App() -> Element {
        rsx! {
            Select {
                SelectTrigger { "Open" }
                SelectContent {
                    SelectGroup {
                        span { "items" }
                    }
                }
            }
        }
    }

    // Group renders inside content which is closed, so it renders children
    // but not the group div (since render is false)
    let html = render(App);
    assert!(
        html.contains("items"),
        "group children still render: {html}"
    );
}

// ---------------------------------------------------------------------------
// SelectSeparator
// ---------------------------------------------------------------------------

#[test]
fn separator_attributes() {
    fn App() -> Element {
        rsx! {
            Select {
                SelectSeparator {}
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="select-separator""#),
        "separator has data-slot: {html}"
    );
    assert!(
        html.contains(r#"role="separator""#),
        "separator has role=separator: {html}"
    );
    assert!(
        html.contains(r#"aria-hidden="true""#),
        "separator has aria-hidden: {html}"
    );
}

// ---------------------------------------------------------------------------
// SelectList backward-compat alias
// ---------------------------------------------------------------------------

#[test]
fn select_list_alias_works() {
    fn App() -> Element {
        rsx! {
            Select {
                SelectTrigger { "Open" }
                SelectList {
                    p { "alias" }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("alias"),
        "SelectList alias renders children: {html}"
    );
}

// ---------------------------------------------------------------------------
// SelectOption backward-compat alias
// ---------------------------------------------------------------------------

#[test]
fn select_option_alias_works() {
    // Verify the alias compiles and renders without panic
    fn App() -> Element {
        rsx! {
            Select {
                SelectTrigger { "Open" }
                SelectContent {
                    SelectOption {
                        value: "test",
                        "Test Item"
                    }
                }
            }
        }
    }

    let html = render(App);
    // Content is closed in SSR, so items won't render their DOM,
    // but the alias should compile and not panic
    assert!(
        html.contains("Open"),
        "SelectOption alias does not panic: {html}"
    );
}

// ---------------------------------------------------------------------------
// SelectGroupLabel backward-compat alias
// ---------------------------------------------------------------------------

#[test]
fn select_group_label_alias_works() {
    // Verify the alias compiles and renders without panic
    fn App() -> Element {
        rsx! {
            Select {
                SelectTrigger { "Open" }
                SelectContent {
                    SelectGroup {
                        SelectGroupLabel { "Label" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains("Open"),
        "SelectGroupLabel alias does not panic: {html}"
    );
}

// ---------------------------------------------------------------------------
// SelectItemText
// ---------------------------------------------------------------------------

#[test]
fn select_item_text_compiles() {
    // Verify SelectItemText compiles and renders without panic inside SelectItem
    fn App() -> Element {
        rsx! {
            Select {
                SelectTrigger { SelectValue {} }
                SelectContent {
                    SelectItem { value: "apple",
                        SelectItemText { text: "Apple", "🍎 Apple" }
                        SelectItemIndicator { "✓" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="select-value""#),
        "SelectItemText does not panic: {html}"
    );
}

#[test]
fn select_item_text_without_text_prop() {
    // SelectItemText without `text` prop should still compile and render
    fn App() -> Element {
        rsx! {
            Select {
                SelectTrigger { SelectValue {} }
                SelectContent {
                    SelectItem { value: "apple",
                        SelectItemText { "Apple" }
                    }
                }
            }
        }
    }

    let html = render(App);
    assert!(
        html.contains(r#"data-slot="select-value""#),
        "SelectItemText without text prop does not panic: {html}"
    );
}
