//! Styled radio group matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::radio_group` primitives with
//! Tailwind classes — matching the shadcn/ui radio-group component 1:1.
//!
//! Key difference from primitives: `RadioGroupItem` composes the indicator
//! and circle icon internally, matching shadcn's structure where the consumer
//! does not manually render the indicator.

use dioxus::prelude::*;
pub use dioxus_primitives::direction::{Direction, Orientation};
use dioxus_primitives::radio_group as primitives;
use dx_icons_lucide::IconCircle;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// RadioGroup (styled root)
// ---------------------------------------------------------------------------

/// The props for the styled [`RadioGroup`] component.
#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
    /// The controlled value of the selected radio item.
    #[props(default)]
    pub value: ReadSignal<Option<String>>,

    /// The default selected value when uncontrolled.
    #[props(default)]
    pub default_value: String,

    /// Callback fired when the selected value changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Whether the entire group is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether the group is required for form submission.
    #[props(default)]
    pub required: bool,

    /// Form input name for hidden radio inputs.
    #[props(default)]
    pub name: Option<String>,

    /// The orientation of the radio group.
    #[props(default)]
    pub orientation: Orientation,

    /// Text direction for RTL support.
    #[props(default)]
    pub dir: Direction,

    /// Whether keyboard navigation loops.
    #[props(default = true)]
    pub r#loop: bool,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the radio group.
    pub children: Element,
}

/// Styled RadioGroup root — matches shadcn exactly.
#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let class = tw_merge!("grid gap-3", props.class);

    rsx! {
        primitives::RadioGroup {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            orientation: props.orientation,
            dir: props.dir,
            r#loop: props.r#loop,
            class: class,
            "data-slot": "radio-group",
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// RadioGroupItem (styled — composes indicator internally)
// ---------------------------------------------------------------------------

/// The props for the styled [`RadioGroupItem`] component.
///
/// Unlike the primitive, this component renders the indicator and circle icon
/// internally — matching shadcn's `RadioGroupItem` which does not accept
/// children for the indicator.
#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupItemProps {
    /// A unique string value for this radio item.
    pub value: String,

    /// Whether the radio item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled RadioGroupItem — matches shadcn exactly:
/// - Circular radio button with focus ring and state styling
/// - Composes indicator + `CircleIcon` from lucide internally
#[component]
pub fn RadioGroupItem(props: RadioGroupItemProps) -> Element {
    let class = tw_merge!(
        "aspect-square size-4 shrink-0 rounded-full border border-input text-primary shadow-xs transition-[color,box-shadow] outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:bg-input/30 dark:aria-invalid:ring-destructive/40",
        props.class,
    );

    rsx! {
        primitives::RadioGroupItem {
            value: props.value,
            disabled: props.disabled,
            class: class,
            "data-slot": "radio-group-item",
            attributes: props.attributes,

            primitives::RadioGroupIndicator {
                class: "relative flex items-center justify-center",
                "data-slot": "radio-group-indicator",
                IconCircle { class: "absolute top-1/2 left-1/2 size-2 -translate-x-1/2 -translate-y-1/2 fill-primary" }
            }
        }
    }
}
