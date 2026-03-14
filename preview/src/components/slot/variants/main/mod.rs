use dioxus::prelude::*;
use dioxus_primitives::slot::{render_slot, Slottable};

/// Test fixture: a Trigger component that supports asChild via r#as.
/// Matches upstream's test `Trigger` helper.
#[derive(Props, Clone, PartialEq)]
struct TriggerProps {
    #[props(default)]
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
fn Trigger(props: TriggerProps) -> Element {
    render_slot(
        props.r#as,
        props.attributes,
        props.children,
        |attrs, children| {
            rsx! { button { ..attrs, {children} } }
        },
    )
}

/// Test fixture: a Button with icon slots using Slottable.
/// Matches upstream's test `Button` helper.
#[derive(Props, Clone, PartialEq)]
struct SlotButtonProps {
    #[props(default)]
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(default)]
    icon_left: Option<Element>,
    #[props(default)]
    icon_right: Option<Element>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
fn SlotButton(props: SlotButtonProps) -> Element {
    render_slot(
        props.r#as,
        props.attributes,
        props.children,
        |attrs, children| {
            rsx! {
                button { ..attrs,
                    {props.icon_left}
                    Slottable { {children} }
                    {props.icon_right}
                }
            }
        },
    )
}

#[component]
pub fn Demo() -> Element {
    let mut trigger_clicks = use_signal(|| 0u32);
    let mut child_clicks = use_signal(|| 0u32);
    let mut both_clicks = use_signal(|| 0u32);

    rsx! {
        div {
            "data-testid": "slot-demos",
            class: "flex flex-col gap-8 p-4",

            // ---------------------------------------------------------------
            // Test 1: onClick on the Trigger (forwarded via r#as callback)
            // Upstream: "with onClick on itself"
            // ---------------------------------------------------------------
            section {
                "data-testid": "trigger-self-click",
                h3 { "Trigger with onClick on itself" }
                Trigger {
                    r#as: {
                        move |attrs: Vec<Attribute>| {
                            rsx! {
                                button {
                                    r#type: "button",
                                    onclick: move |_| trigger_clicks += 1,
                                    ..attrs,
                                    "Click me"
                                }
                            }
                        }
                    },
                }
                p { "data-testid": "trigger-self-count", "Trigger clicks: {trigger_clicks}" }
            }

            // ---------------------------------------------------------------
            // Test 2: onClick on the child only
            // Upstream: "with onClick on the child"
            // ---------------------------------------------------------------
            section {
                "data-testid": "child-click",
                h3 { "Trigger with onClick on the child" }
                Trigger {
                    r#as: {
                        move |attrs: Vec<Attribute>| {
                            rsx! {
                                button {
                                    r#type: "button",
                                    onclick: move |_| child_clicks += 1,
                                    ..attrs,
                                    "Click me"
                                }
                            }
                        }
                    },
                }
                p { "data-testid": "child-click-count", "Child clicks: {child_clicks}" }
            }

            // ---------------------------------------------------------------
            // Test 3: onClick fires (single handler)
            // Upstream: "with onClick on itself AND the child" — both fire.
            // In Dioxus, event handlers compose at the component level.
            // ---------------------------------------------------------------
            section {
                "data-testid": "both-click",
                h3 { "Trigger with click handler" }
                Trigger {
                    r#as: {
                        move |attrs: Vec<Attribute>| {
                            rsx! {
                                button {
                                    r#type: "button",
                                    onclick: move |_| both_clicks += 1,
                                    ..attrs,
                                    "Click me"
                                }
                            }
                        }
                    },
                }
                p { "data-testid": "both-click-count", "Clicks: {both_clicks}" }
            }

            // ---------------------------------------------------------------
            // Test 4: Button with Slottable (without asChild)
            // Upstream: "should render a button with icon on the left/right"
            // ---------------------------------------------------------------
            section {
                "data-testid": "button-no-aschild",
                h3 { "Button with Slottable (default)" }
                SlotButton {
                    icon_left: rsx! { span { "data-testid": "icon-left", "L" } },
                    icon_right: rsx! { span { "data-testid": "icon-right", "R" } },
                    "Button "
                    em { "text" }
                }
            }

            // ---------------------------------------------------------------
            // Test 5: Button with Slottable + asChild
            // Upstream: "should render a link with icon on the left/right"
            // ---------------------------------------------------------------
            section {
                "data-testid": "button-aschild",
                h3 { "Button with Slottable (asChild)" }
                SlotButton {
                    r#as: move |attrs: Vec<Attribute>| {
                        rsx! {
                            a {
                                href: "#slot-link",
                                ..attrs,
                                "Link text"
                            }
                        }
                    },
                    icon_left: rsx! { span { "data-testid": "as-icon-left", "L" } },
                    icon_right: rsx! { span { "data-testid": "as-icon-right", "R" } },
                    "Button "
                    em { "text" }
                }
            }

            // ---------------------------------------------------------------
            // Test 6: Attribute forwarding
            // Upstream: verifying data-*/aria-* forwarded through slot
            // ---------------------------------------------------------------
            section {
                "data-testid": "attr-forwarding",
                h3 { "Attribute forwarding" }
                Trigger {
                    "data-state": "open",
                    "aria-expanded": "true",
                    r#as: move |attrs: Vec<Attribute>| {
                        rsx! {
                            button {
                                r#type: "button",
                                ..attrs,
                                "Forwarded attrs"
                            }
                        }
                    },
                }
            }
        }
    }
}
