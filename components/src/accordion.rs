//! Styled accordion matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::accordion` primitives with
//! Tailwind classes, focus-visible rings, and a chevron icon — matching
//! the shadcn/ui accordion component 1:1.

use dioxus::prelude::*;
use dioxus_primitives::accordion as primitives;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Accordion (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`Accordion`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    /// Whether multiple accordion items are allowed to be open at once.
    #[props(default)]
    pub allow_multiple_open: ReadSignal<bool>,

    /// Set whether the accordion is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the accordion can be fully collapsed. Defaults to true.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub collapsible: ReadSignal<bool>,

    /// Whether the accordion is horizontal.
    #[props(default)]
    pub horizontal: ReadSignal<bool>,

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
            allow_multiple_open: props.allow_multiple_open,
            disabled: props.disabled,
            collapsible: props.collapsible,
            horizontal: props.horizontal,
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

    /// Whether this accordion item should be opened by default.
    #[props(default)]
    pub default_open: bool,

    /// The index of the accordion item within the [`Accordion`].
    pub index: usize,

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
            default_open: props.default_open,
            index: props.index,
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
                attributes: props.attributes,

                {props.children}

                // Inline chevron SVG matching shadcn's ChevronDownIcon.
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "24",
                    height: "24",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    class: "pointer-events-none size-4 shrink-0 translate-y-0.5 text-muted-foreground transition-transform duration-200",
                    path { d: "m6 9 6 6 6-6" }
                }
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
            attributes: props.attributes,

            div { class: inner_class,
                {props.children}
            }
        }
    }
}
