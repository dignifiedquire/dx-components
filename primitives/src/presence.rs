//! Presence — line-by-line port of `@radix-ui/react-presence`.
//!
//! Animation-aware mount/unmount lifecycle. Manages CSS animation detection
//! for enter/exit transitions using a state machine with events (`MOUNT`,
//! `UNMOUNT`, `ANIMATION_OUT`, `ANIMATION_END`).
//!
//! ## Public API
//!
//! - [`Presence`] component — wraps children, conditionally renders based on
//!   animation-aware mount/unmount state. Matches upstream's exported
//!   `Presence` component.
//! - [`PresenceContext`] — provided to children so they can read the generated
//!   element ID and `present` boolean. Replaces upstream's `cloneElement` ref
//!   attachment pattern.
//!
//! ## Differences from upstream
//!
//! - **`cloneElement` ref** → [`PresenceContext`] with element ID.
//!   Upstream attaches a ref callback to the child via `cloneElement`. Dioxus
//!   doesn't support `cloneElement`, so `Presence` provides context with a
//!   generated ID. The child reads context and sets this ID on its root
//!   element, enabling the hook's web-sys DOM listeners to find the node.
//!
//! - **`useStateMachine`** → `presence_transition()` function with Rust enums.
//!   Upstream uses a generic `useReducer`-based state machine; Rust pattern
//!   matching is more natural.
//!
//! - **Render function children** → not needed.
//!   Upstream supports `children: (props: { present }) => Element` for
//!   force-mount. In Dioxus, children read `present` from [`PresenceContext`]
//!   instead.
//!
//! - **`useLayoutEffect`** → `use_effect`.
//!   Dioxus doesn't have a `useLayoutEffect` equivalent.
//!
//! - **`getElementRef`** → not needed.
//!   React 18/19 compatibility helper. Not applicable to Dioxus.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// PresenceContext — replaces upstream's cloneElement ref attachment
// ---------------------------------------------------------------------------

/// Context provided by [`Presence`] to its children.
///
/// Replaces upstream's render function `{ present }` argument.
/// Child components can read this context to access the animation-aware
/// present boolean (e.g., for `isOpen = open || present` in collapsible).
#[derive(Clone, Copy)]
pub struct PresenceContext {
    /// Animation-aware present boolean. `true` when the element is mounted or
    /// when an exit animation is in progress (unmount-suspended).
    ///
    /// Upstream: `presence.isPresent` passed via render function.
    pub present: Memo<bool>,
}

// ---------------------------------------------------------------------------
// Presence component — matches upstream's exported Presence
// ---------------------------------------------------------------------------

/// Animation-aware mount/unmount wrapper.
///
/// Upstream: `@radix-ui/react-presence` `Presence` component.
///
/// Wraps children and conditionally renders them based on animation-aware
/// presence state. When `present` transitions from `true` to `false`, keeps
/// children mounted during CSS exit animations before unmounting.
///
/// The `id` prop must match the HTML `id` set on the child's root DOM element.
/// The presence hook uses this ID to find the element for animation detection
/// via `document.get_element_by_id()`.
///
/// Provides [`PresenceContext`] to child components that need the
/// animation-aware `present` boolean (e.g., collapsible's `isOpen`).
///
/// ## Upstream
/// ```tsx
/// <Presence present={forceMount || context.open}>
///   {({ present }) => <ContentImpl present={present} ref={ref} />}
/// </Presence>
/// ```
///
/// ## Dioxus equivalent
/// ```rust,ignore
/// Presence {
///     present: force_mount || ctx.open(),
///     id: content_id,
///     div { id: content_id, "data-state": if open() { "open" } else { "closed" }, ... }
/// }
/// ```
#[component]
pub fn Presence(present: ReadSignal<bool>, id: Memo<String>, children: Element) -> Element {
    let open = use_memo(move || *present.read());
    let presence = use_presence(open, id);

    let is_present = use_memo(move || presence.is_present());

    use_context_provider(|| PresenceContext {
        present: is_present,
    });

    // Upstream: return forceMount || presence.isPresent ? cloneElement(child, { ref }) : null;
    if presence.is_present() {
        rsx! { {children} }
    } else {
        rsx! {}
    }
}

