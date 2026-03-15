//! Dismissable layer — line-by-line port of `@radix-ui/react-dismissable-layer`.
//!
//! Provides [`DismissableLayer`], a container that can be dismissed via Escape
//! key or interactions outside its bounds (pointer down or focus changes).
//!
//! A shared [`DismissableLayerContext`] tracks all active layers, branches,
//! and layers with outside pointer events disabled. This supports nested
//! dismissable layers where only the topmost layer responds to interactions.
//!
//! [`DismissableLayerBranch`] marks a DOM subtree as "inside" the layer for
//! dismissal purposes, even when rendered outside the layer's DOM tree.
//!
//! ## Differences from upstream
//!
//! - **`handleAndDispatchCustomEvent`**: Upstream dispatches a DOM `CustomEvent`
//!   and uses `ReactDOM.flushSync` / `dispatchDiscreteCustomEvent` for React 18+
//!   batching. Dioxus doesn't batch event handlers like React, so we call
//!   callbacks directly and use [`DismissableEvent`] for preventability.
//!
//! - **`dispatchUpdate`**: Upstream dispatches a DOM `CustomEvent` on `document`
//!   to force all layers to re-render (since mutable `Set`s don't trigger React
//!   re-renders). We use a `Signal<u64>` version counter instead; incrementing
//!   it triggers re-renders for all subscribers.
//!
//! - **Refs → IDs**: Upstream stores DOM element references directly. Dioxus
//!   uses `use_unique_id()` with `document.get_element_by_id()`.

use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::rc::Rc;

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Constants — match upstream
// ---------------------------------------------------------------------------

/// Upstream: `const POINTER_DOWN_OUTSIDE = 'dismissableLayer.pointerDownOutside'`
/// Used only for documentation; we call callbacks directly instead of DOM events.
#[allow(dead_code)]
const POINTER_DOWN_OUTSIDE: &str = "dismissableLayer.pointerDownOutside";

/// Upstream: `const FOCUS_OUTSIDE = 'dismissableLayer.focusOutside'`
#[allow(dead_code)]
const FOCUS_OUTSIDE: &str = "dismissableLayer.focusOutside";

// ---------------------------------------------------------------------------
// Module-level state — matches upstream's `let originalBodyPointerEvents: string`
// ---------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
thread_local! {
    static ORIGINAL_BODY_POINTER_EVENTS: RefCell<Option<String>> = const { RefCell::new(None) };
}

// ---------------------------------------------------------------------------
// DismissableLayerContext — matches upstream's React.createContext(...)
// ---------------------------------------------------------------------------

type LayerId = String;

#[derive(Clone)]
struct DismissableLayerContextInner {
    /// All active layer IDs, in insertion (creation) order.
    /// Upstream: `layers: new Set<DismissableLayerElement>()`
    layers: Vec<LayerId>,
    /// Upstream: `layersWithOutsidePointerEventsDisabled: new Set<DismissableLayerElement>()`
    layers_with_outside_pointer_events_disabled: HashSet<LayerId>,
    /// Upstream: `branches: new Set<DismissableLayerBranchElement>()`
    branches: HashSet<LayerId>,
}

/// Shared context tracking all active dismissable layers and branches.
#[derive(Clone, Copy)]
struct DismissableLayerContext {
    inner: Signal<Rc<RefCell<DismissableLayerContextInner>>>,
    /// Version counter — incremented by `dispatch_update()` to trigger re-renders.
    /// Replaces upstream's `document.dispatchEvent(new CustomEvent(CONTEXT_UPDATE))`.
    version: Signal<u64>,
}

