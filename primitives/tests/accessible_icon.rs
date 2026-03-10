use dioxus::prelude::*;

fn render(element: Element) -> String {
    let mut vdom = VirtualDom::new_with_props(move || element.clone(), ());
    vdom.rebuild_in_place();
    dioxus_ssr::render(&vdom)
}

#[test]
fn accessible_icon_renders_wrapper_span() {
    let html = render(rsx! {
        dioxus_primitives::accessible_icon::AccessibleIcon { label: "Close",
            svg { view_box: "0 0 24 24" }
        }
    });

    assert!(html.contains("data-slot=\"accessible-icon\""));
}

#[test]
fn accessible_icon_renders_aria_hidden_on_icon_wrapper() {
    let html = render(rsx! {
        dioxus_primitives::accessible_icon::AccessibleIcon { label: "Close",
            svg { view_box: "0 0 24 24" }
        }
    });

    assert!(html.contains("aria-hidden=\"true\""));
}

#[test]
fn accessible_icon_renders_visually_hidden_label() {
    let html = render(rsx! {
        dioxus_primitives::accessible_icon::AccessibleIcon { label: "Close",
            svg { view_box: "0 0 24 24" }
        }
    });

    // The visually-hidden span should contain the label text
    assert!(html.contains("Close"));
    assert!(html.contains("data-slot=\"visually-hidden\""));
}

#[test]
fn accessible_icon_contains_child_svg() {
    let html = render(rsx! {
        dioxus_primitives::accessible_icon::AccessibleIcon { label: "Menu",
            svg {
                view_box: "0 0 24 24",
                path { d: "M3 12h18" }
            }
        }
    });

    assert!(html.contains("<svg"));
    assert!(html.contains("M3 12h18"));
}
