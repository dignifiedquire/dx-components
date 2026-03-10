use dioxus::prelude::*;
use dioxus_components::button_group::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn button_group_role_and_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ButtonGroup { "Buttons" }
        }
    }

    let html = render(TestApp);
    eprintln!("=== button_group_role_and_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="button-group""#));
    assert!(html.contains(r#"role="group""#));
    assert!(html.contains("flex w-fit items-stretch"));
}

#[test]
fn button_group_horizontal_default() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ButtonGroup { "Buttons" }
        }
    }

    let html = render(TestApp);

    // Horizontal orientation classes
    assert!(html.contains("rounded-l-none"));
    assert!(html.contains("rounded-r-none"));
}

#[test]
fn button_group_vertical() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ButtonGroup { orientation: Orientation::Vertical, "Buttons" }
        }
    }

    let html = render(TestApp);

    assert!(html.contains("flex-col"));
    assert!(html.contains("rounded-t-none"));
    assert!(html.contains("rounded-b-none"));
}

#[test]
fn button_group_text_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ButtonGroup {
                ButtonGroupText { "Label" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="button-group-text""#));
    assert!(html.contains("bg-muted"));
    assert!(html.contains("font-medium"));
}

#[test]
fn button_group_separator_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            ButtonGroup {
                ButtonGroupSeparator {}
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="button-group-separator""#));
    assert!(html.contains("bg-input"));
    assert!(html.contains("self-stretch"));
}
