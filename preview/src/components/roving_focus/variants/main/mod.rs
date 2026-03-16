use dioxus::prelude::*;
use dioxus_primitives::direction::Orientation;
use dioxus_primitives::roving_focus::{RovingFocusGroup, RovingFocusGroupItem, RovingFocusSlotProps};

// ---------------------------------------------------------------------------
// ButtonGroup — wraps RovingFocusGroup with value tracking
// Upstream: ButtonGroup + ButtonGroupContext
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
struct ButtonGroupCtx {
    value: Signal<Option<String>>,
}

#[derive(Props, Clone, PartialEq)]
struct ButtonGroupProps {
    #[props(default)]
    orientation: Option<Orientation>,
    #[props(default)]
    r#loop: bool,
    #[props(default)]
    default_value: Option<String>,
    children: Element,
}

#[component]
fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let value = use_signal(|| props.default_value.clone());
    use_context_provider(|| ButtonGroupCtx { value });

    let flex_dir = match props.orientation {
        Some(Orientation::Vertical) => "column",
        _ => "row",
    };
    let orientation = use_signal(|| props.orientation);
    let looping = use_signal(|| props.r#loop);

    rsx! {
        RovingFocusGroup {
            orientation,
            r#loop: looping,
            style: "display: inline-flex; flex-direction: {flex_dir}; gap: 10px;",
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// Button — wraps RovingFocusGroupItem with selection styling
// Upstream: Button using RovingFocus.Item asChild
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
struct ButtonProps {
    value: String,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    style: Option<String>,
    children: Element,
}

#[component]
fn Button(props: ButtonProps) -> Element {
    let mut ctx: ButtonGroupCtx = use_context();
    let value = props.value.clone();
    let is_selected = ctx.value.read().as_deref() == Some(value.as_str());

    let click_value = props.value.clone();
    let disabled = props.disabled;

    let extra_style = props.style.clone().unwrap_or_default();
    let children = props.children.clone();

    rsx! {
        RovingFocusGroupItem {
            active: is_selected,
            focusable: !props.disabled,
            r#as: move |slot: RovingFocusSlotProps| {
                let bg = if is_selected { "black" } else { "transparent" };
                let fg = if is_selected { "white" } else { "inherit" };
                let border = if is_selected { "black" } else { "#ccc" };
                let opacity = if disabled { "0.5" } else { "1" };
                let style = format!(
                    "border: 1px solid {border}; padding: 5px 10px; border-radius: 5px; \
                     background: {bg}; color: {fg}; opacity: {opacity}; {extra_style}"
                );
                let cv = click_value.clone();
                let children = children.clone();
                rsx! {
                    button {
                        onkeydown: move |e| slot.on_keydown.call(e),
                        onfocus: move |e| {
                            slot.on_focus.call(e);
                            if !disabled {
                                ctx.value.set(Some(cv.clone()));
                            }
                        },
                        onmousedown: move |e| slot.on_mousedown.call(e),
                        onmounted: move |e| slot.on_mounted.call(e),
                        style: "{style}",
                        disabled: disabled,
                        "data-value": "{click_value}",
                        ..slot.attributes,
                        {children}
                    }
                }
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Demo
// ---------------------------------------------------------------------------

#[component]
pub fn Demo() -> Element {
    let mut has_extra = use_signal(|| false);
    let mut one_disabled = use_signal(|| false);

    rsx! {
        div {
            "data-testid": "roving-focus-demos",
            class: "flex flex-col gap-8 p-4",

            // ---------------------------------------------------------
            // Test 1: Basic — horizontal, no looping
            // Upstream: "Basic" story (horizontal orientation + no looping)
            // ---------------------------------------------------------
            section {
                "data-testid": "basic-horizontal",
                h3 { "Horizontal + No Looping" }
                ButtonGroup { orientation: Orientation::Horizontal, default_value: "two",
                    Button { value: "one", "One" }
                    Button { value: "two", "Two" }
                    Button { value: "three", disabled: true, "Three" }
                    Button { value: "four", "Four" }
                }
            }

            // ---------------------------------------------------------
            // Test 2: Horizontal + looping
            // Upstream: "Basic" story (horizontal orientation + looping)
            // ---------------------------------------------------------
            section {
                "data-testid": "basic-horizontal-loop",
                h3 { "Horizontal + Looping" }
                ButtonGroup { orientation: Orientation::Horizontal, r#loop: true,
                    Button { value: "one", "One" }
                    Button { value: "two", "Two" }
                    Button { value: "three", disabled: true, "Three" }
                    Button { value: "four", "Four" }
                }
            }

            // ---------------------------------------------------------
            // Test 3: Vertical + no looping
            // Upstream: "Basic" story (vertical orientation + no looping)
            // ---------------------------------------------------------
            section {
                "data-testid": "basic-vertical",
                h3 { "Vertical + No Looping" }
                ButtonGroup { orientation: Orientation::Vertical,
                    Button { value: "one", "One" }
                    Button { value: "two", "Two" }
                    Button { value: "three", disabled: true, "Three" }
                    Button { value: "four", "Four" }
                }
            }

            // ---------------------------------------------------------
            // Test 4: Vertical + looping
            // Upstream: "Basic" story (vertical orientation + looping)
            // ---------------------------------------------------------
            section {
                "data-testid": "basic-vertical-loop",
                h3 { "Vertical + Looping" }
                ButtonGroup { orientation: Orientation::Vertical, r#loop: true,
                    Button { value: "one", "One" }
                    Button { value: "two", "Two" }
                    Button { value: "three", disabled: true, "Three" }
                    Button { value: "four", "Four" }
                }
            }

            // ---------------------------------------------------------
            // Test 5: Edge cases — dynamic insertion + disabling
            // Upstream: "EdgeCases" story
            // ---------------------------------------------------------
            section {
                "data-testid": "edge-cases",
                h3 { "Edge Cases" }
                div { style: "margin-bottom: 8px; display: flex; gap: 8px;",
                    button {
                        "data-testid": "toggle-extra",
                        r#type: "button",
                        onclick: move |_| has_extra.toggle(),
                        "Add/remove Extra"
                    }
                    button {
                        "data-testid": "toggle-disable-one",
                        r#type: "button",
                        onclick: move |_| one_disabled.toggle(),
                        "Disable/Enable One"
                    }
                }
                ButtonGroup {
                    if has_extra() {
                        Button { value: "extra", "Extra" }
                    }
                    Button { value: "one", disabled: one_disabled(), "One" }
                    Button { value: "two", disabled: true, "Two" }
                    Button { value: "three", "Three" }
                    Button { value: "four", "Four" }
                }
                hr { style: "margin-top: 8px;" }
                button {
                    "data-testid": "outside-button",
                    r#type: "button",
                    "Focusable outside of group"
                }
            }
        }
    }
}
