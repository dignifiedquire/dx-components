//! Generic collection system — matches `@radix-ui/react-collection`.
//!
//! Provides auto-discovery of items (e.g. accordion triggers) without manual
//! index props. Registration order matches render order in Dioxus, so no
//! `compareDocumentPosition` sorting is needed.

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

use crate::use_effect_cleanup;

static NEXT_ITEM_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub struct CollectionItem<D: Clone + 'static> {
    id: usize,
    pub mounted: Signal<Option<Rc<MountedData>>>,
    pub data: D,
}

/// Collection context — equivalent to Radix's `Collection.Provider` + `useCollection`.
pub struct CollectionContext<D: Clone + 'static> {
    items: Signal<Vec<CollectionItem<D>>>,
}

// Manual Clone/Copy impls — Signal<T> is Copy regardless of T,
// but derive requires D: Copy which is unnecessarily restrictive.
impl<D: Clone + 'static> Clone for CollectionContext<D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<D: Clone + 'static> Copy for CollectionContext<D> {}

impl<D: Clone + 'static> CollectionContext<D> {
    pub fn new() -> Self {
        Self {
            items: Signal::new(Vec::new()),
        }
    }

    /// Returns all registered items. Equivalent to Radix's `getItems()`.
    pub fn get_items(&self) -> Vec<CollectionItem<D>> {
        self.items.read().clone()
    }

    fn register(&mut self, item: CollectionItem<D>) {
        self.items.write().push(item);
    }

    fn unregister(&mut self, id: usize) {
        self.items.write().retain(|i| i.id != id);
    }
}

/// Register a collection item — equivalent to Radix's `Collection.ItemSlot`.
///
/// Returns a `Signal<Option<Rc<MountedData>>>` that must be wired to the
/// element's `onmounted` handler for programmatic focus support.
pub fn use_collection_item<D: Clone + 'static>(data: D) -> Signal<Option<Rc<MountedData>>> {
    let mut ctx: CollectionContext<D> = use_context();
    let mounted: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let item_id = use_hook(|| NEXT_ITEM_ID.fetch_add(1, Ordering::Relaxed));

    use_effect(move || {
        ctx.register(CollectionItem {
            id: item_id,
            mounted,
            data: data.clone(),
        });
    });

    use_effect_cleanup(move || {
        ctx.unregister(item_id);
    });

    mounted
}
