#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::menubar::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

#[test]
fn menubar_root_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Menubar {
                "Menu content"
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== menubar_root_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="menubar""#));
    assert!(html.contains(r#"role="menubar""#));
    assert!(html.contains("flex h-9 items-center"));
    assert!(html.contains("rounded-md border bg-background p-1 shadow-xs"));
}

#[test]
fn menubar_trigger_classes() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Menubar {
                MenubarMenu {
                    MenubarTrigger { "File" }
                }
            }
        }
    }

    let html = render(TestApp);
    eprintln!("=== menubar_trigger_classes ===\n{html}\n");

    assert!(html.contains(r#"data-slot="menubar-trigger""#));
    assert!(html.contains("text-sm font-medium"));
    assert!(html.contains("rounded-sm px-2 py-1"));
}

#[test]
fn menubar_consumer_class_merge() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Menubar {
                class: "my-custom-bar",
                "Content"
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains("my-custom-bar"));
}

#[test]
fn menubar_trigger_state_closed() {
    #[component]
    fn TestApp() -> Element {
        rsx! {
            Menubar {
                MenubarMenu {
                    MenubarTrigger { "Edit" }
                }
            }
        }
    }

    let html = render(TestApp);

    assert!(html.contains(r#"data-state="closed""#));
    assert!(html.contains(r#"aria-haspopup="menu""#));
}
