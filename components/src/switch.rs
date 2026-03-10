//! Styled switch matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::switch` primitives with
//! Tailwind classes — matching the shadcn/ui switch component 1:1.
//!
//! Unlike the primitive, the styled [`Switch`] composes the thumb
//! internally — the consumer does not need to render it.

use dioxus::prelude::*;
use dioxus_primitives::switch as primitives;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// SwitchSize
// ---------------------------------------------------------------------------

/// Size variants for the styled switch.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum SwitchSize {
    /// Default switch size.
    #[default]
    Default,
    /// Small switch size.
    Sm,
}

impl SwitchSize {
    fn as_data_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
        }
    }
}

// ---------------------------------------------------------------------------
// Switch (styled — composes thumb internally)
// ---------------------------------------------------------------------------

/// The props for the styled [`Switch`] component.
#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    /// The controlled checked state.
    #[props(default)]
    pub checked: ReadSignal<Option<bool>>,

    /// The default checked state when uncontrolled.
    #[props(default)]
    pub default_checked: bool,

    /// Whether the switch is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether the switch is required in a form.
    #[props(default)]
    pub required: bool,

    /// The name attribute for form submission.
    #[props(default)]
    pub name: Option<String>,

    /// The value attribute for form submission. Defaults to `"on"`.
    #[props(default = "on".to_string())]
    pub value: String,

    /// Callback fired when the checked state changes.
    #[props(default)]
    pub on_checked_change: Callback<bool>,

    /// Size variant.
    #[props(default)]
    pub size: SwitchSize,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the switch element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled Switch — matches shadcn exactly.
///
/// Composes the primitive switch + thumb internally.
#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let class = tw_merge!(
        "peer group/switch cursor-pointer inline-flex shrink-0 items-center rounded-full border border-transparent shadow-xs transition-all outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 data-[size=default]:h-[1.15rem] data-[size=default]:w-8 data-[size=sm]:h-3.5 data-[size=sm]:w-6 data-[state=checked]:bg-primary data-[state=unchecked]:bg-input dark:data-[state=unchecked]:bg-input/80",
        props.class,
    );

    rsx! {
        primitives::Switch {
            checked: props.checked,
            default_checked: props.default_checked,
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            value: props.value,
            on_checked_change: props.on_checked_change,
            class: class,
            "data-size": props.size.as_data_attr(),
            attributes: props.attributes,

            primitives::SwitchThumb {
                class: "pointer-events-none block rounded-full bg-background ring-0 transition-transform group-data-[size=default]/switch:size-4 group-data-[size=sm]/switch:size-3 data-[state=checked]:translate-x-[calc(100%-2px)] data-[state=unchecked]:translate-x-0 dark:data-[state=checked]:bg-primary-foreground dark:data-[state=unchecked]:bg-foreground",
            }
        }
    }
}
