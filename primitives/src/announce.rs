//! Announce primitive — matches `@radix-ui/react-announce`.
//!
//! Provides an ARIA live region for screen reader announcements.
//! Content rendered inside [`Announce`] is mirrored into a visually-hidden
//! `aria-live` region so assistive technologies can announce changes.

use dioxus::prelude::*;

/// The urgency level of an announcement.
///
/// Maps to the `aria-live` attribute value.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AnnounceType {
    /// Polite — announced at the next graceful opportunity (default).
    #[default]
    Polite,
    /// Assertive — announced immediately, interrupting current speech.
    Assertive,
    /// Off — not announced.
    Off,
}

impl AnnounceType {
    /// Returns the `aria-live` attribute value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Polite => "polite",
            Self::Assertive => "assertive",
            Self::Off => "off",
        }
    }

    /// Returns the default ARIA role for this announcement type.
    pub fn default_role(&self) -> &'static str {
        match self {
            Self::Polite => "status",
            Self::Assertive => "alert",
            Self::Off => "none",
        }
    }
}

/// Props for [`Announce`].
#[derive(Props, Clone, PartialEq)]
pub struct AnnounceProps {
    /// The announcement urgency. Defaults to `Polite`.
    #[props(default)]
    pub r#type: AnnounceType,

    /// ARIA role override. Defaults based on `type`.
    #[props(default)]
    pub role: Option<String>,

    /// Whether assistive tech should present all or parts of the changed region.
    #[props(default)]
    pub aria_atomic: Option<bool>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (content to announce).
    pub children: Element,
}

/// ARIA live region for screen reader announcements.
///
/// Matches Radix's `Announce` component. Renders a visually-hidden region
/// with `aria-live` set to the appropriate politeness level.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::announce::{Announce, AnnounceType};
/// rsx! {
///     Announce { r#type: AnnounceType::Assertive,
///         "Item deleted"
///     }
/// };
/// ```
#[component]
pub fn Announce(props: AnnounceProps) -> Element {
    let role = props
        .role
        .unwrap_or_else(|| props.r#type.default_role().to_string());

    rsx! {
        div {
            "data-slot": "announce",
            role: "{role}",
            "aria-live": props.r#type.as_str(),
            "aria-atomic": props.aria_atomic.map(|v| v.to_string()),
            style: "position: absolute; top: -1px; width: 1px; height: 1px; overflow: hidden;",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}
