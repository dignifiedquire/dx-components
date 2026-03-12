#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::core::{current_scope_id, use_drop};
use dioxus::prelude::*;

use dioxus_core::AttributeValue::Text;
use dioxus_elements::geometry::PixelsRect;
use time::OffsetDateTime;

pub use dioxus_attributes;

pub mod accessible_icon;
pub mod accordion;
pub mod alert_dialog;
pub mod announce;
mod aria_hidden;
pub mod arrow;
pub mod aspect_ratio;
pub mod avatar;
pub mod button;
pub mod calendar;
pub mod carousel;
pub mod checkbox;
pub mod collapsible;
mod collection;
pub mod combobox;
pub mod command;
pub mod context_menu;
pub mod date_picker;
pub mod dialog;
pub mod direction;
pub mod dismissable_layer;
pub mod drag_and_drop_list;
pub mod dropdown_menu;
mod focus;
pub mod focus_guards;
pub mod focus_scope;
pub mod form;
pub mod hover_card;
pub mod icon;
pub mod input_otp;
pub mod label;
pub(crate) mod menu;
pub mod menubar;
#[cfg(feature = "router")]
pub mod navbar;
pub mod navigation_menu;
pub mod number;
pub mod password_toggle_field;
pub mod popover;
pub mod popper;
pub mod portal;
pub mod primitive;
pub mod progress;
pub mod radio_group;
pub mod resizable;
pub mod roving_focus;
pub mod scroll_area;
mod scroll_lock;
pub mod select;
pub mod separator;
pub mod slider;
pub(crate) mod slot;
pub mod switch;
pub mod tabs;
pub mod toast;
pub mod toggle;
pub mod toggle_group;
pub mod toolbar;
pub mod tooltip;
pub(crate) mod typeahead;
pub mod visually_hidden;

/// Generate a runtime-unique id.
fn use_unique_id() -> Signal<String> {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    #[allow(unused_mut)]
    let mut initial_value = use_hook(|| {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let id_str = format!("dxc-{id}");
        id_str
    });

    fullstack! {
        let server_id = dioxus::prelude::use_server_cached(move || {
            initial_value.clone()
        });
        initial_value = server_id;
    }
    use_signal(|| initial_value)
}

// Elements can only have one id so if the user provides their own, we must use it as the aria id.
fn use_id_or<T: Clone + PartialEq + 'static>(
    mut gen_id: Signal<T>,
    user_id: ReadSignal<Option<T>>,
) -> Memo<T> {
    // First, check if we have a user-provided ID
    let has_user_id = use_memo(move || user_id().is_some());

    // If we have a user ID, update the gen_id in an effect
    use_effect(move || {
        if let Some(id) = user_id() {
            gen_id.set(id);
        }
    });

    // Return the appropriate ID
    use_memo(move || {
        if has_user_id() {
            user_id().unwrap()
        } else {
            gen_id.peek().clone()
        }
    })
}

/// Allows some state to be either controlled or uncontrolled.
pub fn use_controlled<T: Clone + PartialEq + 'static>(
    prop: ReadSignal<Option<T>>,
    default: T,
    on_change: Callback<T>,
) -> (Memo<T>, Callback<T>) {
    let mut internal_value = use_signal(|| prop.cloned().unwrap_or(default));
    let value = use_memo(move || prop.cloned().unwrap_or_else(&*internal_value));

    let set_value = use_callback(move |x: T| {
        internal_value.set(x.clone());
        on_change.call(x);
    });

    (value, set_value)
}

/// Returns the previous value of a reactive signal.
///
/// Matches Radix's `usePrevious(value)`. On each render, if `value` has changed,
/// the previous value is stored and returned. The initial previous value equals
/// the initial `value`.
pub fn use_previous<T: Clone + PartialEq + 'static>(value: ReadSignal<T>) -> Memo<T> {
    let mut prev = use_signal(|| value.cloned());
    let mut last_seen = use_signal(|| value.cloned());

    use_memo(move || {
        let current = value.cloned();
        let seen = last_seen.cloned();
        if current != seen {
            prev.set(seen);
            last_seen.set(current);
        }
        prev.cloned()
    })
}

