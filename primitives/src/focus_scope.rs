//! Focus scope — matches `@radix-ui/react-focus-scope`.
//!
//! Provides [`FocusScope`], a container that manages focus looping (Tab wraps
//! around at edges) and optional focus trapping (focus cannot leave the scope).
//!
//! When `trapped` is true:
//! - `focusin`/`focusout` document listeners redirect escaping focus back inside
//! - A MutationObserver catches focused-element removal
//! - On mount, auto-focuses the first tabbable element
//! - On unmount, restores focus to the previously focused element
//!
//! A global focus scope stack supports nested scopes: when a new scope activates,
//! the previous scope is paused. When a scope deactivates, the next scope resumes.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Focus scope stack — matches upstream's `createFocusScopesStack`
// ---------------------------------------------------------------------------

struct FocusScopeState {
    paused: Cell<bool>,
}

impl FocusScopeState {
    fn new() -> Self {
        Self {
            paused: Cell::new(false),
        }
    }

    fn pause(&self) {
        self.paused.set(true);
    }

    fn resume(&self) {
        self.paused.set(false);
    }

    fn is_paused(&self) -> bool {
        self.paused.get()
    }
}

thread_local! {
    static FOCUS_SCOPES_STACK: RefCell<Vec<Rc<FocusScopeState>>> = const { RefCell::new(Vec::new()) };
}

fn stack_add(scope: &Rc<FocusScopeState>) {
    FOCUS_SCOPES_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        if let Some(active) = stack.first() {
            if !Rc::ptr_eq(active, scope) {
                active.pause();
            }
        }
        stack.retain(|s| !Rc::ptr_eq(s, scope));
        stack.insert(0, scope.clone());
    });
}

fn stack_remove(scope: &Rc<FocusScopeState>) {
    FOCUS_SCOPES_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        stack.retain(|s| !Rc::ptr_eq(s, scope));
        if let Some(active) = stack.first() {
            active.resume();
        }
    });
}

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// AutoFocusEvent — replaces upstream's cancelable AUTOFOCUS_ON_* CustomEvents
// ---------------------------------------------------------------------------

/// Which auto-focus moment produced an [`AutoFocusEvent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoFocusPhase {
    /// The scope has mounted and is about to focus its first tabbable child.
    /// Upstream: `focusScope.autoFocusOnMount`.
    Mount,
    /// The scope is unmounting and is about to restore focus to the element
    /// that had it before. Upstream: `focusScope.autoFocusOnUnmount`.
    Unmount,
}

/// A preventable auto-focus event.
///
/// Upstream dispatches cancelable `CustomEvent`s and skips its default focus
/// behaviour when `event.defaultPrevented` is set — this is how
/// `DialogContent`, `PopoverContent` and friends implement `onOpenAutoFocus`
/// and `onCloseAutoFocus`. Call
/// [`prevent_default()`](AutoFocusEvent::prevent_default) to take focus
/// management into your own hands.
#[derive(Clone)]
pub struct AutoFocusEvent {
    /// Shared on clone: composing handlers must observe each other's
    /// `prevent_default()`, exactly as multiple DOM listeners on one event do.
    prevented: Rc<Cell<bool>>,
    phase: AutoFocusPhase,
}

impl AutoFocusEvent {
    fn new(phase: AutoFocusPhase) -> Self {
        Self {
            prevented: Rc::new(Cell::new(false)),
            phase,
        }
    }

    /// Skip the scope's default focus behaviour for this moment.
    pub fn prevent_default(&self) {
        self.prevented.set(true);
    }

    /// Whether [`prevent_default()`](AutoFocusEvent::prevent_default) was called.
    pub fn is_default_prevented(&self) -> bool {
        self.prevented.get()
    }

    /// Whether this is the mount or the unmount moment.
    pub fn phase(&self) -> AutoFocusPhase {
        self.phase
    }
}

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

    /// Called before the scope auto-focuses its first tabbable child on mount.
    /// Call [`AutoFocusEvent::prevent_default`] to keep focus where it is.
    /// Upstream: `onMountAutoFocus`.
    #[props(default)]
    pub on_mount_auto_focus: Callback<AutoFocusEvent>,

    /// Called before the scope restores focus on unmount. Call
    /// [`AutoFocusEvent::prevent_default`] to keep focus where it is.
    /// Upstream: `onUnmountAutoFocus`.
    #[props(default)]
    pub on_unmount_auto_focus: Callback<AutoFocusEvent>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children within the focus scope.
    pub children: Element,
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// Options for [`use_focus_scope`] — the prop set of [`FocusScope`] minus the
/// rendering concerns.
#[derive(Clone, Copy, Default)]
pub struct FocusScopeOptions {
    /// Tab from the last item focuses the first, Shift+Tab from the first
    /// focuses the last. Upstream: `loop` (default `false`).
    pub r#loop: bool,
    /// Focus cannot leave the scope. Upstream: `trapped` (default `false`).
    pub trapped: bool,
    /// Called before the scope focuses its first tabbable child on mount.
    pub on_mount_auto_focus: Callback<AutoFocusEvent>,
    /// Called before the scope restores focus on unmount.
    pub on_unmount_auto_focus: Callback<AutoFocusEvent>,
}

