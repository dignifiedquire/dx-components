//! Styled accordion matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::accordion` primitives with
//! Tailwind classes, focus-visible rings, and a chevron icon — matching
//! the shadcn/ui accordion component 1:1.

use dioxus::prelude::*;
use dioxus_primitives::accordion as primitives;
pub use dioxus_primitives::accordion::{AccordionType, Direction, Orientation};
use dx_icons_tabler::IconChevronDown;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Accordion (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`Accordion`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    /// The type of accordion — single or multiple.
    #[props(default)]
    pub r#type: AccordionType,

    /// The controlled value of the accordion — a list of open item values.
    #[props(default)]
    pub value: ReadSignal<Option<Vec<String>>>,

    /// The default open item values when uncontrolled.
    #[props(default)]
    pub default_value: Vec<String>,

    /// Callback fired when the open items change.
    #[props(default)]
    pub on_value_change: Callback<Vec<String>>,

    /// Whether the open item can be collapsed in single mode.
    #[props(default)]
    pub collapsible: ReadSignal<bool>,

    /// Set whether the accordion is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The text direction. Affects horizontal keyboard navigation.
    #[props(default)]
    pub dir: ReadSignal<Direction>,

    /// The orientation of the accordion.
    #[props(default)]
    pub orientation: ReadSignal<Orientation>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the accordion.
    pub children: Element,
}

/// Styled Accordion root.
///
/// shadcn does not add any classes to the root — it just passes props through.
/// Width is controlled by the consumer (e.g. `class: "w-full"`), not baked in.
#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    rsx! {
        primitives::Accordion {
            r#type: props.r#type,
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            collapsible: props.collapsible,
            disabled: props.disabled,
            dir: props.dir,
            orientation: props.orientation,
            class: props.class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AccordionItem (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`AccordionItem`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionItemProps {
    /// A unique string value for this accordion item.
    pub value: String,

    /// Whether the accordion item is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Keep content mounted when closed (maps to Radix's `forceMount`).
    #[props(default)]
    pub keep_mounted: ReadSignal<bool>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to extend the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the accordion item.
    pub children: Element,
}

/// Styled AccordionItem — adds `border-b last:border-b-0` (matching shadcn).
#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    let class = tw_merge!("border-b last:border-b-0", props.class);

    rsx! {
        primitives::AccordionItem {
            value: props.value,
            disabled: props.disabled,
            keep_mounted: props.keep_mounted,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AccordionTrigger (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`AccordionTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionTriggerProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to extend the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the accordion trigger element.
    pub children: Element,
}

/// Styled AccordionTrigger — matches shadcn exactly:
/// - Wraps in `<h3 class="flex">` (AccordionHeader)
/// - Adds focus-visible ring, disabled opacity, chevron icon
/// - Chevron rotates 180° when `data-state="open"`
#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    let class = tw_merge!(
        "flex flex-1 items-start justify-between gap-4 rounded-md py-4 text-left text-sm font-medium transition-all outline-none hover:underline focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 [&[data-state=open]>svg]:rotate-180",
        props.class,
    );

    rsx! {
        primitives::AccordionHeader { class: "flex",
            primitives::AccordionTrigger {
                class: class,
                "data-slot": "accordion-trigger",
                attributes: props.attributes,

                {props.children}

                IconChevronDown { class: "pointer-events-none size-4 shrink-0 translate-y-0.5 text-muted-foreground transition-transform duration-200" }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AccordionContent (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`AccordionContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionContentProps {
    /// Additional Tailwind classes to apply to the **inner** content wrapper
    /// (matching shadcn where className goes on the inner `<div>`, not the
    /// animated outer element).
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to extend the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the accordion content element.
    pub children: Element,
}

/// Styled AccordionContent — matches shadcn exactly:
/// - Outer element: fixed animation classes (`animate-accordion-up/down`)
/// - Inner div: `pt-0 pb-4` + consumer's `className`
#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    let inner_class = tw_merge!("pt-0 pb-4", props.class);

    rsx! {
        primitives::AccordionContent {
            class: "overflow-hidden text-sm data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down",
            "data-slot": "accordion-content",
            attributes: props.attributes,

            div { class: inner_class,
                {props.children}
            }
        }
    }
}
