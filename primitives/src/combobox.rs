//! Combobox primitive — searchable dropdown select.
//!
//! A composable combobox with typeahead filtering, keyboard navigation,
//! and full ARIA combobox pattern compliance. The dropdown renders in
//! the browser top layer via the `popover` attribute, escaping ancestor
//! overflow/transform/stacking-contexts. Positioning is handled by
//! [`Popper`](crate::popper) with `ComboboxInput` as the anchor.
//!
//! ## Architecture
//!
//! - [`Combobox`] — Root component, manages state and context
//! - [`ComboboxInput`] — Text input with `role="combobox"`
//! - [`ComboboxContent`] — Dropdown content panel (popover)
//! - [`ComboboxList`] — Scrollable container for items (`role="listbox"`)
//! - [`ComboboxItem`] — Individual option (`role="option"`)
//! - [`ComboboxEmpty`] — Shown when no items match filter
//! - [`ComboboxGroup`] — Named group of items
//! - [`ComboboxSeparator`] — Visual separator between groups
//!
//! ## Example
//!
//! ```rust,no_run
//! # use dioxus::prelude::*;
//! # use dioxus_primitives::combobox::*;
//! fn Demo() -> Element {
//!     let mut value = use_signal(String::new);
//!
//!     rsx! {
//!         Combobox {
//!             value: value(),
//!             on_value_change: move |v: String| value.set(v),
//!             ComboboxInput { placeholder: "Select framework..." }
//!             ComboboxContent {
//!                 ComboboxList {
//!                     ComboboxEmpty { "No framework found." }
//!                     ComboboxItem { value: "react", "React" }
//!                     ComboboxItem { value: "vue", "Vue" }
//!                     ComboboxItem { value: "svelte", "Svelte" }
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_attributes::attributes;

use crate::merge_attributes;
use crate::popper::{Align, CollisionPadding, Popper, PopperContent, PopperCtx, Side};
use crate::top_layer::{use_top_layer, TopLayerKind};

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

/// Context shared by Combobox sub-components.
#[derive(Clone, Debug)]
pub struct ComboboxCtx {
    /// Whether the dropdown is open.
    pub open: bool,
    /// Currently selected value.
    pub value: String,
    /// Current filter/search text.
    pub filter_text: String,
    /// Content element ID (for aria-controls).
    pub content_id: String,
    /// Listbox element ID.
    pub listbox_id: String,
}

/// Newtype wrapper for selected display text signal so it doesn't conflict with other `Signal<String>` in context.
#[derive(Clone, Copy)]
pub struct SelectedDisplay(pub Signal<String>);

impl PartialEq for ComboboxCtx {
    fn eq(&self, other: &Self) -> bool {
        self.open == other.open
            && self.value == other.value
            && self.filter_text == other.filter_text
            && self.content_id == other.content_id
            && self.listbox_id == other.listbox_id
    }
}

/// Access the nearest [`Combobox`] context.
pub fn use_combobox() -> ComboboxCtx {
    use_context::<Signal<ComboboxCtx>>().cloned()
}

// ---------------------------------------------------------------------------
// Combobox (root)
// ---------------------------------------------------------------------------

/// Props for [`Combobox`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxProps {
    /// Currently selected value (controlled).
    #[props(default)]
    pub value: String,

    /// Callback when value changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Whether the combobox is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Root combobox component. Manages open, value, and filter state.