/// What [`use_focus_scope`] needs the caller to put on the scope element.
#[derive(Clone, Copy)]
pub struct FocusScopeHandle {
    /// Attach to the scope element's `onkeydown` — this is the Tab handling.
    pub on_keydown: Callback<KeyboardEvent>,
}

/// The behaviour of [`FocusScope`] without its `<div>`.
///
/// Upstream renders `FocusScope` with `asChild`, collapsing it onto the element
/// it wraps. Menus depend on that: the scope container, the dismissable layer
/// node, the roving-focus container and the element carrying `role="menu"` all
/// have to be the *same* element, and the auto-focus target is that element
/// too. A nested `<div>` splits them, which is how the port ended up rendering
/// `role="menu"` twice with a focus-scope div in between.
///
/// So overlays that need the scope to *be* an element they already render call
/// this hook and spread the handle onto it, exactly as with
/// [`crate::dismissable_layer::use_dismissable_layer`]. `container_id` must be
/// that element's `id`.
pub fn use_focus_scope(
    container_id: ReadSignal<String>,
    opts: FocusScopeOptions,
) -> FocusScopeHandle {
    let trapped = opts.trapped;
    let looping = opts.r#loop;
    let on_mount_auto_focus = opts.on_mount_auto_focus;
    let on_unmount_auto_focus = opts.on_unmount_auto_focus;

    let scope_state = use_hook(|| Rc::new(FocusScopeState::new()));

    // --- Focus trapping: document listeners + MutationObserver ---
    // Matches upstream's first useEffect [trapped, container, focusScope.paused]
    {
        let scope = scope_state.clone();
        crate::use_effect_with_cleanup(move || {
            if !trapped {
                return Box::new(|| {}) as Box<dyn FnOnce()>;
            }

            #[cfg(target_arch = "wasm32")]
            {
                let cleanup = wasm_impl::setup_trap(container_id, scope.clone());
                Box::new(cleanup) as Box<dyn FnOnce()>
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = &scope;
                Box::new(|| {}) as Box<dyn FnOnce()>
            }
        });
    }

    // --- Mount auto-focus + focus scope stack + unmount restore ---
    // Matches upstream's second useEffect [container, onMountAutoFocus, ...]
    {
        let scope = scope_state.clone();
        crate::use_effect_with_cleanup(move || {
            stack_add(&scope);

            // Upstream only dispatches the mount event when it would actually
            // move focus — i.e. when focus is not already inside the container
            // (`hasFocusedCandidate`) — and skips `focusFirst` if the handler
            // prevented it.
            #[cfg(target_arch = "wasm32")]
            let previously_focused = {
                let previously_focused = wasm_impl::active_element();
                if !wasm_impl::contains_focus(container_id, previously_focused.as_ref()) {
                    let event = AutoFocusEvent::new(AutoFocusPhase::Mount);
                    let prevented = event.prevented.clone();
                    on_mount_auto_focus.call(event);
                    if !prevented.get() {
                        wasm_impl::focus_first_candidate(container_id);
                    }
                }
                previously_focused
            };

            #[cfg(not(target_arch = "wasm32"))]
            on_mount_auto_focus.call(AutoFocusEvent::new(AutoFocusPhase::Mount));

            let scope_cleanup = scope.clone();
            Box::new(move || {
                let event = AutoFocusEvent::new(AutoFocusPhase::Unmount);
                let prevented = event.prevented.clone();
                on_unmount_auto_focus.call(event);

                #[cfg(target_arch = "wasm32")]
                if !prevented.get() {
                    wasm_impl::restore_focus(previously_focused);
                }
                #[cfg(not(target_arch = "wasm32"))]
                let _ = prevented;

                stack_remove(&scope_cleanup);
            }) as Box<dyn FnOnce()>
        });
    }

    // --- Tab key handling ---
    // Matches upstream's handleKeyDown callback
    let scope_for_keydown = scope_state.clone();
    let handle_keydown = move |event: KeyboardEvent| {
        if !looping && !trapped {
            return;
        }
        if scope_for_keydown.is_paused() {
            return;
        }

        // Upstream: isTabKey = event.key === 'Tab' && !event.altKey && !event.ctrlKey && !event.metaKey
        if !matches!(event.key(), Key::Tab) {
            return;
        }
        let modifiers = event.modifiers();
        if !(modifiers.alt() || modifiers.ctrl() || modifiers.meta()) {
            #[cfg(target_arch = "wasm32")]
            wasm_impl::handle_tab(container_id, looping, modifiers.shift(), &event);
        }
    };

    // Off wasm every use of the container id sits behind a `cfg`. Discarding it
    // explicitly is the convention this crate already uses (`use_top_layer`),
    // and keeps the parameter name meaningful in the public signature.
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = container_id;
    }

    FocusScopeHandle {
        on_keydown: use_callback(handle_keydown),
    }
}

