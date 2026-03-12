use dioxus::prelude::*;

use crate::components;
use crate::components::tabs::component::{Tabs, TabsContent, TabsList, TabsTrigger};
use crate::ui::code_block::{CodeBlock, ComponentCode, CopyButton};
use crate::ui::prev_next::PrevNextNav;
use crate::Route;
use crate::{ComponentMetadata, ComponentType, DemoEntry, HighlightedCode, VariantMetadata};

/// Inner component page renderer. Called from generated per-component page functions.
/// `demos` contains the live demo function pointers for this specific component.
#[component]
pub(crate) fn ComponentPageInner(name: &'static str, demos: Vec<DemoEntry>) -> Element {
    let Some(meta) = components::COMPONENT_LIST.iter().find(|m| m.name == name) else {
        return rsx! {
            div { class: "flex flex-col items-center justify-center py-20",
                h3 { class: "text-lg font-semibold", "Component not found" }
                p { class: "text-muted-foreground mt-2", "The requested component does not exist." }
            }
        };
    };
    rsx! {
        ComponentHighlight { meta: meta.clone(), demos }
    }
}

#[component]
fn ComponentHighlight(meta: ComponentMetadata, demos: Vec<DemoEntry>) -> Element {
    let ComponentMetadata {
        name: raw_name,
        description,
        r#type,
        docs,
        variants,
        component,
        style,
    } = meta;
    let name = raw_name.replace("_", " ");
    let [main_variant, extra_variants @ ..] = variants else {
        unreachable!("Expected at least one variant for component: {}", name);
    };

    let main_demo = demos.iter().find(|d| d.name == main_variant.name);

    rsx! {
        // Title + description
        div { class: "flex flex-col gap-2",
            h1 { class: "scroll-m-24 text-3xl font-semibold tracking-tight capitalize", "{name}" }
            if !description.is_empty() {
                p { class: "text-muted-foreground text-base md:max-w-[80%]", "{description}" }
            }
        }

        // Preview
        if let Some(demo) = main_demo {
            ComponentPreview {
                variant: main_variant.clone(),
                demo_component: demo.component,
                r#type,
                component_name: raw_name,
            }
        }

        // Installation
        section { id: "installation", class: "mt-8",
            h2 { class: "scroll-m-24 text-xl font-semibold tracking-tight mb-4", "Installation" }
            Tabs {
                default_value: "Automatic",
                class: "w-full rounded-xl border border-border overflow-hidden",

                TabsList {
                    TabsTrigger { value: "Automatic", "CLI" }
                    TabsTrigger { value: "Manual", "Manual" }
                }
                TabsContent {
                    value: "Automatic",
                    class: "p-4",
                    CliComponentInstallation { name: raw_name }
                }
                TabsContent {
                    value: "Manual",
                    class: "p-4",
                    ManualComponentInstallation { component, style }
                }
            }
        }

        // Usage / docs
        if !docs.is_empty() {
            section { id: "usage", class: "mt-8",
                h2 { class: "scroll-m-24 text-xl font-semibold tracking-tight mb-4", "Usage" }
                div { class: "docs-content",
                    dangerous_inner_html: docs,
                }
            }
        }

        // Examples (additional variants)
        if !extra_variants.is_empty() {
            section { id: "examples", class: "mt-8",
                h2 { class: "scroll-m-24 text-xl font-semibold tracking-tight mb-6", "Examples" }
                div { class: "space-y-8",
                    for variant in extra_variants {
                        {
                            let demo_fn = demos.iter().find(|d| d.name == variant.name).map(|d| d.component);
                            rsx! {
                                div { id: "{variant.name}",
                                    h3 { class: "scroll-m-24 text-lg font-medium mb-4 capitalize",
                                        {variant.name.replace("_", " ")}
                                    }
                                    if let Some(comp) = demo_fn {
                                        ComponentPreview {
                                            variant: variant.clone(),
                                            demo_component: comp,
                                            r#type,
                                            component_name: raw_name,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Prev/Next
        PrevNextNav { current_name: raw_name }
    }
}

#[component]
fn ComponentPreview(
    variant: VariantMetadata,
    demo_component: fn() -> Element,
    r#type: ComponentType,
    component_name: &'static str,
) -> Element {
    let mut expanded = use_signal(|| false);

    let VariantMetadata {
        name: variant_name,
        rs_highlighted: highlighted,
        css_highlighted,
    } = variant;

    let Comp = demo_component;

    let preview_content = match r#type {
        ComponentType::Normal => rsx! {
            div {
                "data-slot": "preview",
                class: "relative flex min-h-72 w-full items-center justify-center p-6 md:p-10",
                Comp {}
            }
        },
        ComponentType::Block => {
            let route_path = Route::ComponentBlockDemo {
                name: component_name.to_string(),
                variant: variant_name.to_string(),
            }
            .to_string();
            let iframe_src = match router().prefix() {
                Some(prefix) => format!("{prefix}{route_path}"),
                None => route_path,
            };
            rsx! {
                iframe {
                    src: "{iframe_src}",
                    width: "100%",
                    height: "600px",
                    class: "border-0",
                }
            }
        }
    };

    rsx! {
        div { class: "group relative mt-4 mb-12 flex flex-col overflow-hidden rounded-xl border border-border",
            {preview_content}
            div {
                "data-slot": "code",
                class: "relative overflow-hidden border-t border-border",
                if expanded() {
                    div { class: "absolute top-1.5 right-9 z-10 flex items-center",
                        button {
                            r#type: "button",
                            class: "h-7 rounded-md px-2 text-xs font-medium text-muted-foreground hover:text-foreground transition-colors",
                            onclick: move |_| expanded.set(false),
                            "Collapse"
                        }
                        div { class: "mx-1.5 h-4 w-px bg-border" }
                    }
                    CopyButton {
                        position: "absolute",
                        top: "0.75rem",
                        right: "0.5rem",
                        z_index: "10",
                    }
                }
                div {
                    class: if expanded() { "relative" } else { "relative max-h-72 overflow-hidden" },
                    if let Some(css) = css_highlighted {
                        ComponentCode {
                            rs_highlighted: highlighted,
                            css_highlighted: css,
                            component_type: r#type,
                            external_collapse: true,
                        }
                    } else {
                        CodeBlock { source: highlighted, collapsed: !expanded() }
                    }
                }
                if !expanded() {
                    div { class: "absolute inset-0 flex items-center justify-center pb-4",
                        div {
                            class: "absolute inset-0",
                            style: "background: linear-gradient(to top, var(--background), color-mix(in oklab, var(--background) 60%, transparent), transparent);",
                        }
                        button {
                            r#type: "button",
                            class: "relative z-10 inline-flex items-center justify-center rounded-lg border border-border bg-background px-3 py-1.5 text-sm font-medium text-foreground hover:bg-muted transition-colors",
                            onclick: move |_| expanded.set(true),
                            "View Code"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ManualComponentInstallation(component: HighlightedCode, style: HighlightedCode) -> Element {
    rsx! {
        ol { class: "list-decimal list-inside space-y-2 text-sm text-muted-foreground",
            li {
                "Add the "
                code { class: "text-foreground", "dx-components-theme.css" }
                " file to your project and import it in the root of your app."
            }
            li { "Add the style.css file to your project." }
            li { "Create a component based on the main.rs below." }
            li { "Modify your components and styles as needed." }
        }
        div { class: "mt-4",
            ComponentCode {
                rs_highlighted: component,
                css_highlighted: style,
                component_type: ComponentType::Normal,
            }
        }
    }
}

#[component]
fn CliComponentInstallation(name: String) -> Element {
    rsx! {
        div { class: "space-y-4 text-sm",
            div {
                p { class: "text-muted-foreground mb-2", "Install the Dioxus CLI:" }
                div {
                    "data-slot": "code",
                    class: "relative flex items-center rounded-md bg-muted px-4 py-3 font-mono text-sm",
                    span { class: "text-muted-foreground mr-2", "$" }
                    span { class: "text-foreground", "cargo install dioxus-cli" }
                    CopyButton { position: "absolute", top: "0.5rem", right: "0.5rem" }
                }
            }
            div {
                p { class: "text-muted-foreground mb-2", "Add the component:" }
                div {
                    "data-slot": "code",
                    class: "relative flex items-center rounded-md bg-muted px-4 py-3 font-mono text-sm",
                    span { class: "text-muted-foreground mr-2", "$" }
                    span { class: "text-foreground", "dx components add {name}" }
                    CopyButton { position: "absolute", top: "0.5rem", right: "0.5rem" }
                }
            }
        }
    }
}
