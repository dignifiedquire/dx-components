//! Accordion primitive — matches `@radix-ui/react-accordion`.

use crate::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::collection::{use_collection_item, CollectionContext, CollectionItem};
use crate::dioxus_elements::Key;
pub use crate::direction::{Direction, Orientation};
use crate::{use_controlled, use_unique_id};
use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// Radix `type: "single" | "multiple"`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AccordionType {
    /// Only one item open at a time.
    #[default]
    Single,
    /// Multiple items can be open simultaneously.
    Multiple,
}

// ---------------------------------------------------------------------------
// Collection item data (accordion-specific)
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
struct TriggerData {
    value: String,
    disabled: ReadSignal<bool>,
}

type AccordionCollection = CollectionContext<TriggerData>;
type AccordionCollectionItem = CollectionItem<TriggerData>;

// ---------------------------------------------------------------------------
// Contexts — matches Radix's 3 providers + per-item provider
// ---------------------------------------------------------------------------

// Radix: AccordionValueProvider
#[derive(Clone, Copy)]
struct AccordionValueCtx {
    value: Memo<Vec<String>>,
    set_value: Callback<Vec<String>>,
    accordion_type: AccordionType,
    collapsible: ReadSignal<bool>,
}

impl AccordionValueCtx {
    fn on_item_open(&self, item_value: &str) {
        match self.accordion_type {
            AccordionType::Single => {
                self.set_value.call(vec![item_value.to_string()]);
            }
            AccordionType::Multiple => {
                let current = self.value.read();
                if !current.iter().any(|v| v == item_value) {
                    let mut new_value = current.clone();
                    new_value.push(item_value.to_string());
                    drop(current);
                    self.set_value.call(new_value);
                }
            }
        }
    }

    fn on_item_close(&self, item_value: &str) {
        match self.accordion_type {
            AccordionType::Single => {
                if (self.collapsible)() {
                    self.set_value.call(vec![]);
                }
            }
            AccordionType::Multiple => {
                let new_value: Vec<String> = self
                    .value
                    .read()
                    .iter()
                    .filter(|v| v.as_str() != item_value)
                    .cloned()
                    .collect();
                self.set_value.call(new_value);
            }
        }
    }

    fn is_open(&self, value: &str) -> bool {
        self.value.read().iter().any(|v| v == value)
    }
}

// Radix: AccordionCollapsibleProvider
#[derive(Clone, Copy)]
struct AccordionCollapsibleCtx {
    collapsible: Memo<bool>,
}

// Radix: AccordionImplProvider
#[derive(Clone, Copy)]
struct AccordionCtx {
    disabled: ReadSignal<bool>,
    orientation: ReadSignal<Orientation>,
    collection: AccordionCollection,
    // Tracks currently focused trigger for keyboard navigation.
    // Dioxus adaptation of Radix's `event.target` comparison (we can't compare
    // DOM refs, so we track the focused item's value via onfocus).
    focused_value: Signal<Option<String>>,
}

// Radix: AccordionItemProvider
#[derive(Clone)]
struct AccordionItemCtx {
    value: String,
    disabled: ReadSignal<bool>,
    open: Memo<bool>,
    trigger_id: Signal<String>,
    content_id: Signal<String>,
}

// ---------------------------------------------------------------------------
// Accordion (root)
// ---------------------------------------------------------------------------

/// Props for [`Accordion`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    #[props(default)]
    pub r#type: AccordionType,

    #[props(default)]
    pub value: ReadSignal<Option<Vec<String>>>,

    #[props(default)]
    pub default_value: Vec<String>,

    #[props(default)]
    pub on_value_change: Callback<Vec<String>>,

    #[props(default)]
    pub collapsible: ReadSignal<bool>,

    #[props(default)]
    pub disabled: ReadSignal<bool>,

    #[props(default)]
    pub dir: ReadSignal<Direction>,

    #[props(default)]
    pub orientation: ReadSignal<Orientation>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// A vertically stacked set of interactive headings that each reveal content.