///
/// Wraps children in [`Popper`] so [`ComboboxInput`] can register itself
/// as the anchor and [`ComboboxContent`] can be positioned and lifted into
/// the top layer.
#[component]
pub fn Combobox(props: ComboboxProps) -> Element {
    let content_id = crate::use_unique_id();
    let listbox_id = crate::use_unique_id();
    let open = use_signal(|| false);
    let filter_text = use_signal(String::new);
    let selected_display = use_signal(String::new);

    let ctx = ComboboxCtx {
        open: open(),
        value: props.value.clone(),
        filter_text: filter_text(),
        content_id: content_id(),
        listbox_id: listbox_id(),
    };

    use_context_provider(|| Signal::new(ctx.clone()));
    use_context_provider(|| open);
    use_context_provider(|| filter_text);
    use_context_provider(|| props.on_value_change);
    use_context_provider(|| SelectedDisplay(selected_display));

    // Update context when state changes
    let mut ctx_signal = use_context::<Signal<ComboboxCtx>>();
    if *ctx_signal.peek() != ctx {
        ctx_signal.set(ctx);
    }

    rsx! {
        Popper {
            div {
                "data-slot": "combobox",
                "data-state": if open() { "open" } else { "closed" },
                "data-disabled": props.disabled.then_some("true"),
                class: props.class,
                ..props.attributes,
                {props.children}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ComboboxInput
// ---------------------------------------------------------------------------

/// Props for [`ComboboxInput`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxInputProps {
    /// Placeholder text.
    #[props(default)]
    pub placeholder: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Text input with `role="combobox"` and ARIA attributes.
///
/// Handles filtering, opening/closing the dropdown, and keyboard
/// navigation (ArrowDown/ArrowUp to open, Escape to close). Registers
/// itself as the [`crate::popper::Popper`] anchor so the
/// dropdown is positioned relative to the input.
#[component]
pub fn ComboboxInput(props: ComboboxInputProps) -> Element {
    let ctx = use_context::<Signal<ComboboxCtx>>();
    let popper_ctx: PopperCtx = use_context();
    let mut open = use_context::<Signal<bool>>();
    let mut filter_text = use_context::<Signal<String>>();
    let selected_display = use_context::<SelectedDisplay>();

    let is_open = ctx.read().open;
    let listbox_id = ctx.read().listbox_id.clone();

    // Show selected display text when not actively filtering
    let display_value = if filter_text().is_empty() {
        selected_display.0.read().clone()
    } else {
        filter_text()
    };

    rsx! {
        input {
            "data-slot": "combobox-input",
            r#type: "text",
            role: "combobox",
            autocomplete: "off",
            aria_expanded: is_open,
            aria_controls: listbox_id,
            aria_autocomplete: "list",
            placeholder: props.placeholder,
            value: display_value,
            class: props.class,
            onmounted: move |e| {
                popper_ctx.set_anchor_ref(e.data());
            },
            oninput: move |e: FormEvent| {
                let val = e.value();
                filter_text.set(val);
                if !open() {
                    open.set(true);
                }
            },
            onfocus: move |_| {
                filter_text.set(String::new());
                open.set(true);
            },
            onkeydown: move |e: KeyboardEvent| {
                match e.key() {
                    Key::Escape => {
                        open.set(false);
                    }
                    Key::ArrowDown | Key::ArrowUp => {
                        if !open() {
                            open.set(true);
                        }
                    }
                    _ => {}
                }
            },
            ..props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// ComboboxContent
// ---------------------------------------------------------------------------

/// Props for [`ComboboxContent`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxContentProps {
    /// Side of the input to place content. Defaults to `Bottom`.
    #[props(default)]
    pub side: Side,

    /// Offset from the input edge in pixels.
    #[props(default)]
    pub side_offset: f64,

    /// Alignment relative to the input. Defaults to `Start`.
    #[props(default = Align::Start)]
    pub align: Align,

    /// Offset along the alignment axis.
    #[props(default)]
    pub align_offset: f64,

    /// Avoid viewport edge collisions. Defaults to `true`.
    #[props(default = true)]
    pub avoid_collisions: bool,

    /// Collision padding in pixels.
    #[props(default)]
    pub collision_padding: CollisionPadding,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Dropdown content panel for the combobox.
///
/// Positioned relative to the input via [`PopperContent`] and rendered in
/// the browser top layer via `popover="manual"` so the dropdown escapes
/// ancestor overflow/transform/stacking-contexts. Only renders when the
/// combobox is open.
#[component]
pub fn ComboboxContent(props: ComboboxContentProps) -> Element {
    let ctx = use_context::<Signal<ComboboxCtx>>();
    let mut open = use_context::<Signal<bool>>();
    let is_open = ctx.read().open;
    let content_id = ctx.read().content_id.clone();

    let content_attrs = attributes!(div {
        id: content_id,
        "data-slot": "combobox-content",
        "data-state": if is_open { "open" } else { "closed" },
    });
    let merged = merge_attributes(vec![content_attrs, props.attributes]);

    // popover="manual" lifts the floated wrapper into the top layer. We
    // own dismissal via input onkeydown / onblur + item click, so manual
    // mode prevents the browser's light-dismiss from racing our logic.
    let wrapper_attrs = attributes!(div { popover: "manual" });
    let mut wrapper_mounted = use_signal(|| None::<Rc<MountedData>>);
    let set_open = Callback::new(move |v: bool| {
        if !v {
            open.set(false);
        }
    });
    use_top_layer(
        wrapper_mounted.into(),
        open.into(),
        set_open,
        TopLayerKind::PopoverManual,
    );

    if !is_open {
        return rsx! {};
    }

    rsx! {
        PopperContent {
            side: props.side,
            side_offset: props.side_offset,
            align: props.align,
            align_offset: props.align_offset,
            avoid_collisions: props.avoid_collisions,
            collision_padding: props.collision_padding,
            css_var_prefix: "combobox",
            class: props.class,
            content_attributes: merged,
            wrapper_attributes: wrapper_attrs,
            on_wrapper_mounted: move |evt: Event<MountedData>| {
                wrapper_mounted.set(Some(evt.data()));
            },

            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ComboboxList
// ---------------------------------------------------------------------------

/// Props for [`ComboboxList`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxListProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Scrollable container for combobox items with `role="listbox"`.
#[component]
pub fn ComboboxList(props: ComboboxListProps) -> Element {
    let ctx = use_context::<Signal<ComboboxCtx>>();
    let listbox_id = ctx.read().listbox_id.clone();

    rsx! {
        div {
            id: listbox_id,
            "data-slot": "combobox-list",
            role: "listbox",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ComboboxItem
// ---------------------------------------------------------------------------

/// Props for [`ComboboxItem`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxItemProps {
    /// The value of this item.
    pub value: String,

    /// Display text used for filtering. Defaults to `value` if not provided.
    #[props(default)]
    pub text_value: Option<String>,

    /// Whether this item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Individual combobox option with `role="option"`.
///
/// Automatically hides when its text doesn't match the current filter.
/// Sets `aria-selected` when this item's value matches the selected value.
#[component]
pub fn ComboboxItem(props: ComboboxItemProps) -> Element {
    let ctx = use_context::<Signal<ComboboxCtx>>();
    let mut open = use_context::<Signal<bool>>();
    let mut filter_text = use_context::<Signal<String>>();
    let on_value_change = use_context::<Callback<String>>();
    let mut selected_display = use_context::<SelectedDisplay>();

    let is_selected = ctx.read().value == props.value;
    let filter = ctx.read().filter_text.clone();

    // Filter: match against text_value or value
    let match_text = props
        .text_value
        .clone()
        .unwrap_or_else(|| props.value.clone());
    let matches_filter =
        filter.is_empty() || match_text.to_lowercase().contains(&filter.to_lowercase());

    if !matches_filter {
        return rsx! {};
    }

    rsx! {
        div {
            "data-slot": "combobox-item",
            "data-value": props.value.clone(),
            "data-disabled": props.disabled.then_some("true"),
            role: "option",
            aria_selected: is_selected,
            class: props.class,
            onclick: {
                let value = props.value.clone();
                let display = match_text.clone();
                move |_| {
                    if !props.disabled {
                        on_value_change.call(value.clone());
                        selected_display.0.set(display.clone());
                        filter_text.set(String::new());
                        open.set(false);
                    }
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ComboboxEmpty
// ---------------------------------------------------------------------------

/// Props for [`ComboboxEmpty`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxEmptyProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Shown when no items match the current filter.
///
/// Visibility is managed by the consumer — place alongside items in
/// [`ComboboxList`] and the consumer controls when to show it.
#[component]
pub fn ComboboxEmpty(props: ComboboxEmptyProps) -> Element {
    rsx! {
        div {
            "data-slot": "combobox-empty",
            role: "presentation",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ComboboxGroup
// ---------------------------------------------------------------------------

/// Props for [`ComboboxGroup`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxGroupProps {
    /// Group heading text.
    #[props(default)]
    pub heading: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Named group of combobox items with `role="group"`.
#[component]
pub fn ComboboxGroup(props: ComboboxGroupProps) -> Element {
    let heading_id = crate::use_unique_id();

    rsx! {
        div {
            "data-slot": "combobox-group",
            role: "group",
            aria_labelledby: if props.heading.is_some() { Some(heading_id()) } else { None },
            class: props.class,
            ..props.attributes,
            if let Some(heading) = &props.heading {
                div {
                    id: heading_id(),
                    "data-slot": "combobox-group-heading",
                    "{heading}"
                }
            }
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ComboboxSeparator
// ---------------------------------------------------------------------------

/// Props for [`ComboboxSeparator`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxSeparatorProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Visual separator between combobox groups.
#[component]
pub fn ComboboxSeparator(props: ComboboxSeparatorProps) -> Element {
    rsx! {
        div {
            "data-slot": "combobox-separator",
            role: "separator",
            class: props.class,
            ..props.attributes,
        }
    }
}