/// Returns `true` once the component has mounted on the client.
///
/// Matches Radix's `useIsHydrated()`: returns `false` during SSR and on
/// the initial server render, `true` after the component mounts in the browser.
pub fn use_is_hydrated() -> ReadSignal<bool> {
    let mut hydrated = use_signal(|| false);
    use_effect(move || {
        hydrated.set(true);
    });
    hydrated.into()
}

/// Element size returned by [`use_size`].
///
/// Matches Radix's `useSize(element)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElementSize {
    /// Width in pixels.
    pub width: f64,
    /// Height in pixels.
    pub height: f64,
}

/// Reads an element's size from its [`MountedData`].
///
/// Matches Radix's `useSize(element)`. Returns `None` until the element is
/// mounted, then queries `get_client_rect()` reactively.
///
/// Pass a `Signal<Option<Rc<MountedData>>>` obtained via `onmounted`.
pub fn use_size(mounted: ReadSignal<Option<Rc<MountedData>>>) -> ReadSignal<Option<ElementSize>> {
    let mut size: Signal<Option<ElementSize>> = use_signal(|| None);

    use_effect(move || {
        if let Some(md) = mounted.cloned() {
            spawn(async move {
                if let Ok(rect) = md.get_client_rect().await {
                    size.set(Some(ElementSize {
                        width: rect.size.width,
                        height: rect.size.height,
                    }));
                }
            });
        } else {
            size.set(None);
        }
    });

    size.into()
}

/// Reads an element's bounding client rect from its [`MountedData`].
///
/// Matches Radix's `useRect(measurable)` which uses `observeElementRect`.
/// Returns `None` until the element is mounted, then queries
/// `get_client_rect()` reactively.
///
/// Pass a `Signal<Option<Rc<MountedData>>>` obtained via `onmounted`.
pub fn use_rect(mounted: ReadSignal<Option<Rc<MountedData>>>) -> ReadSignal<Option<PixelsRect>> {
    let mut rect: Signal<Option<PixelsRect>> = use_signal(|| None);

    use_effect(move || {
        if let Some(md) = mounted.cloned() {
            spawn(async move {
                if let Ok(r) = md.get_client_rect().await {
                    rect.set(Some(r));
                }
            });
        } else {
            rect.set(None);
        }
    });

    rect.into()
}

/// Run some cleanup code when the component is unmounted if the effect was run.
fn use_effect_cleanup<F: FnOnce() + 'static>(#[allow(unused)] cleanup: F) {
    client!(crate::dioxus_core::use_drop(cleanup))
}

/// Run some cleanup code when the component is unmounted if the effect was run.
fn use_effect_with_cleanup<F: FnMut() -> C + 'static, C: FnOnce() + 'static>(mut effect: F) {
    let mut cleanup = use_hook(|| CopyValue::new(None as Option<C>));
    use_effect(move || {
        if let Some(cleanup) = cleanup.take() {
            cleanup();
        }
        cleanup.set(Some(effect()));
    });
    client!(crate::dioxus_core::use_drop(move || {
        if let Some(cleanup) = cleanup.take() {
            cleanup();
        }
    }))
}

/// A stack of escape listeners to allow only the top-most listener to be called.
#[derive(Clone)]
struct EscapeListenerStack(Rc<RefCell<Vec<ScopeId>>>);

fn use_global_escape_listener(mut on_escape: impl FnMut() + Clone + 'static) {
    let scope_id = current_scope_id();
    let stack = use_hook(move || {
        // Get or create the escape listener stack
        let stack: EscapeListenerStack = try_consume_context()
            .unwrap_or_else(|| provide_context(EscapeListenerStack(Default::default())));
        // Push the current scope onto the stack
        stack.0.borrow_mut().push(scope_id);
        stack
    });
    // Remove the current scope id from the stack when we unmount
    use_drop({
        let stack = stack.clone();
        move || {
            let mut stack = stack.0.borrow_mut();
            stack.retain(|id| *id != scope_id);
        }
    });
    use_global_keydown_listener("Escape", move || {
        // Only call the listener if this component is on top of the stack
        let stack = stack.0.borrow();
        if stack.last() == Some(&scope_id) {
            on_escape();
        }
    });
}

