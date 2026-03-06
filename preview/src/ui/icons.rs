use dioxus::prelude::*;

/// lucide plus icon
#[component]
pub(crate) fn PlusIcon() -> Element {
    rsx! {
        svg {
            height: "2rem",
            view_box: "0 0 24 24",
            width: "2rem",
            xmlns: "http://www.w3.org/2000/svg",
            "aria-label": "Add",
            path {
                d: "M5 12h14m-7-7v14",
                fill: "none",
                stroke: "currentColor",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
            }
        }
    }
}

/// lucide search icon
#[component]
pub(crate) fn SearchIcon() -> Element {
    rsx! {
        svg {
            height: "2rem",
            view_box: "0 0 24 24",
            width: "2rem",
            xmlns: "http://www.w3.org/2000/svg",
            "aria-label": "Search",
            g {
                fill: "none",
                stroke: "currentColor",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                path { d: "m21 21l-4.34-4.34" }
                circle { cx: "11", cy: "11", r: "8" }
            }
        }
    }
}

/// lucide edit icon
#[component]
pub(crate) fn EditIcon() -> Element {
    rsx! {
        svg {
            height: "2rem",
            view_box: "0 0 24 24",
            width: "2rem",
            xmlns: "http://www.w3.org/2000/svg",
            "aria-label": "Edit",
            g {
                fill: "none",
                stroke: "currentColor",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                path { d: "M12 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" }
                path { d: "M18.375 2.625a1 1 0 0 1 3 3l-9.013 9.014a2 2 0 0 1-.853.505l-2.873.84a.5.5 0 0 1-.62-.62l.84-2.873a2 2 0 0 1 .506-.852z" }
            }
        }
    }
}

#[component]
pub(crate) fn GotoIcon(mut props: crate::dioxus_router::LinkProps) -> Element {
    props.children = rsx! {
        svg {
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M5 21q-.825 0-1.412-.587T3 19V5q0-.825.588-1.412T5 3h7v2H5v14h14v-7h2v7q0 .825-.587 1.413T19 21zm4.7-5.3l-1.4-1.4L17.6 5H14V3h7v7h-2V6.4z",
                fill: "var(--secondary-color-4)",
            }
        }
    };
    Link(props)
}