// ---------------------------------------------------------------------------
// State machine — matches upstream's useStateMachine
// ---------------------------------------------------------------------------

/// Presence state machine states.
///
/// Transition table (matches Radix's `useStateMachine` reducer):
/// ```text
/// Mounted          + UNMOUNT       → Unmounted
/// Mounted          + ANIMATION_OUT → UnmountSuspended
/// UnmountSuspended + MOUNT         → Mounted
/// UnmountSuspended + ANIMATION_END → Unmounted
/// Unmounted        + MOUNT         → Mounted
/// ```
#[derive(Clone, Copy, PartialEq, Debug)]
enum PresenceState {
    Mounted,
    UnmountSuspended,
    Unmounted,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(dead_code)] // AnimationEnd is only constructed in wasm builds
enum PresenceEvent {
    Mount,
    Unmount,
    AnimationOut,
    AnimationEnd,
}

/// State transition function matching Radix's `useStateMachine` reducer.
fn presence_transition(state: PresenceState, event: PresenceEvent) -> PresenceState {
    match (state, event) {
        (PresenceState::Mounted, PresenceEvent::Unmount) => PresenceState::Unmounted,
        (PresenceState::Mounted, PresenceEvent::AnimationOut) => PresenceState::UnmountSuspended,
        (PresenceState::UnmountSuspended, PresenceEvent::Mount) => PresenceState::Mounted,
        (PresenceState::UnmountSuspended, PresenceEvent::AnimationEnd) => PresenceState::Unmounted,
        (PresenceState::Unmounted, PresenceEvent::Mount) => PresenceState::Mounted,
        _ => state,
    }
}

/// Send a state-machine event, updating the signal only if the state changes.
fn presence_send(state: &mut Signal<PresenceState>, event: PresenceEvent) {
    let current = *state.peek();
    let next = presence_transition(current, event);
    if next != current {
        state.set(next);
    }
}

// ---------------------------------------------------------------------------
// use_presence — internal hook (not exported, matching upstream)
// ---------------------------------------------------------------------------

