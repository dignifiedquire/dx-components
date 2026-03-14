//! Styled Command matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::command` primitive with
//! Tailwind classes — matching the shadcn/ui command component.

use dioxus::prelude::*;
use dioxus_primitives::command as primitives;
use tailwind_fuse::*;

// Re-export context and types
pub use primitives::{CommandCtx, use_command};

// ---------------------------------------------------------------------------
// Command (root)
// ---------------------------------------------------------------------------

/// Props for the styled [`Command`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandProps {
    /// Callback when item is selected.
    #[props(default)]
    pub on_select: Callback<String>,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled Command root — matches shadcn.
#[component]
pub fn Command(props: CommandProps) -> Element {
    let class = tw_merge!(
        "flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground",
        props.class,
    );

    rsx! {
        primitives::Command {
            on_select: props.on_select,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CommandInput
// ---------------------------------------------------------------------------

/// Props for the styled [`CommandInput`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandInputProps {
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

/// Styled CommandInput — matches shadcn.
#[component]
pub fn CommandInput(props: CommandInputProps) -> Element {
    let class = tw_merge!(
        "flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-hidden placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50",
        props.class,
    );

    rsx! {
        primitives::CommandInput {
            placeholder: props.placeholder,
            class: class,
            attributes: props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// CommandList
// ---------------------------------------------------------------------------

/// Props for the styled [`CommandList`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandListProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled CommandList — matches shadcn.
#[component]
pub fn CommandList(props: CommandListProps) -> Element {
    let class = tw_merge!(
        "max-h-[300px] scroll-py-1 overflow-x-hidden overflow-y-auto",
        props.class,
    );

    rsx! {
        primitives::CommandList {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CommandEmpty
// ---------------------------------------------------------------------------

/// Props for the styled [`CommandEmpty`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandEmptyProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled CommandEmpty — matches shadcn.
#[component]
pub fn CommandEmpty(props: CommandEmptyProps) -> Element {
    let class = tw_merge!("py-6 text-center text-sm", props.class);

    rsx! {
        primitives::CommandEmpty {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CommandGroup
// ---------------------------------------------------------------------------

/// Props for the styled [`CommandGroup`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandGroupProps {
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

/// Styled CommandGroup — matches shadcn.
#[component]
pub fn CommandGroup(props: CommandGroupProps) -> Element {
    let class = tw_merge!("overflow-hidden p-1 text-foreground", props.class);

    rsx! {
        primitives::CommandGroup {
            heading: props.heading,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CommandItem
// ---------------------------------------------------------------------------

/// Props for the styled [`CommandItem`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandItemProps {
    /// The value for filtering and selection.
    pub value: String,

    /// Display text for filtering.
    #[props(default)]
    pub text_value: Option<String>,

    /// Whether disabled.
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

/// Styled CommandItem — matches shadcn.
#[component]
pub fn CommandItem(props: CommandItemProps) -> Element {
    let class = tw_merge!(
        "relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none hover:bg-accent hover:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        props.class,
    );

    rsx! {
        primitives::CommandItem {
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
// CommandSeparator
// ---------------------------------------------------------------------------

/// Props for the styled [`CommandSeparator`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandSeparatorProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled CommandSeparator — matches shadcn.
#[component]
pub fn CommandSeparator(props: CommandSeparatorProps) -> Element {
    let class = tw_merge!("-mx-1 h-px bg-border", props.class);

    rsx! {
        primitives::CommandSeparator {
            class: class,
            attributes: props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// CommandShortcut
// ---------------------------------------------------------------------------

/// Props for the styled [`CommandShortcut`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandShortcutProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled CommandShortcut — matches shadcn.
#[component]
pub fn CommandShortcut(props: CommandShortcutProps) -> Element {
    let class = tw_merge!(
        "ml-auto text-xs tracking-widest text-muted-foreground",
        props.class,
    );

    rsx! {
        primitives::CommandShortcut {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
