//! Command primitive — command palette / filterable list.
//!
//! A composable command menu with search filtering, keyboard navigation,
//! and grouping. Inspired by cmdk (used by shadcn/ui).
//!
//! ## Architecture
//!
//! - [`Command`] — Root component, manages filter state
//! - [`CommandInput`] — Search input
//! - [`CommandList`] — Scrollable container for items
//! - [`CommandEmpty`] — Shown when no items match filter
//! - [`CommandGroup`] — Named group of items
//! - [`CommandItem`] — Individual command/action
//! - [`CommandSeparator`] — Visual separator
//! - [`CommandShortcut`] — Keyboard shortcut display
//!
//! ## Example
//!
//! ```rust,no_run
//! # use dioxus::prelude::*;
//! # use dioxus_primitives::command::*;
//! fn Demo() -> Element {
//!     rsx! {
//!         Command {
//!             CommandInput { placeholder: "Type a command..." }
//!             CommandList {
//!                 CommandEmpty { "No results found." }
//!                 CommandGroup { heading: "Actions",
//!                     CommandItem { value: "copy", "Copy" }
//!                     CommandItem { value: "paste", "Paste" }
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

/// Context shared by Command sub-components.
#[derive(Clone, Debug)]
pub struct CommandCtx {
    /// Current search/filter text.
    pub filter_text: String,
}

impl PartialEq for CommandCtx {
    fn eq(&self, other: &Self) -> bool {
        self.filter_text == other.filter_text
    }
}

/// Access the nearest [`Command`] context.
pub fn use_command() -> CommandCtx {
    use_context::<Signal<CommandCtx>>().cloned()
}

// ---------------------------------------------------------------------------
// Command (root)
// ---------------------------------------------------------------------------

/// Props for [`Command`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandProps {
    /// Callback when the selected value changes.
    #[props(default)]
    pub on_select: Callback<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Root command palette component. Manages filter state and context.
#[component]
pub fn Command(props: CommandProps) -> Element {
    let filter_text = use_signal(String::new);

    let ctx = CommandCtx {
        filter_text: filter_text(),
    };

    use_context_provider(|| Signal::new(ctx.clone()));
    use_context_provider(|| filter_text);
    use_context_provider(|| props.on_select);

    // Update context when state changes
    let mut ctx_signal = use_context::<Signal<CommandCtx>>();
    if *ctx_signal.peek() != ctx {
        ctx_signal.set(ctx);
    }

    rsx! {
        div {
            "data-slot": "command",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CommandInput
// ---------------------------------------------------------------------------

/// Props for [`CommandInput`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandInputProps {
    /// Placeholder text.
    #[props(default)]
    pub placeholder: Option<String>,

    /// Additional CSS classes for the input element.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Search input for filtering command items.
#[component]
pub fn CommandInput(props: CommandInputProps) -> Element {
    let mut filter_text = use_context::<Signal<String>>();

    rsx! {
        div {
            "data-slot": "command-input-wrapper",

            input {
                "data-slot": "command-input",
                r#type: "text",
                autocomplete: "off",
                placeholder: props.placeholder,
                value: filter_text(),
                class: props.class,
                oninput: move |e: FormEvent| {
                    filter_text.set(e.value());
                },
                ..props.attributes,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// CommandList
// ---------------------------------------------------------------------------

/// Props for [`CommandList`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandListProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Scrollable container for command items.
#[component]
pub fn CommandList(props: CommandListProps) -> Element {
    rsx! {
        div {
            "data-slot": "command-list",
            role: "listbox",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CommandEmpty
// ---------------------------------------------------------------------------

/// Props for [`CommandEmpty`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandEmptyProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Shown when no command items match the current filter.
#[component]
pub fn CommandEmpty(props: CommandEmptyProps) -> Element {
    rsx! {
        div {
            "data-slot": "command-empty",
            role: "presentation",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CommandGroup
// ---------------------------------------------------------------------------

/// Props for [`CommandGroup`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandGroupProps {
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

/// Named group of command items with `role="group"`.
#[component]
pub fn CommandGroup(props: CommandGroupProps) -> Element {
    let heading_id = crate::use_unique_id();

    rsx! {
        div {
            "data-slot": "command-group",
            role: "group",
            aria_labelledby: if props.heading.is_some() { Some(heading_id()) } else { None },
            class: props.class,
            ..props.attributes,
            if let Some(heading) = &props.heading {
                div {
                    id: heading_id(),
                    "data-slot": "command-group-heading",
                    "{heading}"
                }
            }
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CommandItem
// ---------------------------------------------------------------------------

/// Props for [`CommandItem`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandItemProps {
    /// The value used for filtering and selection.
    pub value: String,

    /// Display text for filtering. Defaults to `value`.
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

/// Individual command item with `role="option"`.
///
/// Automatically hides when its text doesn't match the current filter.
#[component]
pub fn CommandItem(props: CommandItemProps) -> Element {
    let ctx = use_context::<Signal<CommandCtx>>();
    let on_select = use_context::<Callback<String>>();

    let filter = ctx.read().filter_text.clone();
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
            "data-slot": "command-item",
            "data-value": props.value.clone(),
            "data-disabled": props.disabled.then_some("true"),
            role: "option",
            class: props.class,
            onclick: {
                let value = props.value.clone();
                move |_| {
                    if !props.disabled {
                        on_select.call(value.clone());
                    }
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CommandSeparator
// ---------------------------------------------------------------------------

/// Props for [`CommandSeparator`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandSeparatorProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Visual separator between command groups.
#[component]
pub fn CommandSeparator(props: CommandSeparatorProps) -> Element {
    rsx! {
        div {
            "data-slot": "command-separator",
            role: "separator",
            class: props.class,
            ..props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// CommandShortcut
// ---------------------------------------------------------------------------

/// Props for [`CommandShortcut`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandShortcutProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Keyboard shortcut display for command items.
#[component]
pub fn CommandShortcut(props: CommandShortcutProps) -> Element {
    rsx! {
        span {
            "data-slot": "command-shortcut",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}