#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let accordion_type = props.r#type;
    let user_collapsible = props.collapsible;
    let effective_collapsible = use_memo(move || match accordion_type {
        AccordionType::Multiple => true,
        AccordionType::Single => (user_collapsible)(),
    });

    // Radix: Collection.Provider wraps root
    let collection = use_context_provider(AccordionCollection::new);
    let focused_value: Signal<Option<String>> = use_signal(|| None);

    use_context_provider(|| AccordionValueCtx {
        value,
        set_value,
        accordion_type: props.r#type,
        collapsible: user_collapsible,
    });

    use_context_provider(|| AccordionCollapsibleCtx {
        collapsible: effective_collapsible,
    });

    let ctx = use_context_provider(|| AccordionCtx {
        disabled: props.disabled,
        orientation: props.orientation,
        collection,
        focused_value,
    });

    let orientation = (props.orientation)();
    let disabled = props.disabled;
    let dir = props.dir;

    rsx! {
        div {
            "data-slot": "accordion",
            "data-orientation": orientation.as_str(),
            "data-disabled": (disabled)(),
            class: props.class,

            // Radix: onKeyDown on root div (AccordionImpl)
            onkeydown: move |event: KeyboardEvent| {
                if (disabled)() {
                    return;
                }

                let key = event.key();
                let orientation = (ctx.orientation)();
                let is_ltr = (dir)() == Direction::Ltr;

                enum Action { Next, Prev, First, Last, None }
                let action = match key {
                    Key::ArrowDown if orientation == Orientation::Vertical => Action::Next,
                    Key::ArrowUp if orientation == Orientation::Vertical => Action::Prev,
                    Key::ArrowRight if orientation == Orientation::Horizontal => {
                        if is_ltr { Action::Next } else { Action::Prev }
                    }
                    Key::ArrowLeft if orientation == Orientation::Horizontal => {
                        if is_ltr { Action::Prev } else { Action::Next }
                    }
                    Key::Home => Action::First,
                    Key::End => Action::Last,
                    _ => Action::None,
                };

                if matches!(action, Action::None) {
                    return;
                }
                event.prevent_default();

                // Radix: getItems().filter(item => !item.ref.current?.disabled)
                let items = ctx.collection.get_items();
                let enabled: Vec<&AccordionCollectionItem> = items
                    .iter()
                    .filter(|i| !(i.data.disabled)())
                    .collect();
                if enabled.is_empty() {
                    return;
                }

                let trigger_count = enabled.len();

                // Find current index from focused_value
                let current_value = (ctx.focused_value)();
                let current_idx = current_value
                    .as_ref()
                    .and_then(|cv| enabled.iter().position(|i| i.data.value == *cv))
                    .unwrap_or(0);

                let target_idx = match action {
                    Action::Next => (current_idx + 1) % trigger_count,
                    Action::Prev => (current_idx + trigger_count - 1) % trigger_count,
                    Action::First => 0,
                    Action::Last => trigger_count - 1,
                    Action::None => unreachable!(),
                };

                // Radix: triggerCollection[clampedIndex].ref.current?.focus()
                if let Some(md) = (enabled[target_idx].mounted)() {
                    spawn(async move {
                        let _ = md.set_focus(true).await;
                    });
                }
            },

            ..props.attributes,

            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AccordionItem
// ---------------------------------------------------------------------------

/// Props for [`AccordionItem`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct AccordionItemProps {
    pub value: String,

    #[props(default)]
    pub disabled: ReadSignal<bool>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// Contains all parts of a collapsible section inside an [`Accordion`].
#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    let value_ctx: AccordionValueCtx = use_context();
    let ctx: AccordionCtx = use_context();
    let trigger_id = use_unique_id();
    let content_id = use_unique_id();

    let item_value = props.value.clone();
    let open = use_memo(move || value_ctx.is_open(&item_value));

    let item_disabled = props.disabled;
    let root_disabled = ctx.disabled;
    let is_disabled = use_memo(move || (root_disabled)() || (item_disabled)());

    // Radix: AccordionItemProvider
    use_context_provider(|| AccordionItemCtx {
        value: props.value.clone(),
        disabled: is_disabled.into(),
        open,
        trigger_id,
        content_id,
    });

    let on_open_change = {
        let v = props.value.clone();
        use_callback(move |new_open: bool| {
            if new_open {
                value_ctx.on_item_open(&v);
            } else {
                value_ctx.on_item_close(&v);
            }
        })
    };

    let controlled_open = use_memo(move || Some(open()));
    let orientation = (ctx.orientation)().as_str();

    // Radix: CollapsiblePrimitive.Root
    rsx! {
        Collapsible {
            open: controlled_open,
            on_open_change: on_open_change,
            disabled: is_disabled(),
            class: props.class,
            "data-slot": "accordion-item",
            "data-orientation": orientation,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AccordionHeader
// ---------------------------------------------------------------------------

/// Props for [`AccordionHeader`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct AccordionHeaderProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// Wraps the trigger in an `<h3>` — Radix's `Primitive.h3`.
#[component]
pub fn AccordionHeader(props: AccordionHeaderProps) -> Element {
    let item: AccordionItemCtx = use_context();
    let ctx: AccordionCtx = use_context();
    let data_state = if (item.open)() { "open" } else { "closed" };

    rsx! {
        h3 {
            "data-slot": "accordion-header",
            "data-orientation": (ctx.orientation)().as_str(),
            "data-state": data_state,
            "data-disabled": (item.disabled)(),
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AccordionTrigger
// ---------------------------------------------------------------------------

/// Props for [`AccordionTrigger`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct AccordionTriggerProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// Toggles the collapsed state of an [`AccordionItem`].
#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    let mut ctx: AccordionCtx = use_context();
    let collapsible_ctx: AccordionCollapsibleCtx = use_context();
    let item: AccordionItemCtx = use_context();

    // Collection.ItemSlot — register this trigger for keyboard navigation
    let mut mounted_ref = use_collection_item(TriggerData {
        value: item.value.clone(),
        disabled: item.disabled,
    });

    // Radix: aria-disabled={(itemContext.open && !collapsibleContext.collapsible) || undefined}
    let aria_disabled_val = if (item.open)() && !(collapsible_ctx.collapsible)() {
        Some("true".to_string())
    } else {
        None
    };

    let focus_value = item.value.clone();

    // Radix: CollapsiblePrimitive.Trigger
    rsx! {
        CollapsibleTrigger {
            id: Some((item.trigger_id)()),
            class: props.class,
            "data-orientation": (ctx.orientation)().as_str(),
            "data-radix-collection-item": "",
            "aria-disabled": aria_disabled_val,
            attributes: props.attributes,
            onmounted: move |event: Event<MountedData>| {
                mounted_ref.set(Some(event.data()));
            },
            onfocus: move |_| {
                ctx.focused_value.set(Some(focus_value.clone()));
            },
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AccordionContent
// ---------------------------------------------------------------------------

/// Props for [`AccordionContent`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct AccordionContentProps {
    #[props(default)]
    pub force_mount: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// Collapsible content for an [`AccordionItem`].
#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    let item: AccordionItemCtx = use_context();
    let ctx: AccordionCtx = use_context();

    rsx! {
        CollapsibleContent {
            id: Some((item.content_id)()),
            force_mount: props.force_mount,
            class: props.class,
            "data-orientation": (ctx.orientation)().as_str(),
            attributes: props.attributes,
            role: "region",
            aria_labelledby: item.trigger_id,
            style: "--radix-accordion-content-height: var(--radix-collapsible-content-height); --radix-accordion-content-width: var(--radix-collapsible-content-width);",
            {props.children}
        }
    }
}
