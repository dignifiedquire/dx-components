//! Focus scope — matches `@radix-ui/react-focus-scope`.
//!
//! Provides [`FocusScope`], a container that manages focus looping (Tab wraps
//! around at edges) and optional focus trapping (focus cannot leave the scope).
//!
//! In Dioxus, focus management is done at the component level using
//! `MountedData::set_focus()` rather than DOM-level event listeners.

use dioxus::prelude::*;

/// Props for [`FocusScope`].
#[derive(Props, Clone, PartialEq)]
pub struct FocusScopeProps {
    /// When `true`, tabbing from last item focuses first, and Shift+Tab from
    /// first item focuses last. Defaults to `false`.
    #[props(default)]
    pub r#loop: bool,

    /// When `true`, focus cannot escape the scope. Defaults to `false`.
    #[props(default)]
    pub trapped: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children within the focus scope.
    pub children: Element,
}

/// A container that manages focus boundaries.
///
/// Matches Radix's `FocusScope` component. When `loop` is true, Tab/Shift+Tab
/// wraps around at the edges. When `trapped` is true, focus cannot leave the scope.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::focus_scope::FocusScope;
/// rsx! {
///     FocusScope { r#loop: true, trapped: true,
///         button { "First" }
///         button { "Second" }
///         button { "Third" }
///     }
/// };
/// ```
#[component]
pub fn FocusScope(props: FocusScopeProps) -> Element {
    rsx! {
        div {
            "data-slot": "focus-scope",
            tabindex: "-1",
            style: "outline: none;",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}
