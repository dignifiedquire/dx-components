//! Core primitive utilities — matches `@radix-ui/primitive`.
//!
//! Provides [`compose_event_handlers`] for composing two event handlers
//! where the original fires first, then the component's handler fires
//! (unless the event was marked as handled).
//!
//! Also provides DOM utility functions (`can_use_dom`, `get_owner_document`,
//! `get_owner_window`, `get_active_element`, `is_frame`) used internally
//! by other primitives.

use std::cell::Cell;
use std::rc::Rc;

/// Returns `true` when DOM APIs are available.
///
/// Matches Radix's `canUseDOM`. In a Dioxus/wasm context this is always
/// `true` at runtime because the code executes in the browser. On the
/// server (SSR) it returns `false`.
pub fn can_use_dom() -> bool {
    cfg!(target_arch = "wasm32")
}

/// Composes two event handlers so both fire in sequence.
///
/// Matches Radix's `composeEventHandlers(original, ours, { checkForDefaultPrevented })`:
/// 1. Calls `original` first (the user/consumer handler).
/// 2. If `check_for_default_prevented` is true (default) and the original handler
///    signalled prevention, skips `ours`.
/// 3. Otherwise calls `ours` (the component's internal handler).
///
/// # Signalling prevention
///
/// Dioxus events don't expose a `defaultPrevented` boolean that can be read after
/// calling a handler. Instead, the returned closure provides a `prevented` flag
/// via [`Rc<Cell<bool>>`] that the original handler can set to `true` to stop
/// the component's handler from firing.
///
/// For most use cases (where the component doesn't need to check prevention),
/// use [`compose_callbacks`] instead.
#[allow(clippy::type_complexity)]
pub fn compose_event_handlers<E: 'static>(
    original: Option<Rc<dyn Fn(&E, Rc<Cell<bool>>)>>,
    ours: Option<Rc<dyn Fn(&E)>>,
    check_for_default_prevented: bool,
) -> impl Fn(E) {
    move |event: E| {
        let prevented = Rc::new(Cell::new(false));

        if let Some(ref original) = original {
            original(&event, prevented.clone());
        }

        if !check_for_default_prevented || !prevented.get() {
            if let Some(ref ours) = ours {
                ours(&event);
            }
        }
    }
}

/// Simplified event handler composition — both handlers always fire.
///
/// Calls `first` then `second`. This is the common case where neither handler
/// needs to prevent the other.
///
/// ```
/// use dioxus_primitives::primitive::compose_callbacks;
///
/// let count = std::cell::Cell::new(0);
/// let handler = compose_callbacks(
///     Some(|_: &i32| {}),
///     Some(|_: &i32| {}),
/// );
/// handler(42);
/// ```
pub fn compose_callbacks<E: 'static>(
    first: Option<impl Fn(&E) + 'static>,
    second: Option<impl Fn(&E) + 'static>,
) -> impl Fn(E) {
    move |event: E| {
        if let Some(ref f) = first {
            f(&event);
        }
        if let Some(ref s) = second {
            s(&event);
        }
    }
}

// ---------------------------------------------------------------------------
// DOM utilities (wasm32 only)
// ---------------------------------------------------------------------------

/// Returns the `ownerDocument` of the given DOM node, falling back to the
/// global `document`.
///
/// Matches Radix's `getOwnerDocument`. Returns `None` if DOM APIs are
/// unavailable.
#[cfg(target_arch = "wasm32")]
pub fn get_owner_document(node: Option<&web_sys::Node>) -> Option<web_sys::Document> {
    if !can_use_dom() {
        return None;
    }
    Some(
        node.and_then(|n| n.owner_document())
            .or_else(|| web_sys::window()?.document())?,
    )
}

/// Returns the `defaultView` (window) for the given DOM node, falling back
/// to the global `window`.
///
/// Matches Radix's `getOwnerWindow`. Returns `None` if DOM APIs are
/// unavailable.
#[cfg(target_arch = "wasm32")]
pub fn get_owner_window(node: Option<&web_sys::Node>) -> Option<web_sys::Window> {
    if !can_use_dom() {
        return None;
    }
    get_owner_document(node)?
        .default_view()
        .or_else(web_sys::window)
}

