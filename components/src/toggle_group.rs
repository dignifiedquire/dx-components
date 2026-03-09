//! Styled toggle group matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::toggle_group` primitives with
//! Tailwind classes — matching shadcn's toggle-group + toggle components 1:1.
//!
//! Uses Dioxus context to pass variant/size from group to items, matching
//! shadcn's `ToggleGroupContext` pattern.

use dioxus::prelude::*;
pub use dioxus_primitives::direction::{Direction, Orientation};
use dioxus_primitives::toggle_group as primitives;
pub use dioxus_primitives::toggle_group::ToggleGroupType;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Toggle variant/size enums (from shadcn toggle.tsx CVA)
// ---------------------------------------------------------------------------

/// Visual variant of a toggle — matches shadcn's `toggleVariants`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

/// Size of a toggle — matches shadcn's `toggleVariants`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ToggleSize {
    #[default]
    Default,
    Sm,
    Lg,
}

// ---------------------------------------------------------------------------
// Context (variant + size passed from group to items)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct ToggleGroupContext {
    variant: ToggleVariant,
    size: ToggleSize,
}

// ---------------------------------------------------------------------------
// ToggleGroup (styled root)
// ---------------------------------------------------------------------------

/// The props for the styled [`ToggleGroup`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupProps {
    /// Selection mode — single or multiple.
    #[props(default)]
    pub type_: ToggleGroupType,

    /// The controlled value of selected items.
    #[props(default)]
    pub value: ReadSignal<Option<Vec<String>>>,

    /// The default selected values when uncontrolled.
    #[props(default)]
    pub default_value: Vec<String>,

    /// Callback fired when the selected values change.
    #[props(default)]
    pub on_value_change: Callback<Vec<String>>,

    /// Whether the entire group is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether roving focus is enabled.
    #[props(default = true)]
    pub roving_focus: bool,

    /// The orientation of the toggle group.
    #[props(default = Orientation::Horizontal)]
    pub orientation: Orientation,

    /// Text direction for RTL support.
    #[props(default)]
    pub dir: Direction,

    /// Whether keyboard navigation loops.
    #[props(default = true)]
    pub r#loop: bool,

    /// Visual variant applied to all items.
    #[props(default)]
    pub variant: ToggleVariant,

    /// Size applied to all items.
    #[props(default)]
    pub size: ToggleSize,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the toggle group.
    pub children: Element,
}

/// Styled ToggleGroup root — matches shadcn exactly.
#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
    use_context_provider(|| ToggleGroupContext {
        variant: props.variant,
        size: props.size,
    });

    let variant_str = match props.variant {
        ToggleVariant::Default => "default",
        ToggleVariant::Outline => "outline",
    };

    let size_str = match props.size {
        ToggleSize::Default => "default",
        ToggleSize::Sm => "sm",
        ToggleSize::Lg => "lg",
    };

    let class = tw_merge!(
        "group/toggle-group flex w-fit items-center rounded-md",
        props.class,
    );

    rsx! {
        primitives::ToggleGroup {
            type_: props.type_,
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            roving_focus: props.roving_focus,
            orientation: props.orientation,
            dir: props.dir,
            r#loop: props.r#loop,
            class: class,
            "data-slot": "toggle-group",
            "data-variant": variant_str,
            "data-size": size_str,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ToggleGroupItem (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`ToggleGroupItem`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupItemProps {
    /// A unique string value for this toggle item.
    pub value: String,

    /// Whether the toggle item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Override variant for this specific item.
    #[props(default)]
    pub variant: Option<ToggleVariant>,

    /// Override size for this specific item.
    #[props(default)]
    pub size: Option<ToggleSize>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the toggle item.
    pub children: Element,
}

/// Styled ToggleGroupItem — matches shadcn exactly with toggle CVA variants.
#[component]
pub fn ToggleGroupItem(props: ToggleGroupItemProps) -> Element {
    let ctx = use_context::<ToggleGroupContext>();
    let variant = props.variant.unwrap_or(ctx.variant);
    let size = props.size.unwrap_or(ctx.size);

    let variant_class = match variant {
        ToggleVariant::Default => "bg-transparent",
        ToggleVariant::Outline => {
            "border border-input bg-transparent shadow-xs hover:bg-accent hover:text-accent-foreground"
        }
    };

    let size_class = match size {
        ToggleSize::Default => "h-9 min-w-9 px-2",
        ToggleSize::Sm => "h-8 min-w-8 px-1.5",
        ToggleSize::Lg => "h-10 min-w-10 px-2.5",
    };

    let variant_str = match variant {
        ToggleVariant::Default => "default",
        ToggleVariant::Outline => "outline",
    };

    let size_str = match size {
        ToggleSize::Default => "default",
        ToggleSize::Sm => "sm",
        ToggleSize::Lg => "lg",
    };

    let class = tw_merge!(
        // Base toggle classes (from toggle.tsx CVA)
        "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-[color,box-shadow] outline-none hover:bg-muted hover:text-muted-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 data-[state=on]:bg-accent data-[state=on]:text-accent-foreground dark:aria-invalid:ring-destructive/40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        variant_class,
        size_class,
        // Group-specific overrides (items touching, no rounded corners except first/last)
        "w-auto min-w-0 shrink-0 focus:z-10 focus-visible:z-10 rounded-none shadow-none first:rounded-l-md last:rounded-r-md data-[variant=outline]:border-l-0 data-[variant=outline]:first:border-l",
        props.class,
    );

    rsx! {
        primitives::ToggleGroupItem {
            value: props.value,
            disabled: props.disabled,
            class: class,
            "data-slot": "toggle-group-item",
            "data-variant": variant_str,
            "data-size": size_str,
            attributes: props.attributes,
            {props.children}
        }
    }
}
