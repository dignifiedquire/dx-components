use dioxus::prelude::*;
use dioxus_primitives::top_layer::{use_top_layer, TopLayerKind};
use std::rc::Rc;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            "data-testid": "top-layer-demos",
            class: "flex flex-col gap-8 p-4",

            // ----------------------------------------------------------
            // Test 1: Base — popover="auto" opens/closes from button
            // ----------------------------------------------------------
            BasePopover {}

            // ----------------------------------------------------------
            // Test 2: Escapes overflow:hidden ancestor
            // ----------------------------------------------------------
            OverflowEscape {}

            // ----------------------------------------------------------
            // Test 3: Escapes transform ancestor (containing block)
            // ----------------------------------------------------------
            TransformEscape {}

            // ----------------------------------------------------------
            // Test 4: Escapes z-index isolation stacking context
            // ----------------------------------------------------------
            StackingEscape {}

            // ----------------------------------------------------------
            // Test 5: Manual popover (no light-dismiss)
            // ----------------------------------------------------------
            ManualPopover {}

            // ----------------------------------------------------------
            // Test 6: Dialog modal — show_modal/close
            // ----------------------------------------------------------
            DialogDemo {}
        }
    }
}

#[component]
fn BasePopover() -> Element {
    let mut open = use_signal(|| false);
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    use_top_layer(mounted.into(), open, TopLayerKind::PopoverAuto);

    rsx! {
        section {
            "data-testid": "tl-base",
            h3 { "Base popover (auto)" }
            button {
                "data-testid": "tl-base-toggle",
                r#type: "button",
                onclick: move |_| open.toggle(),
                if open() { "Hide" } else { "Show" }
            }
            div {
                "data-testid": "tl-base-content",
                popover: "auto",
                onmounted: move |evt| mounted.set(Some(evt.data())),
                style: "padding: 16px; border: 2px solid blue; background: #eef;",
                "Base popover content"
            }
        }
    }
}

#[component]
fn OverflowEscape() -> Element {
    let mut open = use_signal(|| false);
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    use_top_layer(mounted.into(), open, TopLayerKind::PopoverAuto);

    rsx! {
        section {
            "data-testid": "tl-overflow",
            h3 { "Escapes overflow:hidden" }
            div {
                "data-testid": "tl-overflow-clip",
                style: "max-width: 200px; max-height: 60px; overflow: hidden; border: 1px solid gray; padding: 8px;",
                button {
                    "data-testid": "tl-overflow-toggle",
                    r#type: "button",
                    onclick: move |_| open.toggle(),
                    "Toggle"
                }
                div {
                    "data-testid": "tl-overflow-content",
                    popover: "auto",
                    onmounted: move |evt| mounted.set(Some(evt.data())),
                    style: "padding: 24px; border: 2px solid green; background: #efe; width: 320px; height: 120px;",
                    "I escape overflow:hidden"
                }
            }
        }
    }
}

#[component]
fn TransformEscape() -> Element {
    let mut open = use_signal(|| false);
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    use_top_layer(mounted.into(), open, TopLayerKind::PopoverAuto);

    rsx! {
        section {
            "data-testid": "tl-transform",
            h3 { "Escapes transform containing block" }
            div {
                "data-testid": "tl-transform-clip",
                style: "transform: translateZ(0); border: 1px solid gray; padding: 8px;",
                button {
                    "data-testid": "tl-transform-toggle",
                    r#type: "button",
                    onclick: move |_| open.toggle(),
                    "Toggle"
                }
                div {
                    "data-testid": "tl-transform-content",
                    popover: "auto",
                    onmounted: move |evt| mounted.set(Some(evt.data())),
                    style: "padding: 16px; border: 2px solid orange; background: #fed;",
                    "I escape transform"
                }
            }
        }
    }
}

#[component]
fn StackingEscape() -> Element {
    let mut open = use_signal(|| false);
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    use_top_layer(mounted.into(), open, TopLayerKind::PopoverAuto);

    rsx! {
        section {
            "data-testid": "tl-stacking",
            h3 { "Escapes z-index isolation" }
            div {
                "data-testid": "tl-stacking-clip",
                style: "z-index: 0; isolation: isolate; border: 1px solid gray; padding: 8px;",
                button {
                    "data-testid": "tl-stacking-toggle",
                    r#type: "button",
                    onclick: move |_| open.toggle(),
                    "Toggle"
                }
                div {
                    "data-testid": "tl-stacking-content",
                    popover: "auto",
                    onmounted: move |evt| mounted.set(Some(evt.data())),
                    style: "padding: 16px; border: 2px solid purple; background: #fef;",
                    "I escape isolation"
                }
            }
        }
    }
}

#[component]
fn ManualPopover() -> Element {
    let mut open = use_signal(|| false);
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    use_top_layer(mounted.into(), open, TopLayerKind::PopoverManual);

    rsx! {
        section {
            "data-testid": "tl-manual",
            h3 { "Manual popover (no light-dismiss)" }
            button {
                "data-testid": "tl-manual-toggle",
                r#type: "button",
                onclick: move |_| open.toggle(),
                if open() { "Hide" } else { "Show" }
            }
            div {
                "data-testid": "tl-manual-content",
                popover: "manual",
                onmounted: move |evt| mounted.set(Some(evt.data())),
                style: "padding: 16px; border: 2px solid teal; background: #eff;",
                "Manual popover — click outside does NOT close me"
            }
        }
    }
}

#[component]
fn DialogDemo() -> Element {
    let mut open = use_signal(|| false);
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    use_top_layer(mounted.into(), open, TopLayerKind::DialogModal);

    rsx! {
        section {
            "data-testid": "tl-dialog",
            h3 { "Dialog modal (show_modal)" }
            button {
                "data-testid": "tl-dialog-toggle",
                r#type: "button",
                onclick: move |_| open.toggle(),
                "Open dialog"
            }
            dialog {
                "data-testid": "tl-dialog-content",
                onmounted: move |evt| mounted.set(Some(evt.data())),
                style: "padding: 16px; border: 2px solid red;",
                p { "I am a modal dialog with a native backdrop." }
                button {
                    "data-testid": "tl-dialog-close",
                    r#type: "button",
                    onclick: move |_| open.set(false),
                    "Close"
                }
            }
        }
    }
}