/// Get or create the shared dismissable layer context.
///
/// Must not use `use_context_provider` conditionally (hooks can't be called
/// inside conditionals in Dioxus). Instead, `use_hook` runs only on the first
/// render and caches the result, keeping hook count stable across re-renders.
fn use_dismissable_layer_context() -> DismissableLayerContext {
    use_hook(|| match try_consume_context::<DismissableLayerContext>() {
        Some(c) => c,
        None => {
            let ctx = DismissableLayerContext {
                inner: Signal::new_in_scope(
                    Rc::new(RefCell::new(DismissableLayerContextInner {
                        layers: Vec::new(),
                        layers_with_outside_pointer_events_disabled: HashSet::new(),
                        branches: HashSet::new(),
                    })),
                    ScopeId::ROOT,
                ),
                version: Signal::new_in_scope(0u64, ScopeId::ROOT),
            };
            provide_context(ctx);
            ctx
        }
    })
}

// ---------------------------------------------------------------------------
// DismissableEvent — replaces upstream's preventable CustomEvent pattern
// ---------------------------------------------------------------------------

/// A preventable event passed to dismissable layer callbacks.
///
/// Upstream uses DOM `CustomEvent` with `cancelable: true` and checks
/// `event.defaultPrevented`. This Rust type replicates that pattern.
///
/// Call [`prevent_default()`](DismissableEvent::prevent_default) in your
/// callback to prevent the layer from being dismissed.
pub struct DismissableEvent {
    prevented: Rc<Cell<bool>>,
}

impl DismissableEvent {
    /// Create a new preventable event.
    pub fn new() -> Self {
        Self {
            prevented: Rc::new(Cell::new(false)),
        }
    }

    /// Prevent the default dismissal behavior.
    pub fn prevent_default(&self) {
        self.prevented.set(true);
    }

    /// Check if `prevent_default()` was called.
    pub fn is_default_prevented(&self) -> bool {
        self.prevented.get()
    }
}

impl Default for DismissableEvent {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// DismissableLayer
// ---------------------------------------------------------------------------

/// Props for [`DismissableLayer`].
#[derive(Props, Clone, PartialEq)]
pub struct DismissableLayerProps {
    /// When `true`, hover/focus/click interactions are disabled on elements
    /// outside the layer.
    /// Upstream: `disableOutsidePointerEvents?: boolean` (default `false`)
    #[props(default)]
    pub disable_outside_pointer_events: bool,

    /// Called when the Escape key is pressed. Can be prevented.
    /// Upstream: `onEscapeKeyDown?: (event: KeyboardEvent) => void`
    #[props(default)]
    pub on_escape_key_down: Callback<DismissableEvent>,

    /// Called when a pointer event occurs outside the layer. Can be prevented.
    /// Upstream: `onPointerDownOutside?: (event: PointerDownOutsideEvent) => void`
    #[props(default)]
    pub on_pointer_down_outside: Callback<DismissableEvent>,

    /// Called when focus moves outside the layer. Can be prevented.
    /// Upstream: `onFocusOutside?: (event: FocusOutsideEvent) => void`
    #[props(default)]
    pub on_focus_outside: Callback<DismissableEvent>,

    /// Called when any outside interaction occurs (pointer or focus). Can be prevented.
    /// Upstream: `onInteractOutside?: (event: PointerDownOutsideEvent | FocusOutsideEvent) => void`
    #[props(default)]
    pub on_interact_outside: Callback<DismissableEvent>,

