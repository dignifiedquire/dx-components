use dioxus::prelude::*;
use dx_icons_tabler::{IconCheck, IconChevronDown, IconChevronUp, IconCopy};

use crate::components::tabs::component::*;
use crate::{ComponentType, HighlightedCode, THEME_CSS};

#[component]
pub(crate) fn CodeBlock(source: HighlightedCode, collapsed: bool) -> Element {
    rsx! {
        div {
            class: "code-block dark-code-block",
            tabindex: "0",
            "data-collapsed": "{collapsed}",
            dangerous_inner_html: source.dark,
        }
        div {
            class: "code-block light-code-block",
            tabindex: "0",
            "data-collapsed": "{collapsed}",
            dangerous_inner_html: source.light,
        }
    }
}

#[component]
pub(crate) fn CopyButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut copied = use_signal(|| false);

    rsx! {
        button {
            class: "copy-button",
            r#type: "button",
            aria_label: "Copy code",
            "data-copied": copied,
            "onclick": "navigator.clipboard.writeText(this.closest('[data-slot=code]')?.querySelector('pre')?.innerText || this.parentNode.firstChild?.innerText || '');",
            onclick: move |_| copied.set(true),
            ..attributes,
            if copied() {
                IconCheck { size: 16 }
            } else {
                IconCopy { size: 16 }
            }
        }
    }
}

#[component]
pub(crate) fn ComponentCode(
    rs_highlighted: HighlightedCode,
    css_highlighted: HighlightedCode,
    #[props(default = ComponentType::Normal)] component_type: ComponentType,
    /// When true, the internal expand/collapse controls are hidden (parent handles it).
    #[props(default = false)]
    external_collapse: bool,
) -> Element {
    let collapsed = use_signal(|| true);
    // When parent handles collapse, always show code fully (parent clips with max-h).
    let is_collapsed = !external_collapse && collapsed();

    rsx! {
        Tabs {
            default_value: "main.rs",
            border_bottom_left_radius: "0.5rem",
            border_bottom_right_radius: "0.5rem",
            horizontal: true,
            width: "100%",
            TabList {
                TabTrigger { value: "main.rs", index: 0usize, "main.rs" }
                TabTrigger { value: "style.css", index: 1usize, "style.css" }
                if component_type != ComponentType::Block {
                    TabTrigger { value: "dx-components-theme.css", index: 2usize, "dx-components-theme.css" }
                }
            }
            div {
                width: "100%",
                height: "100%",
                display: "flex",
                flex_direction: "column",
                justify_content: "center",
                align_items: "center",
                TabContent {
                    index: 0usize,
                    value: "main.rs",
                    width: "100%",
                    position: "relative",
                    CodeBlock { source: rs_highlighted, collapsed: is_collapsed }
                    if !external_collapse {
                        ExpandButton { collapsed }
                    }
                }
                TabContent {
                    index: 1usize,
                    value: "style.css",
                    width: "100%",
                    position: "relative",
                    CodeBlock { source: css_highlighted, collapsed: is_collapsed }
                    if !external_collapse {
                        ExpandButton { collapsed }
                    }
                }
                if component_type != ComponentType::Block {
                    TabContent {
                        index: 2usize,
                        value: "dx-components-theme.css",
                        width: "100%",
                        position: "relative",
                        CodeBlock { source: THEME_CSS, collapsed: is_collapsed }
                        if !external_collapse {
                            ExpandButton { collapsed }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ExpandButton(collapsed: Signal<bool>) -> Element {
    rsx! {
        button {
            aria_label: if collapsed() { "Expand code" } else { "Collapse code" },
            class: "flex w-full items-center justify-center h-8 text-muted-foreground hover:text-foreground transition-colors",
            style: "background: none; border: none; cursor: pointer;",
            r#type: "button",
            onclick: move |_| {
                collapsed.toggle();
            },
            if collapsed() {
                IconChevronDown { size: 20 }
            } else {
                IconChevronUp { size: 20 }
            }
        }
    }
}

#[component]
pub(crate) fn ColapsibleCodeBlock(highlighted: HighlightedCode) -> Element {
    let collapsed = use_signal(|| true);

    rsx! {
        div {
            class: "tabs-content",
            width: "100%",
            height: "100%",
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            border_bottom_left_radius: "0.5rem",
            border_bottom_right_radius: "0.5rem",
            CodeBlock { source: highlighted, collapsed: collapsed() }
            ExpandButton { collapsed }
        }
    }
}
