//! Input OTP primitive — one-time password input.
//!
//! A composable OTP/verification-code input with a hidden native `<input>`
//! and individual visual slot cells. Inspired by the `input-otp` library
//! used in shadcn/ui.
//!
//! ## Architecture
//!
//! - [`InputOTP`] — Root component with hidden input and context provider
//! - [`InputOTPGroup`] — Layout wrapper for grouping slots
//! - [`InputOTPSlot`] — Individual character display cell
//! - [`InputOTPSeparator`] — Visual separator between groups
//!
//! ## Example
//!
//! ```rust,no_run
//! # use dioxus::prelude::*;
//! # use dioxus_primitives::input_otp::*;
//! fn Demo() -> Element {
//!     let mut value = use_signal(String::new);
//!
//!     rsx! {
//!         InputOTP {
//!             max_length: 6,
//!             value: value(),
//!             on_change: move |v: String| value.set(v),
//!             InputOTPGroup {
//!                 InputOTPSlot { index: 0 }
//!                 InputOTPSlot { index: 1 }
//!                 InputOTPSlot { index: 2 }
//!             }
//!             InputOTPSeparator {}
//!             InputOTPGroup {
//!                 InputOTPSlot { index: 3 }
//!                 InputOTPSlot { index: 4 }
//!                 InputOTPSlot { index: 5 }
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

/// Context shared between InputOTP root and its slots.
#[derive(Clone, Debug)]
pub struct InputOTPCtx {
    /// Current OTP value.
    pub value: String,
    /// Maximum number of characters.
    pub max_length: usize,
    /// Currently active (focused) slot index.
    pub active_index: Option<usize>,
    /// Whether the input is disabled.
    pub disabled: bool,
}

impl InputOTPCtx {
    /// Get the character at a given slot index.
    pub fn char_at(&self, index: usize) -> Option<char> {
        self.value.chars().nth(index)
    }

    /// Whether the given slot is the active one (has fake caret).
    pub fn is_active(&self, index: usize) -> bool {
        self.active_index == Some(index)
    }

    /// Whether the given slot has a fake caret (active and at the input position).
    pub fn has_fake_caret(&self, index: usize) -> bool {
        self.is_active(index)
    }
}

/// Access the nearest [`InputOTP`] context.
pub fn use_input_otp() -> InputOTPCtx {
    use_context::<Signal<InputOTPCtx>>().cloned()
}

// ---------------------------------------------------------------------------
// InputOTP (root)
// ---------------------------------------------------------------------------

/// Props for [`InputOTP`].
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

    /// Regex pattern string for character validation (e.g., `"[0-9]"` for digits only).
    #[props(default)]
    pub pattern: Option<String>,

    /// Additional CSS classes for the container.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes on the container div.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (groups, slots, separators).
    pub children: Element,
}

/// Root OTP input component.
///
/// Renders a hidden `<input>` to capture keyboard input and provides
/// context to child [`InputOTPSlot`] components. The visual slots are
/// purely presentational — all input handling goes through the hidden element.
#[component]
pub fn InputOTP(props: InputOTPProps) -> Element {
    let id = crate::use_unique_id();
    let mut focused = use_signal(|| false);

    // Active index = current value length (next slot to fill), clamped to max_length - 1
    let active_index = if focused() {
        Some(props.value.len().min(props.max_length.saturating_sub(1)))
    } else {
        None
    };

    let ctx = InputOTPCtx {
        value: props.value.clone(),
        max_length: props.max_length,
        active_index,
        disabled: props.disabled,
    };

    use_context_provider(|| Signal::new(ctx.clone()));

    // Update context when props change
    let mut ctx_signal = use_context::<Signal<InputOTPCtx>>();
    if *ctx_signal.peek() != ctx {
        ctx_signal.set(ctx);
    }

    let max_length = props.max_length;
    let pattern = props.pattern.clone();
    let current_value = props.value.clone();

    rsx! {
        div {
            "data-slot": "input-otp",
            "data-disabled": if props.disabled { "true" },
            class: props.class,
            style: "position: relative; display: flex; align-items: center; gap: 0.5rem;",
            ..props.attributes,

            // Hidden input captures all keyboard interaction
            input {
                id: id(),
                "data-slot": "input-otp-input",
                r#type: "text",
                inputmode: "numeric",
                autocomplete: "one-time-code",
                maxlength: max_length as i64,
                value: current_value.clone(),
                disabled: props.disabled,
                spellcheck: "false",
                style: "position: absolute; inset: 0; width: 100%; height: 100%; opacity: 0; cursor: pointer; z-index: 1;",

                onfocus: move |_| {
                    focused.set(true);
                },
                onblur: move |_| {
                    focused.set(false);
                },
                oninput: {
                    let on_change = props.on_change;
                    let on_complete = props.on_complete;
                    let pattern = pattern.clone();
                    move |e: FormEvent| {
                        let mut new_val = e.value();

                        // Apply pattern filter if specified
                        if let Some(ref pat) = pattern {
                            // Simple single-char pattern like "[0-9]" or "[a-zA-Z0-9]"
                            new_val = new_val
                                .chars()
                                .filter(|c| {
                                    matches_char_pattern(pat, *c)
                                })
                                .collect();
                        }

                        // Truncate to max length
                        if new_val.len() > max_length {
                            new_val = new_val[..max_length].to_string();
                        }

                        on_change.call(new_val.clone());

                        if new_val.len() == max_length {
                            on_complete.call(new_val);
                        }
                    }
                },
            }

            // Visual slots
            {props.children}
        }
    }
}

