use dioxus::prelude::*;
use dioxus_components::empty::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn empty_root_classes() {
    fn App() -> Element {
        rsx! {
            Empty {
                EmptyHeader {
                    EmptyTitle { "No results" }
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== empty_root_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="empty""#));
    assert!(html.contains("border-dashed"));
    assert!(html.contains("text-balance"));
}

#[test]
fn empty_header_slot() {
    fn App() -> Element {
        rsx! {
            Empty {
                EmptyHeader {
                    EmptyTitle { "Title" }
                    EmptyDescription { "Description" }
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="empty-header""#));
    assert!(html.contains("max-w-sm"));
}

#[test]
fn empty_media_default_variant() {
    fn App() -> Element {
        rsx! {
            Empty {
                EmptyMedia { "icon here" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="empty-icon""#));
    assert!(html.contains("bg-transparent"));
}

#[test]
fn empty_media_icon_variant() {
    fn App() -> Element {
        rsx! {
            Empty {
                EmptyMedia { variant: EmptyMediaVariant::Icon, "icon here" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains("bg-muted"));
    assert!(html.contains("rounded-lg"));
}

#[test]
fn empty_title_classes() {
    fn App() -> Element {
        rsx! {
            Empty {
                EmptyHeader {
                    EmptyTitle { "No results found" }
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="empty-title""#));
    assert!(html.contains("font-medium"));
    assert!(html.contains("tracking-tight"));
}

#[test]
fn empty_description_classes() {
    fn App() -> Element {
        rsx! {
            Empty {
                EmptyHeader {
                    EmptyDescription { "Try adjusting your search." }
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="empty-description""#));
    assert!(html.contains("text-muted-foreground"));
}

#[test]
fn empty_content_classes() {
    fn App() -> Element {
        rsx! {
            Empty {
                EmptyContent { "Actions here" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="empty-content""#));
    assert!(html.contains("max-w-sm"));
}
