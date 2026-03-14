//! Focus guards — matches `@radix-ui/react-focus-guards`.
//!
//! Injects a pair of invisible sentinel `<span>` elements at the edges of
//! `document.body` to ensure `focusin`/`focusout` events can be caught
//! consistently. Uses reference counting so guards are shared across
//! multiple consumers and only removed when the last consumer unmounts.

use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use std::sync::atomic::{AtomicUsize, Ordering};

/// Number of components which have requested interest to have focus guards.
/// Matches upstream's module-level `let count = 0;`.
#[cfg(target_arch = "wasm32")]
static GUARD_COUNT: AtomicUsize = AtomicUsize::new(0);

#[cfg(target_arch = "wasm32")]
const GUARD_ATTR: &str = "data-radix-focus-guard";

/// Props for [`FocusGuards`].
#[derive(Props, Clone, PartialEq)]
pub struct FocusGuardsProps {
    /// Children — rendered as-is. The focus guards are injected at body edges,
    /// not inline around the children.
    pub children: Element,
}

/// Renders its children and injects focus guards at body edges.
///
/// Matches Radix's `FocusGuards` component. Calls [`use_focus_guards`]
/// internally — the component itself renders no wrapper elements.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::focus_guards::FocusGuards;
/// rsx! {
///     FocusGuards {
///         // Content that needs focus boundary guards
///         div { "Focusable content" }
///     }
/// };
/// ```
#[component]
pub fn FocusGuards(props: FocusGuardsProps) -> Element {
    use_focus_guards();
    props.children
}

/// Injects a pair of focus guards at the edges of `document.body`.
///
/// Matches Radix's `useFocusGuards` hook. The guards are invisible,
/// focusable `<span>` elements placed at the very start and end of
/// `<body>` to catch focus events at boundaries.
///
/// Reference-counted: guards are shared across consumers and only
/// removed when the last consumer unmounts. Safe to call from multiple
/// components simultaneously.
pub fn use_focus_guards() {
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        inject_guards();
    });
    crate::use_effect_cleanup(move || {
        #[cfg(target_arch = "wasm32")]
        cleanup_guards();
    });
}

/// Creates a focus guard `<span>` element matching upstream's `createFocusGuard`.
#[cfg(target_arch = "wasm32")]
fn create_focus_guard(doc: &web_sys::Document) -> Option<web_sys::Node> {
    let el = doc.create_element("span").ok()?;
    el.set_attribute(GUARD_ATTR, "").ok()?;
    el.set_attribute("tabindex", "0").ok()?;
    el.set_attribute(
        "style",
        "outline: none; opacity: 0; position: fixed; pointer-events: none",
    )
    .ok()?;
    Some(el.into())
}

/// Injects focus guard spans at body edges, matching upstream's `useFocusGuards`
/// effect body.
#[cfg(target_arch = "wasm32")]
fn inject_guards() {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    let Some(body) = doc.body() else { return };

    let selector = format!("[{GUARD_ATTR}]");
    let Ok(existing) = doc.query_selector_all(&selector) else {
        return;
    };

    // Reuse existing guards or create new ones (matches upstream's ?? operator)
    let first = existing.item(0).or_else(|| create_focus_guard(&doc));
    let last = existing.item(1).or_else(|| create_focus_guard(&doc));

    let body_node: &web_sys::Node = body.as_ref();

    // Insert first guard at very beginning of body (matches 'afterbegin')
    if let Some(guard) = first {
        let _ = body_node.insert_before(&guard, body_node.first_child().as_ref());
    }

    // Insert last guard at very end of body (matches 'beforeend')
    if let Some(guard) = last {
        let _ = body_node.append_child(&guard);
    }

    GUARD_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Decrements guard count and removes guard elements when the last consumer
/// unmounts. Matches upstream's cleanup function.
#[cfg(target_arch = "wasm32")]
fn cleanup_guards() {
    let prev = GUARD_COUNT.fetch_sub(1, Ordering::Relaxed);
    if prev <= 1 {
        let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
            return;
        };
        let selector = format!("[{GUARD_ATTR}]");
        let Ok(guards) = doc.query_selector_all(&selector) else {
            return;
        };
        for i in 0..guards.length() {
            if let Some(node) = guards.item(i) {
                if let Some(parent) = node.parent_node() {
                    let _ = parent.remove_child(&node);
                }
            }
        }
    }
}
