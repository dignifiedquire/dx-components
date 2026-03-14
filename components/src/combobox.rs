//! Styled Combobox matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::combobox` primitive with
//! Tailwind classes — matching the shadcn/ui combobox component.

use dioxus::prelude::*;
use dioxus_primitives::combobox as primitives;
use tailwind_fuse::*;

// Re-export context and types
pub use primitives::{ComboboxCtx, use_combobox};

// ---------------------------------------------------------------------------
// Combobox (root)
// ---------------------------------------------------------------------------

/// Props for the styled [`Combobox`].
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

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled Combobox root — matches shadcn.
#[component]
pub fn Combobox(props: ComboboxProps) -> Element {
    let class = tw_merge!("relative", props.class);

    rsx! {
        primitives::Combobox {
            value: props.value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ComboboxInput
// ---------------------------------------------------------------------------

/// Props for the styled [`ComboboxInput`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxInputProps {
    /// Placeholder text.
    #[props(default)]
    pub placeholder: Option<String>,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled ComboboxInput — matches shadcn.
#[component]
pub fn ComboboxInput(props: ComboboxInputProps) -> Element {
    let class = tw_merge!(
        "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-base shadow-xs transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        props.class,
    );

    rsx! {
        primitives::ComboboxInput {
            placeholder: props.placeholder,
            class: class,
            attributes: props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// ComboboxContent
// ---------------------------------------------------------------------------

/// Props for the styled [`ComboboxContent`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxContentProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled ComboboxContent — matches shadcn.
#[component]
pub fn ComboboxContent(props: ComboboxContentProps) -> Element {
    let class = tw_merge!(
        "absolute left-0 top-full z-50 mt-1 w-full overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md",
        props.class,
    );

    rsx! {
        primitives::ComboboxContent {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ComboboxList
// ---------------------------------------------------------------------------

/// Props for the styled [`ComboboxList`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxListProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled ComboboxList — matches shadcn.
#[component]
pub fn ComboboxList(props: ComboboxListProps) -> Element {
    let class = tw_merge!("max-h-96 overflow-y-auto p-1", props.class,);

    rsx! {
        primitives::ComboboxList {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ComboboxItem
// ---------------------------------------------------------------------------

/// Props for the styled [`ComboboxItem`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxItemProps {
    /// The value of this item.
    pub value: String,

    /// Display text used for filtering.
    #[props(default)]
    pub text_value: Option<String>,

    /// Whether this item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled ComboboxItem — matches shadcn.
#[component]
pub fn ComboboxItem(props: ComboboxItemProps) -> Element {
    let class = tw_merge!(
        "relative flex w-full cursor-pointer items-center gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm outline-hidden select-none hover:bg-accent hover:text-accent-foreground aria-selected:bg-accent aria-selected:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        props.class,
    );

    rsx! {
        primitives::ComboboxItem {
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ComboboxEmpty
// ---------------------------------------------------------------------------

/// Props for the styled [`ComboboxEmpty`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxEmptyProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled ComboboxEmpty — matches shadcn.
#[component]
pub fn ComboboxEmpty(props: ComboboxEmptyProps) -> Element {
    let class = tw_merge!(
        "py-2 text-center text-sm text-muted-foreground",
        props.class,
    );

    rsx! {
        primitives::ComboboxEmpty {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ComboboxGroup
// ---------------------------------------------------------------------------

/// Props for the styled [`ComboboxGroup`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxGroupProps {
    /// Group heading text.
    #[props(default)]
    pub heading: Option<String>,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled ComboboxGroup — matches shadcn.
#[component]
pub fn ComboboxGroup(props: ComboboxGroupProps) -> Element {
    rsx! {
        primitives::ComboboxGroup {
            heading: props.heading,
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ComboboxSeparator
// ---------------------------------------------------------------------------

/// Props for the styled [`ComboboxSeparator`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxSeparatorProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled ComboboxSeparator — matches shadcn.
#[component]
pub fn ComboboxSeparator(props: ComboboxSeparatorProps) -> Element {
    let class = tw_merge!("-mx-1 my-1 h-px bg-border", props.class);

    rsx! {
        primitives::ComboboxSeparator {
            class: class,
            attributes: props.attributes,
        }
    }
}
