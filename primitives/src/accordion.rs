//! Defines the [`Accordion`] primitive and its sub-components.
//!
//! This is an unstyled behavioral primitive matching Radix UI's architecture.
//! AccordionItem wraps Collapsible, AccordionTrigger wraps CollapsibleTrigger,
//! and AccordionContent wraps CollapsibleContent with CSS variable aliases.

use crate::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::dioxus_elements::Key;
use crate::{use_effect_cleanup, use_unique_id};
use dioxus::prelude::*;
use std::rc::Rc;

// ---------------------------------------------------------------------------
// Accordion-level context (manages which items are open, keyboard nav)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct AccordionContext {
    open_values: Signal<Vec<String>>,
    allow_multiple_open: ReadSignal<bool>,
    disabled: ReadSignal<bool>,
    collapsible: ReadSignal<bool>,
    horizontal: ReadSignal<bool>,
    num_items: Signal<usize>,
    focused_index: Signal<Option<usize>>,
}

impl AccordionContext {
    fn on_item_open(&mut self, value: &str) {
        let mut open = self.open_values.write();
        if !(self.allow_multiple_open)() {
            open.clear();
        }
        open.push(value.to_string());
    }

    fn on_item_close(&mut self, value: &str) {
        let mut open = self.open_values.write();
        if !(self.collapsible)() && open.len() == 1 {
            return;
        }
        open.retain(|v| v != value);
    }

    fn is_open(&self, value: &str) -> bool {
        self.open_values.read().iter().any(|v| v == value)
    }

    fn is_disabled(&self) -> bool {
        (self.disabled)()
    }

    fn is_horizontal(&self) -> bool {
        (self.horizontal)()
    }

    fn set_focus(&mut self, index: Option<usize>) {
        self.focused_index.set(index);
    }

    fn is_focused(&self, index: usize) -> bool {
        *self.focused_index.read() == Some(index)
    }

    fn focus_next(&mut self) {
        let Some(current) = *self.focused_index.read() else {
            return;
        };
        let count = (self.num_items)();
        if count == 0 {
            return;
        }
        self.focused_index.set(Some((current + 1) % count));
    }

    fn focus_prev(&mut self) {
        let Some(current) = *self.focused_index.read() else {
            return;
        };
        let count = (self.num_items)();
        if count == 0 {
            return;
        }
        self.focused_index
            .set(Some(if current == 0 { count - 1 } else { current - 1 }));
    }

    fn focus_start(&mut self) {
        self.focused_index.set(Some(0));
    }

    fn focus_end(&mut self) {
        let count = (self.num_items)();
        if count > 0 {
            self.focused_index.set(Some(count - 1));
        }
    }
}

// ---------------------------------------------------------------------------
// Per-item context (shared between AccordionItem, Trigger, Content)
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct AccordionItemCtx {
    value: String,
    index: usize,
    open: Memo<bool>,
    trigger_id: Signal<String>,
    content_id: Signal<String>,
}

// ---------------------------------------------------------------------------
// Accordion (root)
// ---------------------------------------------------------------------------

/// The props for the [`Accordion`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    /// Whether multiple accordion items are allowed to be open at once.
    #[props(default)]
    pub allow_multiple_open: ReadSignal<bool>,

    /// Set whether the accordion is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the accordion can be fully collapsed.
    ///
    /// Setting this to true will allow all accordion items to close. Defaults to true.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub collapsible: ReadSignal<bool>,

    /// Whether the accordion is horizontal.
    #[props(default)]
    pub horizontal: ReadSignal<bool>,

    /// Additional classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the accordion.
    pub children: Element,
}