    /// Called when the layer should be dismissed.
    /// Upstream: `onDismiss?: () => void`
    #[props(default)]
    pub on_dismiss: Callback<()>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children within the dismissable layer.
    pub children: Element,
}

/// A layer that can be dismissed via Escape key or outside interactions.
///
/// Matches Radix's `DismissableLayer`. Escape key handling uses stack
/// discipline (only the topmost layer responds). Outside interaction
/// detection uses document-level event listeners via web-sys.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::dismissable_layer::{DismissableLayer, DismissableEvent};
/// rsx! {
///     DismissableLayer {
///         on_dismiss: move |_| { /* close the overlay */ },
///         div { "Dismissable content" }
///     }
/// };
/// ```
#[component]
pub fn DismissableLayer(props: DismissableLayerProps) -> Element {
    let on_dismiss = props.on_dismiss;
    let on_escape_key_down = props.on_escape_key_down;
    let disable_outside_pointer_events = props.disable_outside_pointer_events;

    let layer_id = crate::use_unique_id();
    #[allow(clippy::redundant_closure)] // Signal<String> is not FnMut, clippy's suggestion is wrong
    let layer_id_memo = use_memo(move || layer_id());

    // Ensure a layer context exists (creates one if this is the outermost layer)
    // Upstream: `const context = React.useContext(DismissableLayerContext)`
    let ctx = use_dismissable_layer_context();

    // --- Compute pointer-events-related indices ---
    // Upstream:
    //   const layers = Array.from(context.layers);
    //   const [highestLayerWithOutsidePointerEventsDisabled] = [...context.layersWithOutsidePointerEventsDisabled].slice(-1);
    //   const highestLayerWithOutsidePointerEventsDisabledIndex = layers.indexOf(highestLayerWithOutsidePointerEventsDisabled!);
    //   const index = node ? layers.indexOf(node) : -1;
    //   const isBodyPointerEventsDisabled = context.layersWithOutsidePointerEventsDisabled.size > 0;
    //   const isPointerEventsEnabled = index >= highestLayerWithOutsidePointerEventsDisabledIndex;

    // Read version to subscribe to context updates (replaces upstream's CONTEXT_UPDATE listener)
    let _version = ctx.version.read();

    let (is_body_pointer_events_disabled, is_pointer_events_enabled) = {
        let inner = ctx.inner.read();
        let inner = inner.borrow();
        let layers = &inner.layers;
        let disabled_set = &inner.layers_with_outside_pointer_events_disabled;

        let is_body_disabled = !disabled_set.is_empty();

        // Find the highest (last) layer that has outside pointer events disabled
        let highest_disabled_idx = layers.iter().rposition(|l| disabled_set.contains(l));

        let this_idx = layers.iter().position(|l| l == &layer_id_memo());

        let is_enabled = match (this_idx, highest_disabled_idx) {
            (Some(this), Some(highest)) => this >= highest,
            (Some(_), None) => true,
            _ => true,
        };

        (is_body_disabled, is_enabled)
    };

    // --- usePointerDownOutside ---
    // Returns an `Rc<Cell<bool>>` that serves as `isPointerInsideReactTreeRef`.
    // The component sets it to `true` in `onPointerDownCapture`.
    let is_pointer_inside_tree = use_hook(|| Rc::new(Cell::new(false)));

    #[cfg(target_arch = "wasm32")]
    {
        let on_pointer_down_outside = props.on_pointer_down_outside;
        let on_interact_outside = props.on_interact_outside;
        let is_pointer_inside = is_pointer_inside_tree.clone();
        wasm_impl::use_pointer_down_outside_effect(
            layer_id_memo,
            is_pointer_inside,
            ctx,
            on_pointer_down_outside,
            on_interact_outside,
            on_dismiss,
        );
    }

    // --- useFocusOutside ---
    let is_focus_inside_tree = use_hook(|| Rc::new(Cell::new(false)));

    #[cfg(target_arch = "wasm32")]
    {
        let on_focus_outside = props.on_focus_outside;
        let on_interact_outside = props.on_interact_outside;
        let is_focus_inside = is_focus_inside_tree.clone();
        wasm_impl::use_focus_outside_effect(
            layer_id_memo,
            is_focus_inside,
            ctx,
            on_focus_outside,
            on_interact_outside,
            on_dismiss,
        );
    }

    // --- useEscapeKeydown ---
    // Upstream: useEscapeKeydown((event) => { ... }, ownerDocument)
    crate::use_escape_keydown::use_escape_keydown(
        move |event| {
            // Compute index at call time — not render time — because the layer
            // isn't registered in context until the mount effect runs.
            let inner = ctx.inner.read();
            let inner_ref = inner.borrow();
            let id = layer_id_memo();
            let is_highest = inner_ref
                .layers
                .iter()
                .position(|l| l == &id)
                .map(|idx| idx == inner_ref.layers.len() - 1)
                .unwrap_or(false);
            drop(inner_ref);

            if !is_highest {
                return;
            }

            let dismissable_event = DismissableEvent::new();
            let prevented = dismissable_event.prevented.clone();
            on_escape_key_down.call(dismissable_event);

            if !prevented.get() {
                event.prevent_default();
                on_dismiss.call(());
            }
        },
        None,
    );

    // --- Effect 1: mount — add to context, set body pointer-events, dispatch_update ---
    // Upstream: React.useEffect(() => { ... }, [node, ownerDocument, disableOutsidePointerEvents, context])
    crate::use_effect_with_cleanup({
        let id = layer_id_memo;
        move || {
            let id_val = id();
            {
                let inner = ctx.inner.read();
                let mut inner = inner.borrow_mut();

                if disable_outside_pointer_events {
                    // Upstream: if (context.layersWithOutsidePointerEventsDisabled.size === 0)
                    //             originalBodyPointerEvents = ownerDocument.body.style.pointerEvents;
                    //             ownerDocument.body.style.pointerEvents = 'none';
                    if inner.layers_with_outside_pointer_events_disabled.is_empty() {
                        #[cfg(target_arch = "wasm32")]
                        wasm_impl::save_and_disable_body_pointer_events();
                    }
                    inner
                        .layers_with_outside_pointer_events_disabled
                        .insert(id_val.clone());
                }

                // Upstream: context.layers.add(node)
                inner.layers.push(id_val.clone());
            }

            // Upstream: dispatchUpdate()
            dispatch_update(ctx);

            // Cleanup — upstream: return () => { ... }
            // NOTE: In upstream React, Effect 1's cleanup runs before Effect 2's
            // (effects clean up in registration order). In Dioxus, `use_drop` runs
            // in reverse order (LIFO), so Effect 2's cleanup runs first. We move the
            // body pointer-events restoration into Effect 2's cleanup to ensure it
            // checks the set BEFORE removal.
            Box::new(|| {}) as Box<dyn FnOnce()>
        }
    });

    // --- Effect 2: unmount — remove from context, dispatch_update ---
    // Upstream comment: "We purposefully prevent combining this effect with the
    // `disableOutsidePointerEvents` effect because a change to
    // `disableOutsidePointerEvents` would remove this layer from the stack and
    // add it to the end again so the layering order wouldn't be _creation order_.
    // We only want them to be removed from context stacks when unmounted."
    //
    // Upstream: React.useEffect(() => { return () => { ... } }, [node, context])
    crate::use_effect_cleanup({
        let id = layer_id_memo;
        move || {
            let id_val = id();
            let inner = ctx.inner.read();
            let mut inner = inner.borrow_mut();

            // Restore body pointer-events BEFORE removing from the set.
            // Upstream: if (disableOutsidePointerEvents && context.layersWithOutsidePointerEventsDisabled.size === 1)
            //             ownerDocument.body.style.pointerEvents = originalBodyPointerEvents;
            if disable_outside_pointer_events
                && inner.layers_with_outside_pointer_events_disabled.len() == 1
                && inner
                    .layers_with_outside_pointer_events_disabled
                    .contains(&id_val)
            {
                #[cfg(target_arch = "wasm32")]
                wasm_impl::restore_body_pointer_events();
            }

            inner.layers.retain(|l| l != &id_val);
            inner
                .layers_with_outside_pointer_events_disabled
                .remove(&id_val);
            drop(inner);
            dispatch_update(ctx);
        }
    });

    // --- Pointer-events style ---
    // Upstream:
    //   style={{ pointerEvents: isBodyPointerEventsDisabled
    //     ? isPointerEventsEnabled ? 'auto' : 'none'
    //     : undefined, ...props.style }}
    let pointer_events_style = if is_body_pointer_events_disabled {
        if is_pointer_events_enabled {
            "pointer-events: auto;"
        } else {
            "pointer-events: none;"
        }
    } else {
        ""
    };

    // --- Capture handlers for pointer-inside-tree and focus-inside-tree flags ---
    let pointer_inside_clone = is_pointer_inside_tree.clone();
    let focus_inside_clone_focus = is_focus_inside_tree.clone();
    let focus_inside_clone_blur = is_focus_inside_tree.clone();

    rsx! {
        div {
            id: "{layer_id}",
            style: pointer_events_style,

            // Upstream: onPointerDownCapture => isPointerInsideReactTreeRef.current = true
            onpointerdown: move |_: PointerEvent| {
                pointer_inside_clone.set(true);
            },

            // Upstream: onFocusCapture => isFocusInsideReactTreeRef.current = true
            onfocusin: move |_: FocusEvent| {
                focus_inside_clone_focus.set(true);
            },

            // Upstream: onBlurCapture => isFocusInsideReactTreeRef.current = false
            onfocusout: move |_: FocusEvent| {
                focus_inside_clone_blur.set(false);
            },

            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DismissableLayerBranch
// ---------------------------------------------------------------------------

/// Props for [`DismissableLayerBranch`].
#[derive(Props, Clone, PartialEq)]
pub struct DismissableLayerBranchProps {
    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A branch of a [`DismissableLayer`] that is considered "inside" the layer.
///
/// Interactions within a branch won't trigger dismissal, even if the branch
/// is rendered outside the layer's DOM subtree.
///
/// Matches Radix's `DismissableLayerBranch`.
#[component]
pub fn DismissableLayerBranch(props: DismissableLayerBranchProps) -> Element {
    let branch_id = crate::use_unique_id();

    // Register this branch in the shared context (if one exists)
    // Upstream: React.useEffect(() => { context.branches.add(node); return () => { context.branches.delete(node) } }, [context.branches])
    let ctx = use_hook(try_consume_context::<DismissableLayerContext>);
    crate::use_effect_with_cleanup({
        let id = branch_id;
        move || {
            if let Some(ctx) = ctx {
                let id_val = (id)();
                {
                    let inner = ctx.inner.read();
                    let mut inner = inner.borrow_mut();
                    inner.branches.insert(id_val.clone());
                }

                Box::new(move || {
                    let inner = ctx.inner.read();
                    let mut inner = inner.borrow_mut();
                    inner.branches.remove(&id_val);
                }) as Box<dyn FnOnce()>
            } else {
                Box::new(|| {}) as Box<dyn FnOnce()>
            }
        }
    });

    rsx! {
        div {
            id: "{branch_id}",
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// dispatchUpdate — matches upstream
// ---------------------------------------------------------------------------

/// Upstream: `function dispatchUpdate() { document.dispatchEvent(new CustomEvent(CONTEXT_UPDATE)) }`
///
/// We increment a version signal instead, which triggers re-renders for all
/// components that read it.
fn dispatch_update(ctx: DismissableLayerContext) {
    let mut version = ctx.version;
    version += 1;
}

// ---------------------------------------------------------------------------
// Wasm implementation — usePointerDownOutside, useFocusOutside
// ---------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
mod wasm_impl {
    use super::*;
    use wasm_bindgen::prelude::*;

    // -----------------------------------------------------------------------
    // usePointerDownOutside — matches upstream
    // -----------------------------------------------------------------------

    /// Matches upstream `usePointerDownOutside(callback, ownerDocument)`.
    ///
    /// Registers a document-level `pointerdown` listener (delayed via
    /// `setTimeout(0)` to avoid capturing the mount event). For touch devices,
    /// defers to a `click` listener with `{ once: true }`.
    pub(super) fn use_pointer_down_outside_effect(
        layer_id: Memo<String>,
        is_pointer_inside_tree: Rc<Cell<bool>>,
        ctx: DismissableLayerContext,
        on_pointer_down_outside: Callback<DismissableEvent>,
        on_interact_outside: Callback<DismissableEvent>,
        on_dismiss: Callback<()>,
    ) {
        crate::use_effect_with_cleanup(move || {
            let document = web_sys::window()
                .and_then(|w| w.document())
                .expect("document");
            let window = web_sys::window().expect("window");

            // Shared state for click handler reference (touch devices)
            // Upstream: `const handleClickRef = React.useRef(() => {})`
            let click_closure: Rc<RefCell<Option<Closure<dyn FnMut(web_sys::Event)>>>> =
                Rc::new(RefCell::new(None));

            let is_pointer_inside = is_pointer_inside_tree.clone();
            let click_closure_for_handler = click_closure.clone();
            let doc_for_handler = document.clone();

            let layer_id_for_handler = layer_id;
            let handler = Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
                let target = event.target();
                let has_target = target.is_some();

                if has_target && !is_pointer_inside.get() {
                    // Check branches — upstream: const isPointerDownOnBranch = [...context.branches].some(branch => branch.contains(target))
                    let target_node: Option<web_sys::Node> = target
                        .as_ref()
                        .and_then(|t| t.dyn_ref::<web_sys::Node>().cloned());

                    let is_on_branch = check_branches_contain_target(&ctx, target_node.as_ref());

                    // Compute is_pointer_events_enabled at event time (not render time)
                    let is_pointer_events_enabled = {
                        let inner = ctx.inner.read();
                        let inner = inner.borrow();
                        let id = layer_id_for_handler();
                        let highest_disabled_idx = inner.layers.iter().rposition(|l| {
                            inner
                                .layers_with_outside_pointer_events_disabled
                                .contains(l)
                        });
                        let this_idx = inner.layers.iter().position(|l| l == &id);
                        match (this_idx, highest_disabled_idx) {
                            (Some(this), Some(highest)) => this >= highest,
                            (Some(_), None) => true,
                            _ => true,
                        }
                    };

                    if !is_pointer_events_enabled || is_on_branch {
                        // Reset flag and return
                        is_pointer_inside.set(false);
                        return;
                    }

                    // The actual dispatch function
                    let on_pdo = on_pointer_down_outside;
                    let on_io = on_interact_outside;
                    let on_d = on_dismiss;

                    let do_dispatch = {
                        move || {
                            // Upstream: handleAndDispatchCustomEvent(POINTER_DOWN_OUTSIDE, handler, eventDetail, { discrete: true })
                            // We call callbacks directly instead.
                            let event = DismissableEvent::new();
                            let prevented = event.prevented.clone();
                            on_pdo.call(event);

                            if !prevented.get() {
                                let event2 = DismissableEvent::new();
                                let prevented2 = event2.prevented.clone();
                                on_io.call(event2);

                                if !prevented2.get() {
                                    on_d.call(());
                                }
                            }
                        }
                    };

                    // Touch handling — upstream:
                    // if (event.pointerType === 'touch') {
                    //   ownerDocument.removeEventListener('click', handleClickRef.current);
                    //   handleClickRef.current = handleAndDispatchPointerDownOutsideEvent;
                    //   ownerDocument.addEventListener('click', handleClickRef.current, { once: true });
                    // } else { handleAndDispatchPointerDownOutsideEvent(); }
                    if event.pointer_type() == "touch" {
                        // Remove previous click listener if any
                        if let Some(old_closure) = click_closure_for_handler.borrow_mut().take() {
                            let _ = doc_for_handler.remove_event_listener_with_callback(
                                "click",
                                old_closure.as_ref().unchecked_ref(),
                            );
                        }

                        let new_click_closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                            do_dispatch();
                        })
                            as Box<dyn FnMut(web_sys::Event)>);

                        let mut opts = web_sys::AddEventListenerOptions::new();
                        opts.once(true);
                        let _ = doc_for_handler
                            .add_event_listener_with_callback_and_add_event_listener_options(
                                "click",
                                new_click_closure.as_ref().unchecked_ref(),
                                &opts,
                            );

                        *click_closure_for_handler.borrow_mut() = Some(new_click_closure);
                    } else {
                        do_dispatch();
                    }
                } else {
                    // Upstream: ownerDocument.removeEventListener('click', handleClickRef.current)
                    // "We need to remove the event listener in case the outside click has been canceled."
                    if let Some(old_closure) = click_closure_for_handler.borrow_mut().take() {
                        let _ = doc_for_handler.remove_event_listener_with_callback(
                            "click",
                            old_closure.as_ref().unchecked_ref(),
                        );
                    }
                }

                // Upstream: isPointerInsideReactTreeRef.current = false
                is_pointer_inside.set(false);
            }) as Box<dyn FnMut(web_sys::PointerEvent)>);

            // Delayed registration — upstream:
            // const timerId = window.setTimeout(() => {
            //   ownerDocument.addEventListener('pointerdown', handlePointerDown);
            // }, 0);
            let handler_fn = handler.as_ref().unchecked_ref::<js_sys::Function>().clone();
            let doc_for_timeout = document.clone();

            let timeout_closure = Closure::wrap(Box::new(move || {
                let _ =
                    doc_for_timeout.add_event_listener_with_callback("pointerdown", &handler_fn);
            }) as Box<dyn FnMut()>);

            let timer_id = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    timeout_closure.as_ref().unchecked_ref(),
                    0,
                )
                .unwrap_or(0);

            // Must prevent timeout_closure from being dropped
            timeout_closure.forget();

            let handler_fn_cleanup = handler.as_ref().unchecked_ref::<js_sys::Function>().clone();
            let click_closure_cleanup = click_closure.clone();
            let doc_cleanup = document.clone();

            // Keep handler alive
            let _handler_prevent_drop = handler;

            // Cleanup — upstream: return () => { clearTimeout(timerId); removeEventListener('pointerdown', ...); removeEventListener('click', ...) }
            Box::new(move || {
                if let Some(window) = web_sys::window() {
                    window.clear_timeout_with_handle(timer_id);
                }
                let _ = doc_cleanup
                    .remove_event_listener_with_callback("pointerdown", &handler_fn_cleanup);

                if let Some(click) = click_closure_cleanup.borrow_mut().take() {
                    let _ = doc_cleanup.remove_event_listener_with_callback(
                        "click",
                        click.as_ref().unchecked_ref(),
                    );
                }

                drop(_handler_prevent_drop);
            }) as Box<dyn FnOnce()>
        });
    }

    // -----------------------------------------------------------------------
    // useFocusOutside — matches upstream
    // -----------------------------------------------------------------------

    /// Matches upstream `useFocusOutside(callback, ownerDocument)`.
    ///
    /// Listens for `focusin` events on the document. If focus moves outside
    /// the layer's component tree (checked via `isFocusInsideReactTreeRef`),
    /// calls the provided handler.
    pub(super) fn use_focus_outside_effect(
        _layer_id: Memo<String>,
        is_focus_inside_tree: Rc<Cell<bool>>,
        ctx: DismissableLayerContext,
        on_focus_outside: Callback<DismissableEvent>,
        on_interact_outside: Callback<DismissableEvent>,
        on_dismiss: Callback<()>,
    ) {
        crate::use_effect_with_cleanup(move || {
            let document = web_sys::window()
                .and_then(|w| w.document())
                .expect("document");

            let is_focus_inside = is_focus_inside_tree.clone();

            // Upstream:
            // const handleFocus = (event: FocusEvent) => {
            //   if (event.target && !isFocusInsideReactTreeRef.current) {
            //     handleAndDispatchCustomEvent(FOCUS_OUTSIDE, handleFocusOutside, eventDetail, { discrete: false });
            //   }
            // };
            let closure = Closure::wrap(Box::new(move |event: web_sys::FocusEvent| {
                let target = event.target();
                if target.is_none() {
                    return;
                }

                if !is_focus_inside.get() {
                    // Check branches
                    let target_node: Option<web_sys::Node> = target
                        .as_ref()
                        .and_then(|t| t.dyn_ref::<web_sys::Node>().cloned());

                    let is_focus_in_branch =
                        check_branches_contain_target(&ctx, target_node.as_ref());

                    if is_focus_in_branch {
                        return;
                    }

                    // Upstream: handleAndDispatchCustomEvent(FOCUS_OUTSIDE, handler, ..., { discrete: false })
                    let event = DismissableEvent::new();
                    let prevented = event.prevented.clone();
                    on_focus_outside.call(event);

                    if !prevented.get() {
                        let event2 = DismissableEvent::new();
                        let prevented2 = event2.prevented.clone();
                        on_interact_outside.call(event2);

                        if !prevented2.get() {
                            on_dismiss.call(());
                        }
                    }
                }
            }) as Box<dyn FnMut(web_sys::FocusEvent)>);

            // Upstream: ownerDocument.addEventListener('focusin', handleFocus)
            let _ = document
                .add_event_listener_with_callback("focusin", closure.as_ref().unchecked_ref());

            let closure_ref = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();
            let _prevent_drop = closure;

            Box::new(move || {
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    let _ = document.remove_event_listener_with_callback("focusin", &closure_ref);
                }
                drop(_prevent_drop);
            }) as Box<dyn FnOnce()>
        });
    }