/// A container that manages focus boundaries.
///
/// Matches Radix's `FocusScope` component.
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
///
/// Renders a `<div>` wrapper; see [`use_focus_scope`] when the scope must
/// be an element you already render.
#[component]
pub fn FocusScope(props: FocusScopeProps) -> Element {
    let container_id = crate::use_unique_id();

    let scope = use_focus_scope(
        container_id.into(),
        FocusScopeOptions {
            r#loop: props.r#loop,
            trapped: props.trapped,
            on_mount_auto_focus: props.on_mount_auto_focus,
            on_unmount_auto_focus: props.on_unmount_auto_focus,
        },
    );

    rsx! {
        div {
            id: "{container_id}",
            tabindex: "-1",
            style: "outline: none;",
            onkeydown: move |e: KeyboardEvent| scope.on_keydown.call(e),
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// Wasm-only implementation details
// ---------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
mod wasm_impl {
    use super::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    /// Focusable element selector — covers common interactive elements.
    /// Upstream uses TreeWalker with runtime tabIndex checks; this selector
    /// approximates the same set. Visibility filtering is applied separately.
    const FOCUSABLE_SELECTOR: &str = concat!(
        "a[href]:not([disabled]):not([tabindex=\"-1\"]),",
        "button:not([disabled]):not([tabindex=\"-1\"]),",
        "input:not([disabled]):not([type=\"hidden\"]):not([tabindex=\"-1\"]),",
        "select:not([disabled]):not([tabindex=\"-1\"]),",
        "textarea:not([disabled]):not([tabindex=\"-1\"]),",
        "[tabindex]:not([disabled]):not([tabindex=\"-1\"])"
    );

    /// Set up document-level focus trap listeners and MutationObserver.
    /// Returns a cleanup closure that removes everything.
    pub(super) fn setup_trap(
        container_id: ReadSignal<String>,
        scope: Rc<FocusScopeState>,
    ) -> impl FnOnce() {
        let doc = match web_sys::window().and_then(|w| w.document()) {
            Some(d) => d,
            None => return Box::new(|| {}) as Box<dyn FnOnce()>,
        };

        let id = container_id.peek().clone();
        let last_focused: Rc<Cell<Option<web_sys::HtmlElement>>> = Rc::new(Cell::new(None));

        // --- focusin handler ---
        let focusin_closure = {
            let id = id.clone();
            let scope = scope.clone();
            let last = last_focused.clone();
            Closure::wrap(Box::new(move |event: web_sys::FocusEvent| {
                if scope.is_paused() {
                    return;
                }
                let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
                    return;
                };
                let Some(container) = doc.get_element_by_id(&id) else {
                    return;
                };
                let target: Option<web_sys::Node> = event.target().and_then(|t| t.dyn_into().ok());
                if let Some(ref target) = target {
                    if container.contains(Some(target)) {
                        last.set(target.clone().dyn_into::<web_sys::HtmlElement>().ok());
                    } else {
                        // Focus escaped — bring it back
                        if let Some(el) = last.take() {
                            focus_element(&el);
                            last.set(Some(el));
                        }
                    }
                }
            }) as Box<dyn FnMut(web_sys::FocusEvent)>)
        };

        // --- focusout handler ---
        let focusout_closure = {
            let id = id.clone();
            let scope = scope.clone();
            let last = last_focused.clone();
            Closure::wrap(Box::new(move |event: web_sys::FocusEvent| {
                if scope.is_paused() {
                    return;
                }
                // null relatedTarget = browser/tab switch or element removed — don't interfere
                let Some(related) = event.related_target() else {
                    return;
                };
                let related_node: Option<&web_sys::Node> = related.dyn_ref::<web_sys::Node>();
                let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
                    return;
                };
                let Some(container) = doc.get_element_by_id(&id) else {
                    return;
                };
                if !container.contains(related_node) {
                    if let Some(el) = last.take() {
                        focus_element(&el);
                        last.set(Some(el));
                    }
                }
            }) as Box<dyn FnMut(web_sys::FocusEvent)>)
        };

        // --- MutationObserver for removed elements ---
        let mutation_closure = {
            let id = id.clone();
            Closure::wrap(Box::new(move |mutations: js_sys::Array, _: JsValue| {
                let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
                    return;
                };
                // Only act if focus fell to body (element was removed)
                let is_body = doc
                    .active_element()
                    .map_or(true, |e| e.tag_name() == "BODY");
                if !is_body {
                    return;
                }
                for i in 0..mutations.length() {
                    let record: web_sys::MutationRecord = mutations.get(i).unchecked_into();
                    if record.removed_nodes().length() > 0 {
                        if let Some(container) = doc.get_element_by_id(&id) {
                            if let Some(el) = container.dyn_ref::<web_sys::HtmlElement>() {
                                let _ = el.focus();
                            }
                        }
                        return;
                    }
                }
            }) as Box<dyn FnMut(js_sys::Array, JsValue)>)
        };

        let observer =
            web_sys::MutationObserver::new(mutation_closure.as_ref().unchecked_ref()).ok();
        if let Some(ref obs) = observer {
            if let Some(container) = doc.get_element_by_id(&id) {
                let mut init = web_sys::MutationObserverInit::new();
                init.set_child_list(true);
                init.set_subtree(true);
                let _ = obs.observe_with_options(container.as_ref(), &init);
            }
        }

        // Register listeners
        let target: &web_sys::EventTarget = doc.as_ref();
        let _ = target
            .add_event_listener_with_callback("focusin", focusin_closure.as_ref().unchecked_ref());
        let _ = target.add_event_listener_with_callback(
            "focusout",
            focusout_closure.as_ref().unchecked_ref(),
        );

        // Cleanup
        let doc_cleanup = doc.clone();
        Box::new(move || {
            let target: &web_sys::EventTarget = doc_cleanup.as_ref();
            let _ = target.remove_event_listener_with_callback(
                "focusin",
                focusin_closure.as_ref().unchecked_ref(),
            );
            let _ = target.remove_event_listener_with_callback(
                "focusout",
                focusout_closure.as_ref().unchecked_ref(),
            );
            if let Some(obs) = observer {
                obs.disconnect();
            }
            drop(mutation_closure);
        }) as Box<dyn FnOnce()>
    }

    /// The currently focused element, captured before the scope takes over.
    /// Upstream: `const previouslyFocusedElement = document.activeElement`.
    pub(super) fn active_element() -> Option<web_sys::HtmlElement> {
        web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.active_element())
            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
    }

    /// Whether `element` already sits inside the scope container.
    /// Upstream: `const hasFocusedCandidate = container.contains(previouslyFocusedElement)`.
    pub(super) fn contains_focus(
        container_id: ReadSignal<String>,
        element: Option<&web_sys::HtmlElement>,
    ) -> bool {
        let Some(element) = element else {
            return false;
        };
        let id = container_id.peek().clone();
        web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(&id))
            .map(|container| container.contains(Some(element.as_ref())))
            .unwrap_or(false)
    }

    /// Focus the scope's first tabbable child, falling back to the container.
    /// Upstream: `focusFirst(removeLinks(getTabbableCandidates(container)), { select: true })`.
    pub(super) fn focus_first_candidate(container_id: ReadSignal<String>) -> Option<()> {
        let doc = web_sys::window().and_then(|w| w.document())?;
        let id = container_id.peek().clone();
        let container = doc.get_element_by_id(&id)?;

        // Focus first tabbable candidate (excluding links, matching upstream's removeLinks)
        let candidates = get_tabbable_candidates(&container);
        let non_links: Vec<_> = candidates
            .into_iter()
            .filter(|el| el.tag_name() != "A")
            .collect();
        let moved = focus_first(&non_links);

        // If focus didn't move, focus the container itself
        if !moved {
            if let Some(el) = container.dyn_ref::<web_sys::HtmlElement>() {
                let _ = el.focus();
            }
        }

        Some(())
    }

    /// Restore focus to the previously focused element on unmount.
    pub(super) fn restore_focus(previously_focused: Option<web_sys::HtmlElement>) {
        if let Some(el) = previously_focused {
            focus_element(&el);
        } else if let Some(body) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
        {
            let _ = body.focus();
        }
    }

    /// Handle Tab/Shift+Tab by always managing focus programmatically.
    ///
    /// Upstream only handles edge cases (first/last), relying on the browser's
    /// native Tab order for intermediate elements. However, WebKit/Safari on
    /// macOS does not include `<button>` in the native Tab order by default,
    /// causing focus to skip buttons or escape the scope. We always intercept
    /// Tab and manually move focus to ensure consistent behavior across browsers.
    pub(super) fn handle_tab(
        container_id: ReadSignal<String>,
        looping: bool,
        shift: bool,
        event: &KeyboardEvent,
    ) {
        let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
            return;
        };
        let id = container_id.peek().clone();
        let Some(container) = doc.get_element_by_id(&id) else {
            return;
        };

        let focused = doc
            .active_element()
            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok());
        let Some(focused) = focused else { return };

        let candidates = get_visible_tabbable_candidates(&container);
        if candidates.is_empty() {
            event.prevent_default();
            return;
        }

        let focused_el: &web_sys::Element = focused.as_ref();
        let current_idx = candidates
            .iter()
            .position(|el| el.as_ref() as &web_sys::Element == focused_el);

        event.prevent_default();

        match current_idx {
            Some(idx) => {
                if shift {
                    if idx > 0 {
                        focus_element(&candidates[idx - 1]);
                    } else if looping {
                        focus_element(candidates.last().unwrap());
                    }
                } else if idx < candidates.len() - 1 {
                    focus_element(&candidates[idx + 1]);
                } else if looping {
                    focus_element(&candidates[0]);
                }
            }
            None => {
                // Focus is on the container or a non-tabbable element —
                // move to the first (Tab) or last (Shift+Tab) candidate.
                let target = if shift {
                    candidates.last()
                } else {
                    candidates.first()
                };
                if let Some(el) = target {
                    focus_element(el);
                }
            }
        }
    }

    /// Get tabbable candidates inside a container (unfiltered by visibility).
    fn get_tabbable_candidates(container: &web_sys::Element) -> Vec<web_sys::HtmlElement> {
        let Ok(nodes) = container.query_selector_all(FOCUSABLE_SELECTOR) else {
            return Vec::new();
        };
        let mut result = Vec::new();
        for i in 0..nodes.length() {
            if let Some(node) = nodes.item(i) {
                if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                    result.push(el);
                }
            }
        }
        result
    }

    /// Get visible tabbable candidates inside a container.
    fn get_visible_tabbable_candidates(container: &web_sys::Element) -> Vec<web_sys::HtmlElement> {
        get_tabbable_candidates(container)
            .into_iter()
            .filter(|el| !is_hidden(el, Some(container)))
            .collect()
    }

    /// Returns the first and last visible tabbable elements.
    /// Matches upstream's `getTabbableEdges`.
    fn get_tabbable_edges(
        container: &web_sys::Element,
    ) -> (Option<web_sys::HtmlElement>, Option<web_sys::HtmlElement>) {
        let candidates = get_visible_tabbable_candidates(container);
        (candidates.first().cloned(), candidates.last().cloned())
    }

    /// Checks if an element is hidden (display: none or visibility: hidden).
    /// Matches upstream's `isHidden`.
    fn is_hidden(node: &web_sys::HtmlElement, up_to: Option<&web_sys::Element>) -> bool {
        let Some(window) = web_sys::window() else {
            return false;
        };

        let el: &web_sys::Element = node.as_ref();
        if let Ok(Some(style)) = window.get_computed_style(el) {
            if style.get_property_value("visibility").as_deref() == Ok("hidden") {
                return true;
            }
        }

        // Walk up the DOM tree checking for display: none
        let mut current: Option<web_sys::Element> = Some(el.clone());
        while let Some(ref el) = current {
            if let Some(up_to) = up_to {
                if el == up_to {
                    return false;
                }
            }
            if let Ok(Some(style)) = window.get_computed_style(el) {
                if style.get_property_value("display").as_deref() == Ok("none") {
                    return true;
                }
            }
            current = el.parent_element();
        }

        false
    }

    /// Attempts to focus the first element in a list. Returns true if focus moved.
    /// Matches upstream's `focusFirst`.
    fn focus_first(candidates: &[web_sys::HtmlElement]) -> bool {
        let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
            return false;
        };
        let previously_focused = doc.active_element();
        for candidate in candidates {
            focus_element(candidate);
            if doc.active_element().as_ref() != previously_focused.as_ref() {
                return true;
            }
        }
        false
    }

    /// Focus an element. Matches upstream's `focus` utility.
    fn focus_element(element: &web_sys::HtmlElement) {
        let _ = element.focus();
    }
}