fn use_global_keydown_listener(key: &'static str, on_escape: impl FnMut() + Clone + 'static) {
    use_effect_with_cleanup(move || {
        let mut escape = document::eval(
            "let targetKey = await dioxus.recv();
            function listener(event) {
                if (event.key === targetKey) {
                    event.preventDefault();
                    dioxus.send(true);
                }
            }
            document.addEventListener('keydown', listener);
            await dioxus.recv();
            document.removeEventListener('keydown', listener);",
        );
        let _ = escape.send(key);
        let mut on_escape = on_escape.clone();
        spawn(async move {
            while let Ok(true) = escape.recv().await {
                on_escape();
            }
        });
        move || _ = escape.send(true)
    });
}

// Deprecated: use `use_presence` instead. This legacy hook will be removed
// once all remaining consumers (navbar, menubar, context_menu, dropdown_menu,
// select) are migrated in Phase 2e/2f.
fn use_animated_open(
    id: impl Readable<Target = String> + Copy + 'static,
    open: impl Readable<Target = bool> + Copy + 'static,
) -> impl Fn() -> bool + Copy {
    let animating = use_signal(|| false);

    // Show in dom is a few frames behind the open signal to allow for the animation to start.
    // If it does start, we wait for the animation to finish before removing the element from the DOM.
    // Initialize to the current open state so SSR renders correctly.
    let mut show_in_dom = use_signal(|| *open.peek());

    use_effect(move || {
        let open = open.cloned();
        if open {
            show_in_dom.set(open);
        } else {
            spawn(async move {
                let id = id.cloned();
                let mut eval = dioxus::document::eval(
                    "const id = await dioxus.recv();
                    const element = document.getElementById(id);
                    if (element && element.getAnimations().length > 0) {
                        Promise.all(element.getAnimations().map((animation) => animation.finished)).then(() => {
                            dioxus.send(true);
                        });
                    } else {
                        dioxus.send(true);
                    }"
                );
                let _ = eval.send(id);
                _ = eval.recv::<bool>().await;
                show_in_dom.set(open);
            });
        }
    });

    move || show_in_dom() || animating()
}

// ---------------------------------------------------------------------------
// Delayed open/close — shared by Tooltip, HoverCard, NavigationMenu
// ---------------------------------------------------------------------------

/// Handle returned by [`use_delayed_open`] for controlling delayed open/close.
#[derive(Clone, Copy)]
pub(crate) struct DelayedOpenHandle {
    /// The current open state.
    pub open: Memo<bool>,
    /// Set the open state directly (also fires `on_open_change`).
    pub set_open: Callback<bool>,
    /// Start the open timer (opens after `open_delay`).
    pub handle_delayed_open: Callback<()>,
    /// Start the close timer (closes after `close_delay`).
    pub handle_delayed_close: Callback<()>,
    /// Open immediately, cancelling any pending timers.
    pub handle_immediate_open: Callback<()>,
    /// Close immediately, cancelling any pending timers.
    pub handle_immediate_close: Callback<()>,
}

/// Hook for delayed open/close with generation-counter cancellation.
///
/// Used by Tooltip, HoverCard, and NavigationMenu to delay open/close
/// on hover, matching Radix's timer-based approach.
pub(crate) fn use_delayed_open(
    controlled_open: ReadSignal<Option<bool>>,
    default_open: bool,
    on_open_change: Callback<bool>,
    open_delay_ms: u64,
    close_delay_ms: u64,
) -> DelayedOpenHandle {
    use std::time::Duration;

    let (open, set_open) = use_controlled(controlled_open, default_open, on_open_change);

    // Generation counter for cancelling pending timers.
    // Bumped on every timer start or cancel — stale spawns check their
    // generation against current and bail if mismatched.
    let mut open_gen = use_signal(|| 0u64);
    let mut close_gen = use_signal(|| 0u64);

    let cancel_timers = use_callback(move |_: ()| {
        open_gen += 1;
        close_gen += 1;
    });

    let handle_immediate_open = use_callback(move |_: ()| {
        cancel_timers.call(());
        set_open.call(true);
    });

    let handle_immediate_close = use_callback(move |_: ()| {
        cancel_timers.call(());
        set_open.call(false);
    });

    let handle_delayed_open = use_callback(move |_: ()| {
        close_gen += 1; // cancel any pending close
        if open_delay_ms == 0 {
            set_open.call(true);
            return;
        }
        open_gen += 1;
        let gen = open_gen();
        spawn(async move {
            dioxus_sdk_time::sleep(Duration::from_millis(open_delay_ms)).await;
            if open_gen() == gen {
                set_open.call(true);
            }
        });
    });

    let handle_delayed_close = use_callback(move |_: ()| {
        open_gen += 1; // cancel any pending open
        if close_delay_ms == 0 {
            set_open.call(false);
            return;
        }
        close_gen += 1;
        let gen = close_gen();
        spawn(async move {
            dioxus_sdk_time::sleep(Duration::from_millis(close_delay_ms)).await;
            if close_gen() == gen {
                set_open.call(false);
            }
        });
    });

    DelayedOpenHandle {
        open,
        set_open,
        handle_delayed_open,
        handle_delayed_close,
        handle_immediate_open,
        handle_immediate_close,
    }
}

