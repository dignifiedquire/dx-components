use dioxus::prelude::*;
use dioxus_components::field::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn fieldset_slot_and_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            FieldSet {
                Field { "content" }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== fieldset_slot_and_classes ===\n{html}\n");

    assert!(html.contains("<fieldset"));
    assert!(html.contains(r#"data-slot="field-set""#));
    assert!(html.contains("flex flex-col gap-6"));
}

#[test]
fn field_legend_variants() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            FieldSet {
                FieldLegend { "Legend" }
                FieldLegend { variant: FieldLegendVariant::Label, "Label" }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== field_legend_variants ===\n{html}\n");

    assert!(html.contains("<legend"));
    assert!(html.contains(r#"data-slot="field-legend""#));
    assert!(html.contains(r#"data-variant="legend""#));
    assert!(html.contains(r#"data-variant="label""#));
}

#[test]
fn field_group_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            FieldGroup {
                Field { "content" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="field-group""#));
    assert!(html.contains("group/field-group"));
    assert!(html.contains("gap-7"));
}

#[test]
fn field_vertical_default() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Field {
                FieldLabel { "Name" }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== field_vertical_default ===\n{html}\n");

    assert!(html.contains(r#"data-slot="field""#));
    assert!(html.contains("group/field"));
    assert!(html.contains("flex-col"));
}

#[test]
fn field_horizontal_orientation() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Field { orientation: FieldOrientation::Horizontal,
                FieldLabel { "Name" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains("flex-row"));
    assert!(html.contains("items-center"));
}

#[test]
fn field_invalid_state() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Field { invalid: true,
                FieldLabel { "Name" }
                FieldError { "Required" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-invalid="true""#));
}

#[test]
fn field_disabled_state() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Field { disabled: true,
                FieldLabel { "Name" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-disabled="true""#));
}

#[test]
fn field_content_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Field {
                FieldContent {
                    FieldDescription { "Enter your name" }
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="field-content""#));
    assert!(html.contains("group/field-content"));
}

#[test]
fn field_label_is_label_element() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Field {
                FieldLabel { html_for: "name", "Name" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains("<label"));
    assert!(html.contains(r#"data-slot="field-label""#));
    assert!(html.contains(r#"for="name""#));
}

#[test]
fn field_description_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Field {
                FieldDescription { "Help text" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="field-description""#));
    assert!(html.contains("text-muted-foreground"));
}

#[test]
fn field_error_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Field { invalid: true,
                FieldError { "This field is required" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="field-error""#));
    assert!(html.contains("text-destructive"));
}

#[test]
fn field_separator_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            FieldGroup {
                Field { "A" }
                FieldSeparator {}
                Field { "B" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="field-separator""#));
}

#[test]
fn field_separator_with_text() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            FieldGroup {
                Field { "A" }
                FieldSeparator { text: "or" }
                Field { "B" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="field-separator-content""#));
    assert!(html.contains("or"));
}

#[test]
fn field_title_slot() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Field {
                FieldTitle { "Section Title" }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-slot="field-label""#));
    assert!(html.contains("font-medium"));
}
