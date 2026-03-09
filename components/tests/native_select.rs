use dioxus::prelude::*;
use dioxus_components::native_select::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn native_select_wrapper_and_slots() {
    fn App() -> Element {
        rsx! {
            NativeSelect {
                NativeSelectOption { value: "a", "Alpha" }
                NativeSelectOption { value: "b", "Beta" }
            }
        }
    }

    let html = render(App);
    eprintln!("=== native_select_wrapper_and_slots ===\n{html}\n");

    assert!(html.contains(r#"data-slot="native-select-wrapper""#));
    assert!(html.contains(r#"data-slot="native-select""#));
    assert!(html.contains("<select"));
    assert!(html.contains("appearance-none"));
    assert!(html.contains("rounded-md"));
}

#[test]
fn native_select_has_chevron_icon() {
    fn App() -> Element {
        rsx! {
            NativeSelect {
                NativeSelectOption { value: "x", "X" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains("<svg"));
    assert!(html.contains("pointer-events-none"));
}

#[test]
fn native_select_option_slot() {
    fn App() -> Element {
        rsx! {
            NativeSelect {
                NativeSelectOption { value: "v1", "Value 1" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="native-select-option""#));
    assert!(html.contains("<option"));
}

#[test]
fn native_select_optgroup_slot() {
    fn App() -> Element {
        rsx! {
            NativeSelect {
                NativeSelectOptGroup { label: "Group",
                    NativeSelectOption { value: "a", "A" }
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="native-select-optgroup""#));
    assert!(html.contains("<optgroup"));
}

#[test]
fn native_select_sm_size() {
    fn App() -> Element {
        rsx! {
            NativeSelect { size: NativeSelectSize::Sm,
                NativeSelectOption { value: "x", "X" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-size="sm""#));
}