// ---------------------------------------------------------------------------
// Presence state machine — matches Radix's useStateMachine + usePresence
// (radix-ui-primitives/packages/react/presence/src/presence.tsx)
// ---------------------------------------------------------------------------

/// Presence state machine states.
///
/// Transition table (matches Radix's `useStateMachine`):
/// ```text
/// Mounted          + AnimationOut → UnmountSuspended
/// Mounted          + Unmount      → Unmounted
/// UnmountSuspended + Mount        → Mounted
/// UnmountSuspended + AnimationEnd → Unmounted
/// Unmounted        + Mount        → Mounted
/// ```
#[derive(Clone, Copy, PartialEq, Debug)]
enum PresenceState {
    Mounted,
    UnmountSuspended,
    Unmounted,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(dead_code)]
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

/// Returned by [`use_presence`]. Animation-aware mount/unmount lifecycle.
#[derive(Clone, Copy)]
pub(crate) struct UsePresence {
    state: Signal<PresenceState>,
    open: Memo<bool>,
}

impl UsePresence {
    /// Whether the element should be present in the DOM.
    /// True when mounted or when exit animation is in progress (unmount-suspended).
    pub fn is_present(&self) -> bool {
        !matches!(*self.state.read(), PresenceState::Unmounted)
    }

    /// Returns `"open"` or `"closed"` for the `data-state` attribute.
    pub fn data_state(&self) -> &'static str {
        if (self.open)() {
            "open"
        } else {
            "closed"
        }
    }

    /// Must be called from the element's `onanimationend` handler.
    /// Transitions `UnmountSuspended` → `Unmounted` when exit animation finishes.
    pub fn on_animation_end(&mut self) {
        self.send(PresenceEvent::AnimationEnd);
    }

    fn send(&mut self, event: PresenceEvent) {
        let current = *self.state.peek();
        let next = presence_transition(current, event);
        if next != current {
            self.state.set(next);
        }
    }
}

/// Animation-aware presence hook matching Radix's `usePresence`.
///
/// Uses a state machine with events (`Mount`, `Unmount`, `AnimationOut`, `AnimationEnd`)
/// matching Radix's transitions. When `open` changes to `false`, checks via JS eval
/// whether a CSS animation exists on the element (by `id`). If an exit animation is
/// detected, stays in `UnmountSuspended` until [`UsePresence::on_animation_end`] is
/// called. If no animation exists, transitions directly to `Unmounted`.
///
/// The component must:
/// 1. Set the element's `id` to match the `id` parameter
/// 2. Call [`UsePresence::on_animation_end`] from `onanimationend`
pub(crate) fn use_presence(open: Memo<bool>, id: Memo<String>) -> UsePresence {
    let initial_state = if *open.peek() {
        PresenceState::Mounted
    } else {
        PresenceState::Unmounted
    };
    let mut state = use_signal(|| initial_state);

    // Generation counter to prevent stale async callbacks from applying.
    let mut gen = use_signal(|| 0u64);

    // Radix: useLayoutEffect([present, send])
    // Reacts to open changes with animation-aware state transitions.
    use_effect(move || {
        let is_open = open();
        let current = *state.peek();
        let my_gen = *gen.peek() + 1;
        gen.set(my_gen);

        if is_open {
            // Opening → Mount
            let next = presence_transition(current, PresenceEvent::Mount);
            if next != current {
                state.set(next);
            }
        } else if current == PresenceState::Mounted {
            // Closing → optimistically AnimationOut (assume animation exists)
            state.set(PresenceState::UnmountSuspended);

            // Radix: reads getComputedStyle(node).animationName to detect exit animation.
            // Dioxus adaptation: async check after one frame for CSS rules to apply.
            let id_val = id();
            spawn(async move {
                // Wait one frame for CSS to reflect new data-state
                let mut raf = document::eval("requestAnimationFrame(() => dioxus.send(true))");
                let _ = raf.recv::<bool>().await;

                if *gen.peek() != my_gen {
                    return;
                }

                // Check computed animation-name (matches Radix's getAnimationName)
                let js = format!(
                    "var e=document.getElementById('{id_val}');\
                     if(e){{var s=getComputedStyle(e);\
                     dioxus.send(s.animationName!=='none'&&s.display!=='none')}}\
                     else{{dioxus.send(false)}}"
                );
                let mut eval = document::eval(&js);
                if let Ok(has_anim) = eval.recv::<bool>().await {
                    if *gen.peek() != my_gen {
                        return;
                    }
                    if !has_anim && *state.peek() == PresenceState::UnmountSuspended {
                        // No exit animation → immediate unmount
                        // (Radix: send('UNMOUNT') instead of send('ANIMATION_OUT'))
                        state.set(PresenceState::Unmounted);
                    }
                }
            });
        }
    });

    UsePresence { state, open }
}

