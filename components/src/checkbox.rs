//! Styled checkbox matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::checkbox` primitives with
//! Tailwind classes and a check icon — matching the shadcn/ui checkbox
//! component 1:1.
//!
//! Unlike the primitive, the styled [`Checkbox`] composes the indicator
//! and icon internally — the consumer does not need to render them.

use dioxus::prelude::*;
use dioxus_primitives::checkbox as primitives;
pub use dioxus_primitives::checkbox::CheckedState;
use dx_icons_lucide::IconCheck;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Checkbox (styled — composes indicator + icon internally)
// ---------------------------------------------------------------------------

/// The props for the styled [`Checkbox`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    /// The controlled checked state.
    #[props(default)]
    pub checked: ReadSignal<Option<CheckedState>>,

    /// The default checked state when uncontrolled.
    #[props(default = CheckedState::Unchecked)]
    pub default_checked: CheckedState,

    /// Whether the checkbox is required in a form.
    #[props(default)]
    pub required: bool,

    /// Whether the checkbox is disabled.
    #[props(default)]
    pub disabled: bool,

    /// The name for form submission.
    #[props(default)]
    pub name: Option<String>,

    /// The value for form submission. Defaults to `"on"`.
    #[props(default = "on".to_string())]
    pub value: String,

    /// Callback fired when the checked state changes.
    #[props(default)]
    pub on_checked_change: Callback<CheckedState>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the checkbox element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled Checkbox — matches shadcn exactly.
///
/// Composes the primitive checkbox + indicator + check icon internally.
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let class = tw_merge!(
        "peer size-4 shrink-0 rounded-[4px] border border-input shadow-xs transition-shadow outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 data-[state=checked]:border-primary data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground dark:bg-input/30 dark:aria-invalid:ring-destructive/40 dark:data-[state=checked]:bg-primary",
        props.class,
    );

    rsx! {
        primitives::Checkbox {
            checked: props.checked,
            default_checked: props.default_checked,
            required: props.required,
            disabled: props.disabled,
            name: props.name,
            value: props.value,
            on_checked_change: props.on_checked_change,
            class: class,
            attributes: props.attributes,

            primitives::CheckboxIndicator {
                class: "grid place-content-center text-current transition-none",
                IconCheck { class: "size-3.5" }
            }
        }
    }
}