/// Animation-aware presence hook — line-by-line port of Radix's `usePresence`.
///
/// Internal hook — upstream does not export this; only the `Presence`
/// component is public.
///
/// Uses a state machine with events (`MOUNT`, `UNMOUNT`, `ANIMATION_OUT`,
/// `ANIMATION_END`) matching upstream's transitions. Attaches DOM event
/// listeners (`animationstart`, `animationend`, `animationcancel`) to the
/// element via web-sys to detect animation lifecycle. Reads `getComputedStyle`
/// to compare animation names when `present` changes, matching upstream's
/// synchronous detection pattern.
///
/// The component must set the element's `id` to match the `id` parameter.
fn use_presence(open: Memo<bool>, id: Memo<String>) -> UsePresence {
    // Upstream: const initialState = present ? 'mounted' : 'unmounted';
    let initial_state = if *open.peek() {
        PresenceState::Mounted
    } else {
        PresenceState::Unmounted
    };
    // Upstream: const [state, send] = useStateMachine(initialState, { ... });
    let mut state = use_signal(|| initial_state);

    // Upstream: const prevPresentRef = React.useRef(present);
    let prev_present = use_hook(|| Rc::new(Cell::new(*open.peek())));

    // Upstream: const prevAnimationNameRef = React.useRef<string>('none');
    let prev_animation_name = use_hook(|| Rc::new(RefCell::new("none".to_string())));

    // --- Effect 1: Upstream React.useEffect([state]) ---
    // Update prevAnimationNameRef when state changes.
    //
    // Upstream:
    //   const currentAnimationName = getAnimationName(stylesRef.current);
    //   prevAnimationNameRef.current = state === 'mounted' ? currentAnimationName : 'none';
    {
        let prev_anim = prev_animation_name.clone();
        use_effect(move || {
            let current_state = *state.read();
            let name = if current_state == PresenceState::Mounted {
                get_animation_name_by_id(&id())
            } else {
                "none".to_string()
            };
            *prev_anim.borrow_mut() = name;
        });
    }

    // --- Effect 2: Upstream useLayoutEffect([present, send]) ---
    // React to `present` changes with animation-aware state transitions.
    //
    // Upstream:
    //   if (hasPresentChanged) {
    //     if (present) send('MOUNT');
    //     else if (currentAnimationName === 'none' || styles?.display === 'none') send('UNMOUNT');
    //     else if (wasPresent && isAnimating) send('ANIMATION_OUT');
    //     else send('UNMOUNT');
    //   }
    {
        let prev_pres = prev_present.clone();
        let prev_anim = prev_animation_name.clone();
        use_effect(move || {
            let is_open = open();
            let was_present = prev_pres.get();
            let has_present_changed = was_present != is_open;

            if has_present_changed {
                let prev_anim_name = prev_anim.borrow().clone();
                let id_val = id();
                let current_anim_name = get_animation_name_by_id(&id_val);
                let display = get_display_by_id(&id_val);

                if is_open {
                    presence_send(&mut state, PresenceEvent::Mount);
                } else if current_anim_name == "none" || display == "none" {
                    // If there is no exit animation or the element is hidden,
                    // animations won't run so we unmount instantly
                    presence_send(&mut state, PresenceEvent::Unmount);
                } else {
                    // When `present` changes to `false`, we check changes to
                    // animation-name to determine whether an animation has started.
                    // We chose this approach (reading computed styles) because there
                    // is no `animationrun` event and `animationstart` fires after
                    // `animation-delay` has expired which would be too late.
                    let is_animating = prev_anim_name != current_anim_name;

                    if was_present && is_animating {
                        presence_send(&mut state, PresenceEvent::AnimationOut);
                    } else {
                        presence_send(&mut state, PresenceEvent::Unmount);
                    }
                }

                prev_pres.set(is_open);
            }
        });
    }

    // --- Effect 3: Upstream useLayoutEffect([node, send]) ---
    // Attach animationstart, animationcancel, animationend listeners to the node.
    //
    // In upstream this re-runs when the node ref changes. In our ID-based
    // approach, the element is found by ID. We re-run this effect when `state`
    // changes (which correlates with mount/unmount of the element). The element
    // exists in the DOM when state is Mounted or UnmountSuspended.
    //
    // Differences from upstream:
    // - Uses `CSS.escape` via `js_sys::Function` to escape animation names for
    //   comparison. Upstream: `CSS.escape(event.animationName)`.
    // - The `animationFillMode: 'forwards'` flash prevention is preserved.
    #[cfg(target_arch = "wasm32")]
    {
        let prev_pres = prev_present.clone();
        let prev_anim = prev_animation_name.clone();
        crate::use_effect_with_cleanup(move || {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;

            let current_state = *state.read();

            // Only attach listeners when the element is in the DOM
            let element = if current_state != PresenceState::Unmounted {
                web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.get_element_by_id(&id()))
                    .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
            } else {
                None
            };

            let Some(node) = element else {
                // Upstream: send('ANIMATION_END') when node is removed prematurely
                presence_send(&mut state, PresenceEvent::AnimationEnd);
                return Box::new(|| {}) as Box<dyn FnOnce()>;
            };

            let owner_window = node
                .owner_document()
                .and_then(|d| d.default_view())
                .or_else(web_sys::window)
                .expect("window");

            let timeout_id: Rc<Cell<i32>> = Rc::new(Cell::new(0));

            // Upstream: const handleAnimationEnd = (event: AnimationEvent) => { ... }
            let node_end = node.clone();
            let prev_pres_end = prev_pres.clone();
            let timeout_end = timeout_id.clone();
            let owner_window_end = owner_window.clone();
            let closure_end = Closure::wrap(Box::new(move |event: web_sys::AnimationEvent| {
                let current_anim_name = get_animation_name_for_element(&node_end);
                // Upstream: CSS.escape(event.animationName)
                let escaped = css_escape(&event.animation_name());
                let is_current_animation = current_anim_name.contains(&escaped);

                let target = event.target();
                let is_target = target
                    .as_ref()
                    .and_then(|t| t.dyn_ref::<web_sys::HtmlElement>())
                    .map_or(false, |t| *t == node_end);

                if is_target && is_current_animation {
                    presence_send(&mut state, PresenceEvent::AnimationEnd);

                    // Upstream: if (!prevPresentRef.current) { ... animationFillMode ... }
                    if !prev_pres_end.get() {
                        let current_fill_mode = node_end
                            .style()
                            .get_property_value("animation-fill-mode")
                            .unwrap_or_default();
                        let _ = node_end
                            .style()
                            .set_property("animation-fill-mode", "forwards");

                        // Reset the style after the node had time to unmount
                        let node_reset = node_end.clone();
                        let fill_reset = current_fill_mode;
                        let tid = owner_window_end
                            .set_timeout_with_callback(
                                &Closure::once_into_js(move || {
                                    let current = node_reset
                                        .style()
                                        .get_property_value("animation-fill-mode")
                                        .unwrap_or_default();
                                    if current == "forwards" {
                                        let _ = node_reset
                                            .style()
                                            .set_property("animation-fill-mode", &fill_reset);
                                    }
                                })
                                .unchecked_into(),
                            )
                            .unwrap_or(0);
                        timeout_end.set(tid);
                    }
                }
            })
                as Box<dyn FnMut(web_sys::AnimationEvent)>);

            // Upstream: const handleAnimationStart = (event: AnimationEvent) => { ... }
            let node_start = node.clone();
            let prev_anim_start = prev_anim.clone();
            let closure_start = Closure::wrap(Box::new(move |event: web_sys::AnimationEvent| {
                let target = event.target();
                let is_target = target
                    .as_ref()
                    .and_then(|t| t.dyn_ref::<web_sys::HtmlElement>())
                    .map_or(false, |t| *t == node_start);

                if is_target {
                    // Upstream: prevAnimationNameRef.current = getAnimationName(stylesRef.current)
                    *prev_anim_start.borrow_mut() = get_animation_name_for_element(&node_start);
                }
            })
                as Box<dyn FnMut(web_sys::AnimationEvent)>);

            // Upstream:
            //   node.addEventListener('animationstart', handleAnimationStart);
            //   node.addEventListener('animationcancel', handleAnimationEnd);
            //   node.addEventListener('animationend', handleAnimationEnd);
            let _ = node.add_event_listener_with_callback(
                "animationstart",
                closure_start.as_ref().unchecked_ref(),
            );
            let _ = node.add_event_listener_with_callback(
                "animationcancel",
                closure_end.as_ref().unchecked_ref(),
            );
            let _ = node.add_event_listener_with_callback(
                "animationend",
                closure_end.as_ref().unchecked_ref(),
            );

            // Save references for cleanup
            let start_fn = closure_start
                .as_ref()
                .unchecked_ref::<js_sys::Function>()
                .clone();
            let end_fn = closure_end
                .as_ref()
                .unchecked_ref::<js_sys::Function>()
                .clone();
            let _keep_start = closure_start;
            let _keep_end = closure_end;

            // Cleanup: upstream return () => { ... }
            Box::new(move || {
                owner_window.clear_timeout_with_handle(timeout_id.get());
                let _ = node.remove_event_listener_with_callback("animationstart", &start_fn);
                let _ = node.remove_event_listener_with_callback("animationcancel", &end_fn);
                let _ = node.remove_event_listener_with_callback("animationend", &end_fn);
                drop(_keep_start);
                drop(_keep_end);
            }) as Box<dyn FnOnce()>
        });
    }

    // Non-wasm: no animation detection possible, react to open directly.
    #[cfg(not(target_arch = "wasm32"))]
    {
        use_effect(move || {
            if open() {
                presence_send(&mut state, PresenceEvent::Mount);
            } else {
                presence_send(&mut state, PresenceEvent::Unmount);
            }
        });
    }

    UsePresence { state }
}