// ---------------------------------------------------------------------------
// Collapsible content dimensions — matches Radix's CollapsibleContentImpl
// (radix-ui-primitives/packages/react/collapsible/src/collapsible.tsx)
// ---------------------------------------------------------------------------

/// Returned by [`use_collapsible_content_dimensions`].
/// Manages dimension measurement, CSS variables, and mount animation suppression.
pub(crate) struct UseCollapsibleDimensions {
    height: Signal<Option<f64>>,
    width: Signal<Option<f64>>,
    /// Non-reactive ref matching Radix's `isMountAnimationPreventedRef`.
    /// Cleared via rAF + direct DOM manipulation (not via re-render).
    suppress_mount_anim: Rc<Cell<bool>>,
}

impl UseCollapsibleDimensions {
    /// Compute the inline style string.
    ///
    /// Includes `--radix-collapsible-content-height/width` CSS custom properties
    /// and `animation-name: none` during initial mount suppression.
    /// Optionally appends `extra` style (e.g., accordion CSS variable aliases).
    pub fn style(&self, extra: Option<&str>) -> String {
        let mut parts = Vec::new();
        if let Some(h) = *self.height.read() {
            parts.push(format!("--radix-collapsible-content-height: {h}px"));
        }
        if let Some(w) = *self.width.read() {
            parts.push(format!("--radix-collapsible-content-width: {w}px"));
        }
        if self.suppress_mount_anim.get() {
            parts.push("animation-name: none".to_string());
        }
        if let Some(s) = extra {
            if !s.is_empty() {
                parts.push(s.to_string());
            }
        }
        parts.join("; ")
    }
}

