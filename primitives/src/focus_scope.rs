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

    /// Called when auto-focusing on mount.
    #[props(default)]
    pub on_mount_auto_focus: Callback<()>,

    /// Called when auto-focusing on unmount.
    #[props(default)]
    pub on_unmount_auto_focus: Callback<()>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children within the focus scope.
    pub children: Element,
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

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
#[component]
pub fn FocusScope(props: FocusScopeProps) -> Element {
    let trapped = props.trapped;
    let looping = props.r#loop;
    let on_mount_auto_focus = props.on_mount_auto_focus;
    let on_unmount_auto_focus = props.on_unmount_auto_focus;

    let container_id = crate::use_unique_id();
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
            on_mount_auto_focus.call(());

            #[cfg(target_arch = "wasm32")]
            let previously_focused = wasm_impl::mount_auto_focus(container_id);

            let scope_cleanup = scope.clone();
            Box::new(move || {
                on_unmount_auto_focus.call(());

                #[cfg(target_arch = "wasm32")]
                wasm_impl::restore_focus(previously_focused);

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

    rsx! {
        div {
            id: "{container_id}",
            tabindex: "-1",
            style: "outline: none;",
            onkeydown: handle_keydown,
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
        container_id: Signal<String>,
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
                init.child_list(true);
                init.subtree(true);
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

    /// Auto-focus first tabbable element on mount.
    /// Returns the previously focused element for restoration on unmount.
    pub(super) fn mount_auto_focus(container_id: Signal<String>) -> Option<web_sys::HtmlElement> {
        let doc = web_sys::window().and_then(|w| w.document())?;
        let previously_focused = doc
            .active_element()
            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok());

        let id = container_id.peek().clone();
        let container = doc.get_element_by_id(&id)?;

        // Check if focus is already inside the container
        if let Some(ref pf) = previously_focused {
            let pf_node: &web_sys::Node = pf.as_ref();
            if container.contains(Some(pf_node)) {
                return previously_focused;
            }
        }

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

        previously_focused
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

    /// Handle Tab/Shift+Tab key to loop focus at edges.
    /// Matches upstream's handleKeyDown.
    pub(super) fn handle_tab(
        container_id: Signal<String>,
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

        let (first, last) = get_tabbable_edges(&container);

        match (first, last) {
            (Some(first), Some(last)) => {
                let focused_el: &web_sys::Element = focused.as_ref();
                let first_el: &web_sys::Element = first.as_ref();
                let last_el: &web_sys::Element = last.as_ref();

                if !shift && focused_el == last_el {
                    event.prevent_default();
                    if looping {
                        focus_element(&first);
                    }
                } else if shift && focused_el == first_el {
                    event.prevent_default();
                    if looping {
                        focus_element(&last);
                    }
                }
            }
            _ => {
                // No tabbable elements — prevent Tab if focus is on the container
                let container_el: &web_sys::Element = container.as_ref();
                let focused_el: &web_sys::Element = focused.as_ref();
                if focused_el == container_el {
                    event.prevent_default();
                }
            }
        }
    }

    /// Get tabbable candidates inside a container.
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

    /// Returns the first and last visible tabbable elements.
    /// Matches upstream's `getTabbableEdges`.
    fn get_tabbable_edges(
        container: &web_sys::Element,
    ) -> (Option<web_sys::HtmlElement>, Option<web_sys::HtmlElement>) {
        let candidates = get_tabbable_candidates(container);
        let first = candidates
            .iter()
            .find(|el| !is_hidden(el, Some(container)))
            .cloned();
        let last = candidates
            .iter()
            .rev()
            .find(|el| !is_hidden(el, Some(container)))
            .cloned();
        (first, last)
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
