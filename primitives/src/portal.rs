//! Portal primitive — matches `@radix-ui/react-portal`.
//!
//! Radix uses `ReactDOM.createPortal` to render overlay content at `document.body`,
//! escaping parent overflow/z-index stacking contexts. Dioxus has no `createPortal`
//! equivalent, so we use a context-based signal system instead:
//!
//! - [`use_portal`] creates a unique slot in a root-level `HashMap<usize, Signal<Element>>`
//! - [`PortalIn`] writes content into that slot's signal
//! - [`PortalHost`] iterates all slots and renders them at the app root
//!
//! Users must place `<PortalHost />` at the top level of their application for
//! overlay components (Dialog, Popover, Tooltip, etc.) to render correctly.

use crate::dioxus_core::provide_root_context;
use dioxus::prelude::*;
use std::collections::HashMap;

use crate::use_effect_cleanup;

/// Unique identifier for a portal slot.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PortalId(usize);

#[derive(Clone, Copy, PartialEq)]
struct PortalCtx {
    portals: Signal<HashMap<usize, Signal<Element>>>,
    /// Portal IDs that have a dedicated [`PortalOut`] and should be
    /// skipped by [`PortalHost`].
    has_out: Signal<std::collections::HashSet<usize>>,
    /// Whether a [`PortalHost`] is mounted. When false, [`Portal`] renders
    /// children inline as a graceful fallback.
    has_host: Signal<bool>,
}

/// Create a portal slot. Returns a [`PortalId`] for use with [`PortalIn`] or [`PortalOut`].
pub fn use_portal() -> PortalId {
    static NEXT_ID: GlobalSignal<usize> = Signal::global(|| 0);

    let (sig, id) = use_hook(|| {
        let mut next_id = NEXT_ID.write();
        let id = *next_id;
        *next_id += 1;

        let mut ctx = match try_consume_context::<PortalCtx>() {
            Some(ctx) => ctx,
            None => {
                let portals = Signal::new_in_scope(HashMap::new(), ScopeId::ROOT);
                let has_out = Signal::new_in_scope(std::collections::HashSet::new(), ScopeId::ROOT);
                let has_host = Signal::new_in_scope(false, ScopeId::ROOT);
                let ctx = PortalCtx {
                    portals,
                    has_out,
                    has_host,
                };
                provide_root_context(ctx)
            }
        };

        let sig = Signal::new_in_scope(VNode::empty(), ScopeId::ROOT);
        ctx.portals.write().insert(id, sig);

        (sig, PortalId(id))
    });

    // Cleanup the portal.
    use_effect_cleanup(move || {
        let mut ctx = consume_context::<PortalCtx>();
        ctx.portals.write().remove(&id.0);
        sig.manually_drop();
    });

    id
}

/// Send content into a portal slot. The content will be rendered wherever
/// [`PortalHost`] (or [`PortalOut`] for the same id) is placed.
#[component]
pub fn PortalIn(portal: PortalId, children: Element) -> Element {
    if let Some(ctx) = try_use_context::<PortalCtx>() {
        // Use peek() to avoid subscribing to / notifying the portals HashMap.
        // We only need to write to the individual slot signal.
        if let Some(mut slot) = ctx.portals.peek().get(&portal.0).copied() {
            slot.set(children);
        }
    }

    rsx! {}
}

/// Render a single portal slot's content. Used by Toast internally.
/// For overlay components, prefer [`PortalHost`] which renders all slots.
///
/// Portals rendered via `PortalOut` are automatically excluded from
/// [`PortalHost`] to avoid duplicate rendering.
#[component]
pub fn PortalOut(portal: PortalId) -> Element {
    if let Some(mut ctx) = try_use_context::<PortalCtx>() {
        // Register this portal as having a dedicated PortalOut so
        // PortalHost skips it (avoids duplicate rendering).
        let id = portal.0;
        use_hook(move || {
            ctx.has_out.write().insert(id);
        });
        use_effect_cleanup(move || {
            ctx.has_out.write().remove(&id);
        });

        if let Some(children) = ctx.portals.peek().get(&id) {
            return rsx! {
                {*children}
            };
        }
    }

    rsx! {}
}

// ---------------------------------------------------------------------------
// PortalHost
// ---------------------------------------------------------------------------

/// Renders ALL active portal entries at the position where this component is placed.
///
/// Place this at the top level of your application so overlay content (Dialog,
/// Popover, Tooltip, etc.) renders outside parent overflow/stacking contexts.
///
/// ## Radix deviation
/// Radix uses `ReactDOM.createPortal(children, document.body)` which appends
/// directly to `<body>`. Dioxus has no `createPortal`, so we render all portal
/// content inside this host component wherever it's placed in the tree.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::portal::PortalHost;
/// fn App() -> Element {
///     rsx! {
///         div { /* your app content */ }
///         PortalHost {}
///     }
/// }
/// ```
#[component]
pub fn PortalHost() -> Element {
    let mut ctx = match try_use_context::<PortalCtx>() {
        Some(ctx) => ctx,
        None => return rsx! {},
    };

    // Signal to Portal components that a host is available
    if !*ctx.has_host.peek() {
        ctx.has_host.set(true);
    }

    let portals = ctx.portals.read();
    let excluded = ctx.has_out.read();

    rsx! {
        div {
            "data-slot": "portal-host",
            // Radix deviation: Radix portals append directly to document.body as
            // individual elements. We render them inside a host div because Dioxus
            // requires a parent element to iterate children.
            //
            // Skip portals that have a dedicated PortalOut to avoid duplicates.
            for (&id, &signal) in portals.iter() {
                if !excluded.contains(&id) {
                    div {
                        key: "{id}",
                        "data-portal-id": "{id}",
                        {signal}
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Portal (convenience component)
// ---------------------------------------------------------------------------

/// Props for [`Portal`].
#[derive(Props, Clone, PartialEq)]
pub struct PortalProps {
    /// Children to render through the portal.
    pub children: Element,
}

/// Teleports children to the nearest [`PortalHost`].
///
/// Matches Radix's `Portal` component. Wraps content in [`PortalIn`] automatically.
///
/// When no [`PortalHost`] is mounted (e.g. in tests), children render inline
/// as a graceful fallback.
///
/// ## Radix deviation
/// Radix `Portal` accepts a `container` prop to specify a custom DOM container.
/// This is not supported because Dioxus cannot render into arbitrary DOM nodes.
/// Content always renders inside the nearest [`PortalHost`].
#[component]
pub fn Portal(props: PortalProps) -> Element {
    let portal = use_portal();

    // Check if a PortalHost is mounted. If not, render children inline
    // so components work without requiring PortalHost (e.g. in tests).
    let ctx = try_use_context::<PortalCtx>();
    let has_host = ctx.is_some_and(|c| (c.has_host)());

    if has_host {
        rsx! {
            PortalIn { portal, {props.children} }
        }
    } else {
        // Fallback: render inline when no PortalHost is available.
        // The portal slot exists but is unused; cleaned up on unmount.
        rsx! {
            {props.children}
        }
    }
}
