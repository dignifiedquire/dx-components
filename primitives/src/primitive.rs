//! Core primitive utilities — matches `@radix-ui/primitive`.
//!
//! Provides [`compose_event_handlers`] for composing two event handlers
//! where the original fires first, then the component's handler fires
//! (unless the event was marked as handled).

use std::cell::Cell;
use std::rc::Rc;

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
