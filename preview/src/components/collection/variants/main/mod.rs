use dioxus::prelude::*;
use dioxus_primitives::collection::{use_collection_item, CollectionContext};

// ---------------------------------------------------------------------------
// ItemData — matches upstream `type ItemData = { disabled: boolean }`
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
struct ItemData {
    disabled: bool,
    label: String,
}

// ---------------------------------------------------------------------------
// List — wraps CollectionContext.new() + provides context
// Upstream: Collection.Provider + Collection.Slot
// ---------------------------------------------------------------------------

#[component]
fn List(children: Element) -> Element {
    let ctx = use_hook(CollectionContext::<ItemData>::new);
    use_context_provider(|| ctx);

    rsx! {
        ul {
            style: "width: 200px; list-style: none; padding: 0;",
            {children}
        }
    }
}

// ---------------------------------------------------------------------------
// Item — registers into collection via use_collection_item
// Upstream: Collection.ItemSlot
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
struct ItemProps {
    #[props(default = false)]
    disabled: bool,
    #[props(default)]
    style: Option<String>,
    children: Element,
    label: String,
}

#[component]
fn Item(props: ItemProps) -> Element {
    let mut mounted = use_collection_item(ItemData {
        disabled: props.disabled,
        label: props.label.clone(),
    });

    let opacity = if props.disabled { "0.3" } else { "1" };
    let style = format!(
        "opacity: {}; padding: 4px 8px; {}",
        opacity,
        props.style.as_deref().unwrap_or("")
    );

    rsx! {
        li {
            onmounted: move |data| mounted.set(Some(data.data())),
            style: "{style}",
            "data-disabled": if props.disabled { "true" } else { "false" },
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// LogItems — renders collection state for testing
// Upstream: useCollection() + console.log
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
struct LogItemsProps {
    #[props(default = "items".to_string())]
    name: String,
}

#[component]
fn LogItems(props: LogItemsProps) -> Element {
    let ctx: CollectionContext<ItemData> = use_context();
    let items = ctx.get_items();
    let count = items.len();
    let labels: Vec<String> = items.iter().map(|i| i.data.label.clone()).collect();
    let disabled: Vec<String> = items
        .iter()
        .filter(|i| i.data.disabled)
        .map(|i| i.data.label.clone())
        .collect();

    rsx! {
        div {
            "data-testid": "log-{props.name}",
            style: "font-size: 12px; color: #666; margin-top: 8px; padding: 4px; border-top: 1px solid #eee;",
            span { "data-testid": "count-{props.name}", "Count: {count}" }
            {" | "}
            span { "data-testid": "labels-{props.name}", "Labels: {labels:?}" }
            {" | "}
            span { "data-testid": "disabled-{props.name}", "Disabled: {disabled:?}" }
        }
    }
}

// ---------------------------------------------------------------------------
// Demo
// ---------------------------------------------------------------------------

#[component]
pub fn Demo() -> Element {
    let mut has_tomato = use_signal(|| false);
    let mut green_disabled = use_signal(|| false);

    rsx! {
        div {
            "data-testid": "collection-demos",
            class: "flex flex-col gap-8 p-4",

            // ---------------------------------------------------------
            // Test 1: Basic — simple list with disabled items
            // Upstream: collection.stories.tsx "Basic"
            // ---------------------------------------------------------
            section {
                "data-testid": "basic",
                h3 { "Basic" }
                List {
                    Item { label: "Red", "Red" }
                    Item { label: "Green", disabled: true, "Green" }
                    Item { label: "Blue", "Blue" }
                    LogItems {}
                }
            }

            // ---------------------------------------------------------
            // Test 2: With elements in between (non-collection items)
            // Upstream: collection.stories.tsx "WithElementInBetween"
            // ---------------------------------------------------------
            section {
                "data-testid": "element-between",
                h3 { "With Element In Between" }
                List {
                    div { style: "font-variant: small-caps; padding: 4px;", "Colors" }
                    Item { label: "Red", "Red" }
                    Item { label: "Green", disabled: true, "Green" }
                    Item { label: "Blue", "Blue" }
                    div { style: "font-variant: small-caps; padding: 4px;", "Words" }
                    Item { label: "Hello", "Hello" }
                    Item { label: "World", "World" }
                    LogItems {}
                }
            }

            // ---------------------------------------------------------
            // Test 3: Dynamic insertion — add/remove items
            // Upstream: collection.stories.tsx "DynamicInsertion"
            // ---------------------------------------------------------
            section {
                "data-testid": "dynamic-insertion",
                h3 { "Dynamic Insertion" }
                button {
                    "data-testid": "toggle-tomato",
                    r#type: "button",
                    onclick: move |_| has_tomato.toggle(),
                    if has_tomato() { "Remove Tomato" } else { "Add Tomato" }
                }
                List {
                    Item { label: "Red", "Red" }
                    if has_tomato() {
                        Item { label: "Tomato", style: "color: tomato;", "Tomato" }
                    }
                    Item { label: "Green", disabled: true, "Green" }
                    Item { label: "Blue", "Blue" }
                    LogItems {}
                }
            }

            // ---------------------------------------------------------
            // Test 4: Changing item state
            // Upstream: collection.stories.tsx "WithChangingItem"
            // ---------------------------------------------------------
            section {
                "data-testid": "changing-item",
                h3 { "With Changing Item" }
                button {
                    "data-testid": "toggle-disabled",
                    r#type: "button",
                    onclick: move |_| green_disabled.toggle(),
                    if green_disabled() { "Enable Green" } else { "Disable Green" }
                }
                List {
                    Item { label: "Red", "Red" }
                    Item { label: "Green", disabled: green_disabled(), "Green" }
                    Item { label: "Blue", "Blue" }
                    LogItems {}
                }
            }

            // ---------------------------------------------------------
            // Test 5: Nested collections
            // Upstream: collection.stories.tsx "Nested"
            // ---------------------------------------------------------
            section {
                "data-testid": "nested",
                h3 { "Nested" }
                List {
                    Item { label: "1", "1" }
                    Item { label: "2",
                        "2"
                        List {
                            Item { label: "2.1", "2.1" }
                            Item { label: "2.2", "2.2" }
                            Item { label: "2.3", "2.3" }
                            LogItems { name: "inner" }
                        }
                    }
                    Item { label: "3", "3" }
                    LogItems { name: "outer" }
                }
            }
        }
    }
}
