use dioxus::prelude::*;
use dioxus_components::item::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(&dom)
}

#[test]
fn item_group_role_list() {
    fn App() -> Element {
        rsx! {
            ItemGroup {
                Item {
                    ItemContent { ItemTitle { "Title" } }
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== item_group_role_list ===\n{html}\n");

    assert!(html.contains(r#"data-slot="item-group""#));
    assert!(html.contains(r#"role="list""#));
    assert!(html.contains("group/item-group"));
}

#[test]
fn item_default_variant() {
    fn App() -> Element {
        rsx! {
            Item { "Content" }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="item""#));
    assert!(html.contains("bg-transparent"));
    assert!(html.contains("gap-4 p-4"));
}

#[test]
fn item_outline_variant() {
    fn App() -> Element {
        rsx! {
            Item { variant: ItemVariant::Outline, "Content" }
        }
    }

    let html = render(App);

    assert!(html.contains("border-border"));
}

#[test]
fn item_muted_variant() {
    fn App() -> Element {
        rsx! {
            Item { variant: ItemVariant::Muted, "Content" }
        }
    }

    let html = render(App);

    assert!(html.contains("bg-muted/50"));
}

#[test]
fn item_sm_size() {
    fn App() -> Element {
        rsx! {
            Item { size: ItemSize::Sm, "Content" }
        }
    }

    let html = render(App);

    assert!(html.contains("gap-2.5"));
    assert!(html.contains("py-3"));
}

#[test]
fn item_media_icon_variant() {
    fn App() -> Element {
        rsx! {
            Item {
                ItemMedia { variant: ItemMediaVariant::Icon, "I" }
                ItemContent { ItemTitle { "Title" } }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="item-media""#));
    assert!(html.contains("size-8"));
    assert!(html.contains("bg-muted"));
}

#[test]
fn item_media_image_variant() {
    fn App() -> Element {
        rsx! {
            Item {
                ItemMedia { variant: ItemMediaVariant::Image, "img" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains("size-10"));
    assert!(html.contains("overflow-hidden"));
}

#[test]
fn item_content_slot() {
    fn App() -> Element {
        rsx! {
            Item {
                ItemContent {
                    ItemTitle { "Title" }
                    ItemDescription { "Description" }
                }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="item-content""#));
    assert!(html.contains(r#"data-slot="item-title""#));
    assert!(html.contains(r#"data-slot="item-description""#));
    assert!(html.contains("line-clamp-2"));
}

#[test]
fn item_actions_slot() {
    fn App() -> Element {
        rsx! {
            Item {
                ItemActions { "Btn" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="item-actions""#));
}

#[test]
fn item_header_footer() {
    fn App() -> Element {
        rsx! {
            Item {
                ItemHeader { "Header" }
                ItemFooter { "Footer" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="item-header""#));
    assert!(html.contains(r#"data-slot="item-footer""#));
    assert!(html.contains("basis-full"));
}

#[test]
fn item_separator_slot() {
    fn App() -> Element {
        rsx! {
            ItemGroup {
                Item { "A" }
                ItemSeparator {}
                Item { "B" }
            }
        }
    }

    let html = render(App);

    assert!(html.contains(r#"data-slot="item-separator""#));
    assert!(html.contains("bg-border"));
}
