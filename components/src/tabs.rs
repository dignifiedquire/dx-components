//! Styled tabs matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::tabs` primitives with
//! Tailwind classes — matching the shadcn/ui tabs component 1:1.

use dioxus::prelude::*;
pub use dioxus_primitives::direction::{Direction, Orientation};
use dioxus_primitives::tabs as primitives;
pub use dioxus_primitives::tabs::ActivationMode;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// TabsListVariant
// ---------------------------------------------------------------------------

/// Visual variant of the tabs list — matches shadcn's `tabsListVariants`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TabsListVariant {
    #[default]
    Default,
    Line,
}

// ---------------------------------------------------------------------------
// Tabs (styled root)
// ---------------------------------------------------------------------------

/// The props for the styled [`Tabs`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// The controlled value of the active tab.
    #[props(default)]
    pub value: ReadSignal<Option<String>>,

    /// The default active tab when uncontrolled.
    #[props(default)]
    pub default_value: String,

    /// Callback fired when the active tab changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// The orientation of the tabs.
    #[props(default = Orientation::Horizontal)]
    pub orientation: Orientation,

    /// Text direction for RTL support.
    #[props(default)]
    pub dir: Direction,

    /// Activation mode — automatic (on focus) or manual (on click).
    #[props(default)]
    pub activation_mode: ActivationMode,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tabs.
    pub children: Element,
}

/// Styled Tabs root — matches shadcn exactly.
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let class = tw_merge!(
        "group/tabs flex gap-2 data-[orientation=horizontal]:flex-col",
        props.class,
    );

    rsx! {
        primitives::Tabs {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            orientation: props.orientation,
            dir: props.dir,
            activation_mode: props.activation_mode,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// TabsList (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`TabsList`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TabsListProps {
    /// Visual variant of the tabs list.
    #[props(default)]
    pub variant: TabsListVariant,

    /// Whether keyboard navigation loops.
    #[props(default = true)]
    pub r#loop: bool,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the list element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tabs list.
    pub children: Element,
}

/// Styled TabsList — matches shadcn with default/line variant support.
#[component]
pub fn TabsList(props: TabsListProps) -> Element {
    let variant_class = match props.variant {
        TabsListVariant::Default => "bg-muted",
        TabsListVariant::Line => "gap-1 bg-transparent",
    };

    let variant_str = match props.variant {
        TabsListVariant::Default => "default",
        TabsListVariant::Line => "line",
    };

    let class = tw_merge!(
        "inline-flex w-fit items-center justify-center rounded-lg p-[3px] text-muted-foreground group-data-[orientation=horizontal]/tabs:h-9 group-data-[orientation=vertical]/tabs:h-fit group-data-[orientation=vertical]/tabs:flex-col data-[variant=line]:rounded-none",
        variant_class,
        props.class,
    );

    rsx! {
        primitives::TabsList {
            r#loop: props.r#loop,
            class: class,
            "data-variant": variant_str,
            "data-slot": "tabs-list",
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// TabsTrigger (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`TabsTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TabsTriggerProps {
    /// A unique string value for this tab trigger.
    pub value: String,

    /// Whether the tab trigger is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tab trigger.
    pub children: Element,
}

/// Styled TabsTrigger — matches shadcn exactly with state-aware classes.
#[component]
pub fn TabsTrigger(props: TabsTriggerProps) -> Element {
    let class = tw_merge!(
        // Base
        "relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center gap-1.5 rounded-md border border-transparent px-2 py-1 text-sm font-medium whitespace-nowrap text-foreground/60 transition-all group-data-[orientation=vertical]/tabs:w-full group-data-[orientation=vertical]/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 group-data-[variant=default]/tabs-list:data-[state=active]:shadow-sm group-data-[variant=line]/tabs-list:data-[state=active]:shadow-none dark:text-muted-foreground dark:hover:text-foreground [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        // Line variant overrides
        "group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-[state=active]:bg-transparent dark:group-data-[variant=line]/tabs-list:data-[state=active]:border-transparent dark:group-data-[variant=line]/tabs-list:data-[state=active]:bg-transparent",
        // Active state
        "data-[state=active]:bg-background data-[state=active]:text-foreground dark:data-[state=active]:border-input dark:data-[state=active]:bg-input/30 dark:data-[state=active]:text-foreground",
        // Line indicator (::after pseudo)
        "after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-[orientation=horizontal]/tabs:after:inset-x-0 group-data-[orientation=horizontal]/tabs:after:bottom-[-5px] group-data-[orientation=horizontal]/tabs:after:h-0.5 group-data-[orientation=vertical]/tabs:after:inset-y-0 group-data-[orientation=vertical]/tabs:after:-right-1 group-data-[orientation=vertical]/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-[state=active]:after:opacity-100",
        props.class,
    );

    rsx! {
        primitives::TabsTrigger {
            value: props.value,
            disabled: props.disabled,
            class: class,
            "data-slot": "tabs-trigger",
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// TabsContent (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`TabsContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TabsContentProps {
    /// A unique string value matching a tab trigger.
    pub value: String,

    /// When true, content stays mounted even when inactive.
    #[props(default)]
    pub force_mount: bool,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tab content.
    pub children: Element,
}

/// Styled TabsContent — matches shadcn exactly.
#[component]
pub fn TabsContent(props: TabsContentProps) -> Element {
    let class = tw_merge!("flex-1 outline-none", props.class);

    rsx! {
        primitives::TabsContent {
            value: props.value,
            force_mount: props.force_mount,
            class: class,
            "data-slot": "tabs-content",
            attributes: props.attributes,
            {props.children}
        }
    }
}