/// Dimension measurement hook for collapsible content, matching Radix's
/// `CollapsibleContentImpl` inline logic.
///
/// Handles:
/// - **Mount animation prevention** (`isMountAnimationPreventedRef`): prevents the
///   open animation on initial page render via inline `animation-name: none`,
///   cleared after one frame via rAF + direct DOM manipulation (matching Radix's
///   imperative style management in `useLayoutEffect`).
/// - **Dimension measurement**: measures element height/width using `scrollHeight`/
///   `scrollWidth` and sets `--radix-collapsible-content-height/width` CSS custom
///   properties directly on the DOM node.
///
/// The component must set the element's `id` to match the `id` parameter.
pub(crate) fn use_collapsible_content_dimensions(
    id: Memo<String>,
    open: Memo<bool>,
) -> UseCollapsibleDimensions {
    let mut content_height = use_signal(|| None::<f64>);
    let mut content_width = use_signal(|| None::<f64>);

    // Radix: isMountAnimationPreventedRef = React.useRef(isOpen)
    // Non-reactive ref — cleared via rAF + direct DOM manipulation, not re-render.
    // This matches Radix which uses a ref (not state) for this flag.
    let suppress_mount_anim = use_hook(|| Rc::new(Cell::new(*open.peek())));

    // Generation counter for stale async measurement protection.
    let mut measurement_gen = use_signal(|| 0u64);

    // Clear mount animation suppression after first frame (matches Radix rAF).
    // Uses direct DOM manipulation to remove `animation-name: none` from the
    // element's inline style, since this is a non-reactive ref (like Radix's
    // useRef) and changing it doesn't trigger a Dioxus re-render.
    {
        let suppress = suppress_mount_anim.clone();
        use_effect(move || {
            if suppress.get() {
                let id_val = id();
                let suppress = suppress.clone();
                spawn(async move {
                    let mut raf = document::eval("requestAnimationFrame(() => dioxus.send(true))");
                    let _ = raf.recv::<bool>().await;
                    if suppress.get() {
                        suppress.set(false);
                        // Directly remove animation-name from DOM element's inline style.
                        // Matches Radix's imperative style restore in useLayoutEffect.
                        let js = format!(
                            "var e=document.getElementById('{id_val}');\
                             if(e)e.style.removeProperty('animation-name')"
                        );
                        _ = document::eval(&js);
                    }
                });
            }
        });
    }

    // Measurement effect — runs when open/id changes.
    // Uses scrollHeight/scrollWidth which return the full content dimensions
    // even while a CSS animation is in progress (they measure the content,
    // not the rendered box). This avoids touching animationName which would
    // cancel the running CSS animation (unlike Radix's useLayoutEffect which
    // runs synchronously before paint).
    {
        let suppress = suppress_mount_anim.clone();
        let is_first_run = use_hook(|| Rc::new(Cell::new(true)));
        use_effect(move || {
            let _open = open(); // Subscribe to open changes
            let id_val = id();

            // On 2nd+ run (state change after mount), clear suppress flag.
            // This handles the edge case where the user interacts before rAF fires.
            if is_first_run.get() {
                is_first_run.set(false);
            } else {
                suppress.set(false);
            }

            let gen = *measurement_gen.peek() + 1;
            measurement_gen.set(gen);

            spawn(async move {
                let js = format!(
                    "var e=document.getElementById('{id_val}');\
                     if(e){{\
                       var h=e.scrollHeight,w=e.scrollWidth;\
                       if(h>0)e.style.setProperty('--radix-collapsible-content-height',h+'px');\
                       if(w>0)e.style.setProperty('--radix-collapsible-content-width',w+'px');\
                       dioxus.send([h,w])\
                     }}else{{dioxus.send([0.0,0.0])}}"
                );
                let mut eval = document::eval(&js);
                if let Ok(dims) = eval.recv::<Vec<f64>>().await {
                    if *measurement_gen.peek() != gen {
                        return;
                    }
                    if dims.len() == 2 {
                        if dims[0] > 0.0 {
                            content_height.set(Some(dims[0]));
                        }
                        if dims[1] > 0.0 {
                            content_width.set(Some(dims[1]));
                        }
                    }
                }
            });
        });
    }

    UseCollapsibleDimensions {
        height: content_height,
        width: content_width,
        suppress_mount_anim,
    }
}

/// The side where the content will be displayed relative to the trigger
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContentSide {
    /// The content will appear above the trigger
    Top,
    /// The content will appear to the right of the trigger
    Right,
    /// The content will appear below the trigger
    Bottom,
    /// The content will appear to the left of the trigger
    Left,
}

impl ContentSide {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }
}

/// The alignment of the content relative to the trigger
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContentAlign {
    /// The content will be aligned to the start of the trigger
    Start,
    /// The content will be centered relative to the trigger
    Center,
    /// The content will be aligned to the end of the trigger
    End,
}