/// # Accordion
///
/// A vertically stacked set of interactive headings that each reveal a section of content.
///
/// ## Data attributes
///
/// - `data-slot="accordion"`
/// - `data-disabled`: Present when disabled
#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let mut ctx = use_context_provider(|| AccordionContext {
        open_values: Signal::new(Vec::new()),
        allow_multiple_open: props.allow_multiple_open,
        disabled: props.disabled,
        collapsible: props.collapsible,
        horizontal: props.horizontal,
        num_items: Signal::new(0),
        focused_index: Signal::new(None),
    });

    rsx! {
        div {
            "data-slot": "accordion",
            class: props.class,
            "data-disabled": (props.disabled)(),

            onfocusout: move |_| {
                ctx.set_focus(None);
            },

            ..props.attributes,

            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AccordionItem
// ---------------------------------------------------------------------------

/// The props for the [`AccordionItem`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionItemProps {
    /// A unique string value for this accordion item.
    pub value: String,

    /// Whether the accordion item is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether this accordion item should be opened by default.
    #[props(default)]
    pub default_open: bool,

    /// The index of the accordion item within the [`Accordion`].
    pub index: usize,

    /// Additional classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to extend the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the accordion item.
    pub children: Element,
}

/// # AccordionItem
///
/// Contains all parts of a collapsible section inside an [`Accordion`].
/// Wraps `Collapsible` root with controlled open state.
///
/// ## Data attributes
///
/// - `data-state`: `"open"` or `"closed"` (from Collapsible)
/// - `data-disabled`: Present when disabled (from Collapsible)
#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    let mut ctx: AccordionContext = use_context();
    let trigger_id = use_unique_id();
    let content_id = use_unique_id();

    let value = props.value.clone();

    use_hook(|| {
        ctx.num_items += 1;
    });
    use_effect_cleanup(move || {
        ctx.num_items -= 1;
    });

    use_hook(|| {
        if props.default_open {
            ctx.on_item_open(&value);
        }
    });

    let item_value = props.value.clone();
    let open = use_memo(move || ctx.is_open(&item_value));

    use_context_provider(|| AccordionItemCtx {
        value: props.value.clone(),
        index: props.index,
        open,
        trigger_id,
        content_id,
    });

    let is_disabled = ctx.is_disabled() || (props.disabled)();

    let on_open_change = {
        let v = props.value.clone();
        use_callback(move |new_open: bool| {
            if new_open {
                ctx.on_item_open(&v);
            } else {
                ctx.on_item_close(&v);
            }
        })
    };

    // Wrap the bool memo as Option<bool> so Collapsible sees a controlled value.
    // Using a Memo (not Signal::new) keeps reactivity stable across renders.
    let controlled_open = use_memo(move || Some(open()));

    rsx! {
        Collapsible {
            open: controlled_open,
            on_open_change: on_open_change,
            disabled: if is_disabled { true } else { false },
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AccordionHeader
// ---------------------------------------------------------------------------

/// The props for the [`AccordionHeader`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionHeaderProps {
    /// Additional classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to extend the header element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children (should contain an [`AccordionTrigger`]).
    pub children: Element,
}

/// # AccordionHeader
///
/// Wraps the trigger in an `<h3>` element, matching Radix's `AccordionHeader`.
///
/// ## Data attributes
///
/// - `data-slot="accordion-header"`
/// - `data-state`: `"open"` or `"closed"`
/// - `data-disabled`: Present when disabled
#[component]
pub fn AccordionHeader(props: AccordionHeaderProps) -> Element {
    let item: AccordionItemCtx = use_context();
    let ctx: AccordionContext = use_context();
    let data_state = if (item.open)() { "open" } else { "closed" };
    let is_disabled = ctx.is_disabled();

    rsx! {
        h3 {
            "data-slot": "accordion-header",
            "data-state": data_state,
            "data-disabled": is_disabled,
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AccordionTrigger
// ---------------------------------------------------------------------------

/// The props for the [`AccordionTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionTriggerProps {
    /// Additional classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to extend the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the accordion trigger element.
    pub children: Element,
}

/// # AccordionTrigger
///
/// Toggles the open/closed state of an [`AccordionItem`].
/// Wraps `CollapsibleTrigger` and adds keyboard navigation.
///
/// ## Data attributes
///
/// - `data-state`: `"open"` or `"closed"` (from CollapsibleTrigger)
/// - `data-disabled`: Present when disabled (from CollapsibleTrigger)
#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    let mut ctx: AccordionContext = use_context();
    let item: AccordionItemCtx = use_context();

    let mut btn_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let index = item.index;
    use_effect(move || {
        let is_focused = ctx.is_focused(index);
        if is_focused {
            if let Some(md) = btn_ref() {
                spawn(async move {
                    let _ = md.set_focus(true).await;
                });
            }
        }
    });

    rsx! {
        CollapsibleTrigger {
            id: Signal::new(Some((item.trigger_id)())),
            class: props.class,
            attributes: props.attributes,
            onmounted: move |data: MountedEvent| btn_ref.set(Some(data.data())),
            onfocus: move |_| {
                ctx.set_focus(Some(index));
            },
            onkeydown: move |event: KeyboardEvent| {
                let key = event.key();
                let horizontal = ctx.is_horizontal();
                let mut prevent_default = true;

                match key {
                    Key::ArrowUp if !horizontal => ctx.focus_prev(),
                    Key::ArrowDown if !horizontal => ctx.focus_next(),
                    Key::ArrowLeft if horizontal => ctx.focus_prev(),
                    Key::ArrowRight if horizontal => ctx.focus_next(),
                    Key::Home => ctx.focus_start(),
                    Key::End => ctx.focus_end(),
                    _ => prevent_default = false,
                };

                if prevent_default {
                    event.prevent_default();
                }
            },

            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AccordionContent
// ---------------------------------------------------------------------------

/// The props for the [`AccordionContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionContentProps {
    /// Additional classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to extend the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the accordion content element.
    pub children: Element,
}

/// # AccordionContent
///
/// The collapsible content for an [`AccordionItem`].
/// Wraps `CollapsibleContent` and aliases CSS variables for accordion-specific animations.
///
/// Sets `--radix-accordion-content-height` and `--radix-accordion-content-width`
/// as aliases to `--radix-collapsible-content-height` and `--radix-collapsible-content-width`.
///
/// ## Data attributes
///
/// - `data-state`: `"open"` or `"closed"` (from CollapsibleContent)
#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    let item: AccordionItemCtx = use_context();
    let content_id = item.content_id;

    rsx! {
        CollapsibleContent {
            id: Signal::new(Some(content_id())),
            class: props.class,
            attributes: props.attributes,
            role: "region",
            aria_labelledby: item.trigger_id,
            style: "--radix-accordion-content-height: var(--radix-collapsible-content-height); --radix-accordion-content-width: var(--radix-collapsible-content-width);",

            {props.children}
        }
    }
}