// ---------------------------------------------------------------------------
// UsePresence — returned by use_presence (internal)
// ---------------------------------------------------------------------------

/// Returned by [`use_presence`]. Animation-aware mount/unmount lifecycle.
#[derive(Clone, Copy)]
struct UsePresence {
    state: Signal<PresenceState>,
}

impl UsePresence {
    /// Whether the element should be present in the DOM.
    /// True when mounted or when exit animation is in progress (unmount-suspended).
    ///
    /// Upstream: `['mounted', 'unmountSuspended'].includes(state)`
    fn is_present(&self) -> bool {
        !matches!(*self.state.read(), PresenceState::Unmounted)
    }
}

// ---------------------------------------------------------------------------
// Helper functions — upstream getAnimationName + CSS.escape
// ---------------------------------------------------------------------------

/// Upstream: `function getAnimationName(styles)`
/// Returns the `animation-name` computed style, defaulting to `"none"`.
#[cfg(target_arch = "wasm32")]
fn get_animation_name_for_element(element: &web_sys::HtmlElement) -> String {
    web_sys::window()
        .and_then(|w| w.get_computed_style(element).ok().flatten())
        .and_then(|s| s.get_property_value("animation-name").ok())
        .unwrap_or_else(|| "none".to_string())
}

/// Get animation-name for an element found by ID.
#[cfg(target_arch = "wasm32")]
fn get_animation_name_by_id(id: &str) -> String {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
        .and_then(|e| {
            use wasm_bindgen::JsCast;
            e.dyn_into::<web_sys::HtmlElement>().ok()
        })
        .map(|el| get_animation_name_for_element(&el))
        .unwrap_or_else(|| "none".to_string())
}