impl ContentAlign {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

pub(crate) trait LocalDateExt {
    /// A small extension method function to get the local date with a fallback to UTC date if this fails
    fn now_local_date() -> time::Date;
}

impl LocalDateExt for time::OffsetDateTime {
    fn now_local_date() -> time::Date {
        OffsetDateTime::now_local()
            .map(|x| x.date())
            .unwrap_or_else(|_| time::UtcDateTime::now().date())
    }
}

/// Merge multiple attribute vectors.
///
/// This is the Dioxus equivalent of Radix's `mergeProps`. Rules:
/// - `class`: concatenated with a single space separator (trimmed). Last wins for volatility flag.
/// - `style`: concatenated with `"; "` separator (trimmed). Allows multiple sources to contribute
///   inline styles without overwriting each other.
/// - All other attributes: last occurrence wins for the same `(name, namespace)` pair.
///
/// Note: event handler attributes (`AttributeValue::Listener`) follow last-writer-wins like
/// other attributes. In practice, event handlers are typically separate component props
/// (not captured by `extends = GlobalAttributes`), so collisions are rare. The component's
/// internal handlers are included via the `attributes!` macro and passed through `r#as`.
pub fn merge_attributes(mut lists: Vec<Vec<Attribute>>) -> Vec<Attribute> {
    let mut merged = Vec::new();
    // The inputs are usually sorted by name, so we can do a k-way merge cheaply
    for list in &mut lists {
        list.sort_by_key(|a| a.name);
    }
    let mut iters: Vec<_> = lists
        .into_iter()
        .map(|l| l.into_iter().peekable())
        .collect();

    loop {
        // Find the minimum name among all current heads
        let min_name = iters
            .iter_mut()
            .filter_map(|it| it.peek().map(|a| a.name))
            .min();

        let Some(min_name) = min_name else {
            break;
        };

        // Collect all attributes with this name, grouped by namespace
        let mut by_namespace: Vec<Attribute> = Vec::new();

        for iter in &mut iters {
            while iter.peek().map(|a| a.name) == Some(min_name) {
                let attr = iter.next().unwrap();
                if let Some(existing) = by_namespace
                    .iter_mut()
                    .find(|a| a.namespace == attr.namespace)
                {
                    if attr.name == "class" {
                        let was_volatile = existing.volatile;
                        *existing = match (&existing.value, &attr.value) {
                            (Text(a), Text(b)) => Attribute {
                                name: attr.name,
                                namespace: attr.namespace,
                                volatile: was_volatile || attr.volatile,
                                value: Text(join_class(a, b)),
                            },
                            _ => attr,
                        };
                    } else if attr.name == "style" {
                        let was_volatile = existing.volatile;
                        *existing = match (&existing.value, &attr.value) {
                            (Text(a), Text(b)) => Attribute {
                                name: attr.name,
                                namespace: attr.namespace,
                                volatile: was_volatile || attr.volatile,
                                value: Text(join_style(a, b)),
                            },
                            _ => attr,
                        };
                    } else {
                        *existing = attr;
                    }
                } else {
                    by_namespace.push(attr);
                }
            }
        }

        merged.extend(by_namespace);
    }

    merged
}

fn join_class(a: &str, b: &str) -> String {
    let (a, b) = (a.trim(), b.trim());
    if !a.is_empty() && !b.is_empty() {
        format!("{a} {b}")
    } else {
        format!("{a}{b}")
    }
}

fn join_style(a: &str, b: &str) -> String {
    let a = a.trim().trim_end_matches(';').trim();
    let b = b.trim().trim_end_matches(';').trim();
    if !a.is_empty() && !b.is_empty() {
        format!("{a}; {b}")
    } else {
        format!("{a}{b}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn attr(name: &'static str, value: &str) -> Attribute {
        Attribute {
            name,
            namespace: None,
            volatile: false,
            value: Text(value.to_string()),
        }
    }

    fn get_value(attr: &Attribute) -> &str {
        match &attr.value {
            Text(s) => s,
            _ => panic!("expected Text"),
        }
    }

    #[test]
    fn merge_empty_lists() {
        let result = merge_attributes(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn merge_single_list() {
        let result = merge_attributes(vec![vec![attr("a", "1"), attr("b", "2")]]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "a");
        assert_eq!(result[1].name, "b");
    }

    #[test]
    fn merge_preserves_sorted_order() {
        let result = merge_attributes(vec![
            vec![attr("a", "1"), attr("c", "3")],
            vec![attr("b", "2"), attr("d", "4")],
        ]);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].name, "a");
        assert_eq!(result[1].name, "b");
        assert_eq!(result[2].name, "c");
        assert_eq!(result[3].name, "d");
    }

    #[test]
    fn later_list_overwrites() {
        let result = merge_attributes(vec![vec![attr("a", "first")], vec![attr("a", "second")]]);
        assert_eq!(result.len(), 1);
        assert_eq!(get_value(&result[0]), "second");
    }

    #[test]
    fn class_attributes_are_merged() {
        let result = merge_attributes(vec![vec![attr("class", "foo")], vec![attr("class", "bar")]]);
        assert_eq!(result.len(), 1);
        assert_eq!(get_value(&result[0]), "foo bar");
    }

    #[test]
    fn class_merge_trims_whitespace() {
        let result = merge_attributes(vec![
            vec![attr("class", "  foo  ")],
            vec![attr("class", "  bar  ")],
        ]);
        assert_eq!(get_value(&result[0]), "foo bar");
    }

    #[test]
    fn class_merge_handles_empty() {
        let result = merge_attributes(vec![vec![attr("class", "")], vec![attr("class", "bar")]]);
        assert_eq!(get_value(&result[0]), "bar");
    }

    #[test]
    fn mixed_attributes() {
        let result = merge_attributes(vec![
            vec![attr("class", "a"), attr("id", "x")],
            vec![attr("class", "b"), attr("id", "y")],
        ]);
        assert_eq!(result.len(), 2);
        // Should be sorted by name
        assert_eq!(result[0].name, "class");
        assert_eq!(result[1].name, "id");
        // class merged, id overwritten
        assert_eq!(get_value(&result[0]), "a b");
        assert_eq!(get_value(&result[1]), "y");
    }

    #[test]
    fn unsorted_input_still_works() {
        // Even if inputs aren't sorted, the function should handle it
        let result = merge_attributes(vec![
            vec![attr("z", "1"), attr("a", "2")],
            vec![attr("m", "3")],
        ]);
        assert_eq!(result.len(), 3);
        // Output should be sorted
        assert_eq!(result[0].name, "a");
        assert_eq!(result[1].name, "m");
        assert_eq!(result[2].name, "z");
    }

    #[test]
    fn volatile_flag_preserved_on_class_merge() {
        let mut a1 = attr("class", "foo");
        a1.volatile = true;
        let a2 = attr("class", "bar");

        let result = merge_attributes(vec![vec![a1], vec![a2]]);
        assert!(result[0].volatile);
    }

    #[test]
    fn style_attributes_are_merged() {
        let result = merge_attributes(vec![
            vec![attr("style", "color: red")],
            vec![attr("style", "font-size: 14px")],
        ]);
        assert_eq!(result.len(), 1);
        assert_eq!(get_value(&result[0]), "color: red; font-size: 14px");
    }

    #[test]
    fn style_merge_handles_trailing_semicolons() {
        let result = merge_attributes(vec![
            vec![attr("style", "color: red;")],
            vec![attr("style", "font-size: 14px;")],
        ]);
        assert_eq!(get_value(&result[0]), "color: red; font-size: 14px");
    }

    #[test]
    fn style_merge_handles_empty() {
        let result = merge_attributes(vec![
            vec![attr("style", "")],
            vec![attr("style", "color: red")],
        ]);
        assert_eq!(get_value(&result[0]), "color: red");
    }

    #[test]
    fn style_merge_trims_whitespace() {
        let result = merge_attributes(vec![
            vec![attr("style", "  color: red;  ")],
            vec![attr("style", "  font-size: 14px  ")],
        ]);
        assert_eq!(get_value(&result[0]), "color: red; font-size: 14px");
    }

    #[test]
    fn style_merge_three_lists() {
        let result = merge_attributes(vec![
            vec![attr("style", "color: red")],
            vec![attr("style", "font-size: 14px")],
            vec![attr("style", "margin: 0")],
        ]);
        assert_eq!(
            get_value(&result[0]),
            "color: red; font-size: 14px; margin: 0"
        );
    }

    #[test]
    fn mixed_class_style_and_other() {
        let result = merge_attributes(vec![
            vec![
                attr("class", "a"),
                attr("id", "x"),
                attr("style", "color: red"),
            ],
            vec![
                attr("class", "b"),
                attr("id", "y"),
                attr("style", "font-size: 14px"),
            ],
        ]);
        assert_eq!(result.len(), 3);
        assert_eq!(get_value(&result[0]), "a b"); // class merged
        assert_eq!(get_value(&result[1]), "y"); // id overwritten
        assert_eq!(get_value(&result[2]), "color: red; font-size: 14px"); // style merged
    }
}
