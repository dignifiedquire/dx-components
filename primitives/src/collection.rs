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

/// A single item in a collection. Holds mounting data and user-defined item data.
#[derive(Clone)]
pub struct CollectionItem<D: Clone + 'static> {
    id: usize,
    /// Signal tracking the element's mounted state for programmatic focus.
    pub mounted: Signal<Option<Rc<MountedData>>>,
    /// User-defined data associated with this item (e.g. disabled state).
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

impl<D: Clone + 'static> Default for CollectionContext<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: Clone + 'static> CollectionContext<D> {
    /// Creates a new empty collection context.
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
        let mut items = self.items.write();
        // Update in-place if an item with this ID already exists, preserving
        // render order. Without this, unregister+register would move the item
        // to the end of the Vec, breaking keyboard navigation order.
        if let Some(existing) = items.iter_mut().find(|i| i.id == item.id) {
            existing.data = item.data;
            existing.mounted = item.mounted;
        } else {
            items.push(item);
        }
    }

    fn unregister(&mut self, id: usize) {
        self.items.write().retain(|i| i.id != id);
    }
}

/// Register a collection item — equivalent to Radix's `Collection.ItemSlot`.
///
/// Returns a `Signal<Option<Rc<MountedData>>>` that must be wired to the
/// element's `onmounted` handler for programmatic focus support.
///
/// Upstream re-runs `useEffect` when `itemData` values change
/// (`[...Object.values(itemData)]`). We store data in a signal so the effect
/// re-runs when data changes, keeping the collection in sync.
pub fn use_collection_item<D: Clone + 'static>(data: D) -> Signal<Option<Rc<MountedData>>> {
    let mut ctx: CollectionContext<D> = use_context();
    // Create at ROOT scope — the parent collection iterates all items and
    // accesses their mounted signals, which would cross scope boundaries
    // if created in the item's own scope.
    let mounted = use_hook(|| Signal::new_in_scope(None, ScopeId::ROOT));
    let item_id = use_hook(|| NEXT_ITEM_ID.fetch_add(1, Ordering::Relaxed));
    let mut data_sig: Signal<D> = use_signal(|| data.clone());

    // Always update the data signal so the effect below re-runs when
    // item data changes (e.g. disabled toggling).
    data_sig.set(data);

    use_effect(move || {
        let data = data_sig.read().clone();
        // Unregister any previous entry (no-op on first run) then re-register
        // with the latest data. Matches upstream's effect re-running on
        // itemData changes.
        ctx.unregister(item_id);
        ctx.register(CollectionItem {
            id: item_id,
            mounted,
            data,
        });
    });

    use_effect_cleanup(move || {
        ctx.unregister(item_id);
        mounted.manually_drop();
    });

    mounted
}