/// Get `display` computed style for an element found by ID.
#[cfg(target_arch = "wasm32")]
fn get_display_by_id(id: &str) -> String {
    web_sys::window()
        .and_then(|w| {
            let doc = w.document()?;
            let el = doc.get_element_by_id(id)?;
            w.get_computed_style(&el).ok().flatten()
        })
        .and_then(|s| s.get_property_value("display").ok())
        .unwrap_or_default()
}

/// Port of `CSS.escape()` — escapes a string for use in CSS identifiers.
/// Used to compare `event.animationName` with computed `animation-name`.
#[cfg(target_arch = "wasm32")]
fn css_escape(value: &str) -> String {
    use wasm_bindgen::JsCast;
    use wasm_bindgen::JsValue;

    // Use the browser's CSS.escape() via js_sys
    let css = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("CSS"))
        .unwrap_or(JsValue::UNDEFINED);
    if css.is_undefined() {
        return value.to_string();
    }
    let escape_fn = js_sys::Reflect::get(&css, &JsValue::from_str("escape")).unwrap_or_default();
    if let Some(func) = escape_fn.dyn_ref::<js_sys::Function>() {
        let result: JsValue = func
            .call1(&css, &JsValue::from_str(value))
            .unwrap_or_default();
        result.as_string().unwrap_or_else(|| value.to_string())
    } else {
        value.to_string()
    }
}

// Non-wasm stubs for conditional compilation.
// Effects 1 and 2 run on all targets and call these helpers.
#[cfg(not(target_arch = "wasm32"))]
fn get_animation_name_by_id(_id: &str) -> String {
    "none".to_string()
}

#[cfg(not(target_arch = "wasm32"))]
fn get_display_by_id(_id: &str) -> String {
    "block".to_string()
}
