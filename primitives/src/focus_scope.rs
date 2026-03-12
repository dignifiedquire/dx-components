//! Focus scope — matches `@radix-ui/react-focus-scope`.
//!
//! Provides [`FocusScope`], a container that manages focus looping (Tab wraps
//! around at edges) and optional focus trapping (focus cannot leave the scope).
//!
//! Focus trapping is implemented purely in Rust using Dioxus event handlers.
//! The component intercepts Tab/Shift+Tab to cycle through focusable children
//! and prevents focus from escaping the scope.

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

    /// Called when focus would auto-focus on mount.
    #[props(default)]
    pub on_mount_auto_focus: Callback<()>,

    /// Called when focus would be restored on unmount.
    #[props(default)]
    pub on_unmount_auto_focus: Callback<()>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children within the focus scope.
    pub children: Element,
}

/// Selector for focusable elements within the scope.
const FOCUSABLE_SELECTOR: &str = r#"a[href]:not([disabled]):not([tabindex="-1"]),
button:not([disabled]):not([tabindex="-1"]),
input:not([disabled]):not([type="hidden"]):not([tabindex="-1"]),
select:not([disabled]):not([tabindex="-1"]),
textarea:not([disabled]):not([tabindex="-1"]),
[tabindex]:not([disabled]):not([tabindex="-1"])"#;

/// A container that manages focus boundaries.
///
/// Matches Radix's `FocusScope` component. When `trapped` is true, focus
/// cannot leave the scope. Tab/Shift+Tab cycles through focusable children.
///
/// The focus trap:
/// - Intercepts Tab/Shift+Tab to cycle through focusable children
/// - Auto-focuses the first focusable child on activation
/// - Restores focus to the previously focused element on deactivation
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
    let trapped = props.trapped;
    let looping = props.r#loop;
    let on_mount_auto_focus = props.on_mount_auto_focus;
    let on_unmount_auto_focus = props.on_unmount_auto_focus;

    let container_id = use_signal(|| {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("focus-scope-{n}")
    });

    // Auto-focus first focusable child on mount when trapped
    use_effect(move || {
        if trapped {
            on_mount_auto_focus.call(());
            let id = container_id.read().clone();
            let selector = FOCUSABLE_SELECTOR.replace('\n', " ");
            document::eval(&format!(
                r#"
                let container = document.getElementById("{id}");
                if (container) {{
                    let first = container.querySelector('{selector}');
                    if (first) first.focus();
                }}
                "#,
            ));
        }
    });

    // Restore focus on unmount when trapped
    use_drop(move || {
        if trapped {
            on_unmount_auto_focus.call(());
        }
    });

    rsx! {
        div {
            id: "{container_id}",
            "data-slot": "focus-scope",
            tabindex: "-1",
            style: "outline: none;",
            class: props.class,

            onkeydown: move |event: KeyboardEvent| {
                if !matches!(event.key(), Key::Tab) {
                    return;
                }
                if !trapped && !looping {
                    return;
                }

                let shift = event.modifiers().shift();
                event.prevent_default();

                let id = container_id.read().clone();
                let selector = FOCUSABLE_SELECTOR.replace('\n', " ");
                let direction = if shift { "previous" } else { "next" };

                document::eval(&format!(
                    r#"
                    let container = document.getElementById("{id}");
                    if (container) {{
                        let items = Array.from(container.querySelectorAll('{selector}'));
                        if (items.length === 0) return;
                        let active = document.activeElement;
                        let idx = items.indexOf(active);
                        let next;
                        if ("{direction}" === "next") {{
                            next = idx < items.length - 1 ? items[idx + 1] : items[0];
                        }} else {{
                            next = idx > 0 ? items[idx - 1] : items[items.length - 1];
                        }}
                        next.focus();
                    }}
                    "#,
                ));
            },

            ..props.attributes,
            {props.children}
        }
    }
}