/// Simple character-class pattern matcher for patterns like `[0-9]`, `[a-zA-Z0-9]`.
pub fn matches_char_pattern(pattern: &str, c: char) -> bool {
    let pattern = pattern.trim();
    // Handle bracket-enclosed character class: [...]
    if pattern.starts_with('[') && pattern.ends_with(']') {
        let inner = &pattern[1..pattern.len() - 1];
        let mut chars = inner.chars().peekable();
        while let Some(ch) = chars.next() {
            if chars.peek() == Some(&'-') {
                chars.next(); // consume '-'
                if let Some(end) = chars.next() {
                    if c >= ch && c <= end {
                        return true;
                    }
                }
            } else if c == ch {
                return true;
            }
        }
        false
    } else {
        // Fallback: accept all
        true
    }
}

// ---------------------------------------------------------------------------
// InputOTPGroup
// ---------------------------------------------------------------------------

/// Props for [`InputOTPGroup`].
#[derive(Props, Clone, PartialEq)]
pub struct InputOTPGroupProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (slots).
    pub children: Element,
}

/// Groups OTP slots visually.
#[component]
pub fn InputOTPGroup(props: InputOTPGroupProps) -> Element {
    rsx! {
        div {
            "data-slot": "input-otp-group",
            style: "display: flex; align-items: center;",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// InputOTPSlot
// ---------------------------------------------------------------------------

/// Props for [`InputOTPSlot`].
#[derive(Props, Clone, PartialEq)]
pub struct InputOTPSlotProps {
    /// The slot's position in the OTP value (0-indexed).
    pub index: usize,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Individual character display cell.
///
/// Reads from [`InputOTPCtx`] to show the character at `index` and
/// display a caret when active.
#[component]
pub fn InputOTPSlot(props: InputOTPSlotProps) -> Element {
    let ctx = use_input_otp();
    let ch = ctx.char_at(props.index);
    let is_active = ctx.is_active(props.index);
    let has_caret = ctx.has_fake_caret(props.index);

    rsx! {
        div {
            "data-slot": "input-otp-slot",
            "data-active": is_active.then_some("true"),
            "data-filled": ch.is_some().then_some("true"),
            class: props.class,
            ..props.attributes,

            if let Some(c) = ch {
                "{c}"
            } else if has_caret {
                div {
                    "data-slot": "input-otp-caret",
                    style: "pointer-events: none;",
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// InputOTPSeparator
// ---------------------------------------------------------------------------

/// Props for [`InputOTPSeparator`].
#[derive(Props, Clone, PartialEq)]
pub struct InputOTPSeparatorProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Custom separator content. Defaults to a minus icon.
    #[props(default)]
    pub children: Element,
}

/// Visual separator between OTP slot groups.
#[component]
pub fn InputOTPSeparator(props: InputOTPSeparatorProps) -> Element {
    let has_children = props.children != Ok(VNode::placeholder());

    rsx! {
        div {
            "data-slot": "input-otp-separator",
            role: "separator",
            class: props.class,
            ..props.attributes,

            if has_children {
                {props.children}
            } else {
                // Default minus icon (matching shadcn's MinusIcon)
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
                    line { x1: "5", x2: "19", y1: "12", y2: "12" }
                }
            }
        }
    }
}

impl PartialEq for InputOTPCtx {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.max_length == other.max_length
            && self.active_index == other.active_index
            && self.disabled == other.disabled
    }
}
