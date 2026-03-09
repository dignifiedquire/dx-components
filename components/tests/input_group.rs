use dioxus::prelude::*;
use dioxus_components::input_group::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn input_group_role_and_classes() {
    fn App() -> Element {
        rsx! {
            InputGroup {
                InputGroupInput {}
            }
        }
    }

    let html = render(App);
    eprintln!("=== input_group_role_and_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="input-group""#));
    assert!(html.contains(r#"role="group""#));
    assert!(html.contains("group/input-group"));
    assert!(html.contains("rounded-md"));
    assert!(html.contains("border-input"));
}

#[test]
fn input_group_input_slot() {
    fn App() -> Element {
        rsx! {
            InputGroup {
                InputGroupInput { placeholder: "Enter..." }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="input-group-control""#));
    assert!(html.contains("<input"));
    assert!(html.contains("flex-1"));
    assert!(html.contains("border-0"));
    assert!(html.contains("bg-transparent"));
}

#[test]
fn input_group_textarea_slot() {
    fn App() -> Element {
        rsx! {
            InputGroup {
                InputGroupTextarea { placeholder: "Type..." }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="input-group-control""#));
    assert!(html.contains("<textarea"));
    assert!(html.contains("resize-none"));
}

#[test]
fn input_group_addon_inline_start() {
    fn App() -> Element {
        rsx! {
            InputGroup {
                InputGroupAddon { "Icon" }
                InputGroupInput {}
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="input-group-addon""#));
    assert!(html.contains(r#"data-align="inline-start""#));
    assert!(html.contains("order-first"));
}

#[test]
fn input_group_addon_inline_end() {
    fn App() -> Element {
        rsx! {
            InputGroup {
                InputGroupInput {}
                InputGroupAddon { align: InputGroupAddonAlign::InlineEnd, "Icon" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-align="inline-end""#));
    assert!(html.contains("order-last"));
}

#[test]
fn input_group_text_classes() {
    fn App() -> Element {
        rsx! {
            InputGroup {
                InputGroupText { "$" }
                InputGroupInput {}
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="input-group-text""#));
    assert!(html.contains("<span"));
    assert!(html.contains("text-muted-foreground"));
}

#[test]
fn input_group_disabled_state() {
    fn App() -> Element {
        rsx! {
            InputGroup { disabled: true,
                InputGroupInput {}
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-disabled="true""#));
}