/// Returns the currently active element, with support for iframes and
/// `aria-activedescendant`.
///
/// Matches Radix's `getActiveElement`, which was adapted from
/// [Ariakit](https://github.com/ariakit/ariakit) (MIT).
///
/// When `active_descendant` is `true` and the active element has an
/// `aria-activedescendant` attribute, the element referenced by that id
/// is returned instead.
#[cfg(target_arch = "wasm32")]
pub fn get_active_element(
    node: Option<&web_sys::Node>,
    active_descendant: bool,
) -> Option<web_sys::HtmlElement> {
    use wasm_bindgen::JsCast;

    let doc = get_owner_document(node)?;
    let active = doc.active_element()?;

    // `activeElement` can be an empty object when interacting with elements
    // inside an iframe.
    if active.node_name().is_empty() {
        return None;
    }

    // If the active element is an iframe, recurse into its content document.
    if is_frame(&active) {
        let iframe: &web_sys::HtmlIFrameElement = active.unchecked_ref();
        if let Some(content_doc) = iframe.content_document() {
            if let Some(body) = content_doc.body() {
                let body_node: &web_sys::Node = body.as_ref();
                return get_active_element(Some(body_node), active_descendant);
            }
        }
    }

    if active_descendant {
        if let Some(id) = active
            .get_attribute("aria-activedescendant")
            .filter(|id| !id.is_empty())
        {
            if let Some(owner) = get_owner_document(Some(active.as_ref())) {
                if let Some(el) = owner.get_element_by_id(&id) {
                    return el.dyn_into::<web_sys::HtmlElement>().ok();
                }
            }
        }
    }

    active.dyn_into::<web_sys::HtmlElement>().ok()
}

/// Returns `true` if the element is an `<iframe>`.
///
/// Matches Radix's `isFrame`.
#[cfg(target_arch = "wasm32")]
pub fn is_frame(element: &web_sys::Element) -> bool {
    element.tag_name() == "IFRAME"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    type OriginalHandler = Rc<dyn Fn(&(), Rc<Cell<bool>>)>;

    #[test]
    fn compose_callbacks_both_fire() {
        let a_count = Rc::new(Cell::new(0));
        let b_count = Rc::new(Cell::new(0));
        let a = a_count.clone();
        let b = b_count.clone();

        let handler = compose_callbacks(
            Some(move |_: &()| {
                a.set(a.get() + 1);
            }),
            Some(move |_: &()| {
                b.set(b.get() + 1);
            }),
        );

        handler(());
        assert_eq!(a_count.get(), 1);
        assert_eq!(b_count.get(), 1);
    }

    #[test]
    fn compose_callbacks_first_none() {
        let count = Rc::new(Cell::new(0));
        let c = count.clone();

        let handler = compose_callbacks(
            None::<fn(&())>,
            Some(move |_: &()| {
                c.set(c.get() + 1);
            }),
        );

        handler(());
        assert_eq!(count.get(), 1);
    }

    #[test]
    fn compose_callbacks_second_none() {
        let count = Rc::new(Cell::new(0));
        let c = count.clone();

        let handler = compose_callbacks(
            Some(move |_: &()| {
                c.set(c.get() + 1);
            }),
            None::<fn(&())>,
        );

        handler(());
        assert_eq!(count.get(), 1);
    }

    #[test]
    fn compose_event_handlers_original_fires_first() {
        let order = Rc::new(RefCell::new(Vec::new()));
        let o1 = order.clone();
        let o2 = order.clone();

        let original: OriginalHandler = Rc::new(move |_, _| {
            o1.borrow_mut().push("original");
        });
        let ours: Rc<dyn Fn(&())> = Rc::new(move |_| {
            o2.borrow_mut().push("ours");
        });

        let handler = compose_event_handlers(Some(original), Some(ours), true);
        handler(());

        assert_eq!(*order.borrow(), vec!["original", "ours"]);
    }

    #[test]
    fn compose_event_handlers_prevented_stops_ours() {
        let ours_called = Rc::new(Cell::new(false));
        let oc = ours_called.clone();

        let original: OriginalHandler = Rc::new(|_, prevented| {
            prevented.set(true);
        });
        let ours: Rc<dyn Fn(&())> = Rc::new(move |_| {
            oc.set(true);
        });

        let handler = compose_event_handlers(Some(original), Some(ours), true);
        handler(());

        assert!(!ours_called.get());
    }

    #[test]
    fn compose_event_handlers_check_disabled_ignores_prevention() {
        let ours_called = Rc::new(Cell::new(false));
        let oc = ours_called.clone();

        let original: OriginalHandler = Rc::new(|_, prevented| {
            prevented.set(true);
        });
        let ours: Rc<dyn Fn(&())> = Rc::new(move |_| {
            oc.set(true);
        });

        let handler = compose_event_handlers(Some(original), Some(ours), false);
        handler(());

        assert!(ours_called.get());
    }

    use std::cell::RefCell;
}
