//! Styled InputOTP matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::input_otp` primitive with
//! Tailwind classes — matching the shadcn/ui input-otp component.

use dioxus::prelude::*;
use dioxus_primitives::input_otp as primitives;
use tailwind_fuse::*;

// Re-export context accessor and types
pub use primitives::{InputOTPCtx, matches_char_pattern, use_input_otp};

// ---------------------------------------------------------------------------
// InputOTP (root)
// ---------------------------------------------------------------------------

/// The props for the styled [`InputOTP`] component.
#[derive(Props, Clone, PartialEq)]
pub struct InputOTPProps {
    /// Maximum number of characters.
    pub max_length: usize,

    /// Current value (controlled).
    #[props(default)]
    pub value: String,

    /// Callback when value changes.
    #[props(default)]
    pub on_change: Callback<String>,

    /// Callback when all slots are filled.
    #[props(default)]
    pub on_complete: Callback<String>,

    /// Whether the input is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Regex pattern string for character validation.
    #[props(default)]
    pub pattern: Option<String>,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled InputOTP — matches shadcn.
#[component]
pub fn InputOTP(props: InputOTPProps) -> Element {
    let class = tw_merge!(
        "flex items-center gap-2 has-disabled:opacity-50",
        props.class,
    );

    rsx! {
        primitives::InputOTP {
            max_length: props.max_length,
            value: props.value,
            on_change: props.on_change,
            on_complete: props.on_complete,
            disabled: props.disabled,
            pattern: props.pattern,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// InputOTPGroup
// ---------------------------------------------------------------------------

/// The props for the styled [`InputOTPGroup`] component.
#[derive(Props, Clone, PartialEq)]
pub struct InputOTPGroupProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled InputOTPGroup — matches shadcn `flex items-center`.
#[component]
pub fn InputOTPGroup(props: InputOTPGroupProps) -> Element {
    let class = tw_merge!("flex items-center", props.class);

    rsx! {
        primitives::InputOTPGroup {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// InputOTPSlot
// ---------------------------------------------------------------------------

/// The props for the styled [`InputOTPSlot`] component.
#[derive(Props, Clone, PartialEq)]
pub struct InputOTPSlotProps {
    /// The slot's position (0-indexed).
    pub index: usize,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled InputOTPSlot — matches shadcn slot styling.
#[component]
pub fn InputOTPSlot(props: InputOTPSlotProps) -> Element {
    let class = tw_merge!(
        "relative flex h-9 w-9 items-center justify-center border-y border-r border-input text-sm shadow-xs transition-all outline-none first:rounded-l-md first:border-l last:rounded-r-md aria-invalid:border-destructive data-[active=true]:z-10 data-[active=true]:border-ring data-[active=true]:ring-[3px] data-[active=true]:ring-ring/50 data-[active=true]:aria-invalid:border-destructive data-[active=true]:aria-invalid:ring-destructive/20 dark:bg-input/30 dark:data-[active=true]:aria-invalid:ring-destructive/40",
        props.class,
    );

    rsx! {
        primitives::InputOTPSlot {
            index: props.index,
            class: class,
            attributes: props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// InputOTPSeparator
// ---------------------------------------------------------------------------

/// The props for the styled [`InputOTPSeparator`] component.
#[derive(Props, Clone, PartialEq)]
pub struct InputOTPSeparatorProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Custom separator content.
    #[props(default)]
    pub children: Element,
}

/// Styled InputOTPSeparator — matches shadcn.
#[component]
pub fn InputOTPSeparator(props: InputOTPSeparatorProps) -> Element {
    let has_children = props.children != Ok(VNode::placeholder());

    if has_children {
        rsx! {
            primitives::InputOTPSeparator {
                class: props.class,
                attributes: props.attributes,
                {props.children}
            }
        }
    } else {
        rsx! {
            primitives::InputOTPSeparator {
                class: props.class,
                attributes: props.attributes,
            }
        }
    }
}