    // -----------------------------------------------------------------------
    // Body pointer-events helpers
    // -----------------------------------------------------------------------

    /// Save original body pointer-events and set to "none".
    /// Upstream: `originalBodyPointerEvents = ownerDocument.body.style.pointerEvents;`
    ///           `ownerDocument.body.style.pointerEvents = 'none';`
    pub(super) fn save_and_disable_body_pointer_events() {
        if let Some(body) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
        {
            let original = body.style().get_property_value("pointer-events").ok();
            ORIGINAL_BODY_POINTER_EVENTS.with(|o| {
                *o.borrow_mut() = original;
            });
            let _ = body.style().set_property("pointer-events", "none");
        }
    }

    /// Restore original body pointer-events.
    /// Upstream: `ownerDocument.body.style.pointerEvents = originalBodyPointerEvents;`
    pub(super) fn restore_body_pointer_events() {
        if let Some(body) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
        {
            ORIGINAL_BODY_POINTER_EVENTS.with(|o| {
                let original = o.borrow();
                if let Some(ref orig) = *original {
                    let _ = body.style().set_property("pointer-events", orig);
                } else {
                    let _ = body.style().remove_property("pointer-events");
                }
            });
        }
    }

    // -----------------------------------------------------------------------
    // Helper: check if target is inside any branch
    // -----------------------------------------------------------------------

    /// Check if a target node is contained within any registered branch.
    /// Upstream: `[...context.branches].some((branch) => branch.contains(target))`
    fn check_branches_contain_target(
        ctx: &DismissableLayerContext,
        target: Option<&web_sys::Node>,
    ) -> bool {
        let target = match target {
            Some(t) => t,
            None => return false,
        };

        let inner = ctx.inner.read();
        let inner = inner.borrow();

        let document = match web_sys::window().and_then(|w| w.document()) {
            Some(d) => d,
            None => return false,
        };

        for branch_id in &inner.branches {
            if let Some(branch_el) = document.get_element_by_id(branch_id) {
                if branch_el.contains(Some(target)) {
                    return true;
                }
            }
        }

        false
    }

    use wasm_bindgen::JsCast;
}
