//! Roving focus group — matches `@radix-ui/react-roving-focus`.
//!
//! Manages keyboard focus within a group of items using the roving tabindex
//! technique: only the currently active item has `tabindex="0"`, all others
//! have `tabindex="-1"`. Arrow keys move focus between items.
//!
//! Replaces the legacy `FocusState` with a Radix-aligned API that supports
//! orientation, direction, looping, and integrates with the collection system.

use std::cell::Cell;
use std::rc::Rc;

use crate::collection::{use_collection_item, CollectionContext, CollectionItem};
use crate::direction::{Direction, Orientation};
use crate::slot::render_slot;
use crate::{merge_attributes, use_controlled, use_unique_id};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

// ---------------------------------------------------------------------------
// Constants — matches upstream
// ---------------------------------------------------------------------------

// Upstream: `const ENTRY_FOCUS = 'rovingFocusGroup.onEntryFocus';`
// Upstream: `const EVENT_OPTIONS = { bubbles: false, cancelable: true };`
// These constants exist in upstream for CustomEvent dispatch on the DOM node.
// In Dioxus we use `RovingFocusEntryEvent` with `Rc<Cell<bool>>` instead.

// ---------------------------------------------------------------------------
// Collection item data
// ---------------------------------------------------------------------------

// Upstream: `type ItemData = { id: string; focusable: boolean; active: boolean };`
#[derive(Clone, PartialEq)]
struct RovingItemData {
    id: String,
    focusable: bool,
    active: bool,
}

type RovingCollection = CollectionContext<RovingItemData>;
type RovingCollectionItem = CollectionItem<RovingItemData>;

// ---------------------------------------------------------------------------
// Cancelable entry focus event
// ---------------------------------------------------------------------------

/// Cancelable event passed to `on_entry_focus`.
///
/// Matches upstream's pattern where `onEntryFocus` receives a DOM `Event`
/// and can call `preventDefault()` to skip the default entry focus behavior.
/// Uses `Rc<Cell<bool>>` so the caller retains access to the prevented flag
/// after passing a clone to the callback.
#[derive(Clone)]
pub struct RovingFocusEntryEvent {
    prevented: Rc<Cell<bool>>,
}

impl RovingFocusEntryEvent {
    fn new() -> Self {
        Self {
            prevented: Rc::new(Cell::new(false)),
        }
    }

    /// Prevent the default entry focus behavior (focusing the first candidate).
    pub fn prevent_default(&self) {
        self.prevented.set(true);
    }

    /// Check if `prevent_default()` was called.
    pub fn is_default_prevented(&self) -> bool {
        self.prevented.get()
    }
}

// ---------------------------------------------------------------------------
// Context — matches upstream RovingContextValue
// ---------------------------------------------------------------------------

/// Upstream: `type RovingContextValue = RovingFocusGroupOptions & { ... }`
#[derive(Clone, Copy)]
struct RovingCtx {
    orientation: ReadSignal<Option<Orientation>>,
    dir: ReadSignal<Direction>,
    r#loop: ReadSignal<bool>,
    current_tab_stop_id: Memo<Option<String>>,
    on_item_focus: Callback<String>,
    on_item_shift_tab: Callback<()>,
    on_focusable_item_add: Callback<()>,
    on_focusable_item_remove: Callback<()>,
    collection: RovingCollection,
}

// ---------------------------------------------------------------------------
// RovingFocusGroup (root)
// ---------------------------------------------------------------------------

/// Props for [`RovingFocusGroup`].
///
/// Upstream: `RovingFocusGroupImplProps` (the real impl props).
/// `RovingFocusGroupProps` is just a re-export alias in upstream.
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct RovingFocusGroupProps {
    /// The orientation of the group. Controls which arrow keys navigate.
    /// `None` means both axes navigate.
    #[props(default)]
    pub orientation: ReadSignal<Option<Orientation>>,

    /// Text direction for arrow key mapping. Defaults to LTR.
    #[props(default)]
    pub dir: ReadSignal<Direction>,

    /// Whether keyboard navigation loops around. Defaults to `false`.
    #[props(default)]
    pub r#loop: ReadSignal<bool>,

    /// Controlled current tab stop ID.
    #[props(default)]
    pub current_tab_stop_id: ReadSignal<Option<Option<String>>>,

    /// Default current tab stop ID for uncontrolled usage.
    #[props(default)]
    pub default_current_tab_stop_id: Option<String>,

    /// Callback when the current tab stop changes.
    #[props(default)]
    pub on_current_tab_stop_id_change: Callback<Option<String>>,

    /// Callback fired when focus enters the group via keyboard.
    /// Upstream: `onEntryFocus?: (event: Event) => void`.
    /// Call `event.prevent_default()` to skip the default entry focus behavior.
    #[props(default)]
    pub on_entry_focus: Callback<RovingFocusEntryEvent>,

    /// Whether to prevent scroll when focusing on entry.
    /// Upstream: `preventScrollOnEntryFocus` (default false).
    #[props(default)]
    pub prevent_scroll_on_entry_focus: bool,

    /// Render as a custom element (Radix `asChild` equivalent).
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    #[props(default)]
    pub children: Element,
}

/// Manages roving tabindex focus within a group of items.
///
/// Matches Radix's `RovingFocusGroup`. Upstream splits this into
/// `RovingFocusGroup` (Collection wrapper) and `RovingFocusGroupImpl`
/// (the actual logic). In Dioxus we combine them since our collection
/// system doesn't need a separate wrapper layer.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::roving_focus::{RovingFocusGroup, RovingFocusGroupItem};
/// rsx! {
///     RovingFocusGroup {
///         RovingFocusGroupItem { button { "One" } }
///         RovingFocusGroupItem { button { "Two" } }
///         RovingFocusGroupItem { button { "Three" } }
///     }
/// };
/// ```
#[component]
pub fn RovingFocusGroup(props: RovingFocusGroupProps) -> Element {
    // Upstream: useControllableState for currentTabStopId
    let (current_tab_stop_id, set_current_tab_stop_id) = use_controlled(
        props.current_tab_stop_id,
        props.default_current_tab_stop_id,
        props.on_current_tab_stop_id_change,
    );

    // Upstream: const [isTabbingBackOut, setIsTabbingBackOut] = React.useState(false);
    let mut is_tabbing_back_out = use_signal(|| false);

    // Upstream: const isClickFocusRef = React.useRef(false);
    let mut is_click_focus = use_signal(|| false);

    // Upstream: const [focusableItemsCount, setFocusableItemsCount] = React.useState(0);
    let mut focusable_items_count = use_signal(|| 0i32);

    // Upstream: const getItems = useCollection(__scopeRovingFocusGroup);
    let collection = use_context_provider(RovingCollection::new);

    // Upstream: onItemFocus callback in RovingFocusProvider
    let on_item_focus = use_callback(move |id: String| {
        set_current_tab_stop_id.call(Some(id));
    });

    // Upstream: onItemShiftTab callback in RovingFocusProvider
    let on_item_shift_tab = use_callback(move |_: ()| {
        is_tabbing_back_out.set(true);
    });

    // Upstream: onFocusableItemAdd / onFocusableItemRemove callbacks in RovingFocusProvider
    let on_focusable_item_add = use_callback(move |_: ()| {
        *focusable_items_count.write() += 1;
    });
    let on_focusable_item_remove = use_callback(move |_: ()| {
        *focusable_items_count.write() -= 1;
    });

    // Upstream: RovingFocusProvider with all context values
    use_context_provider(|| RovingCtx {
        orientation: props.orientation,
        dir: props.dir,
        r#loop: props.r#loop,
        current_tab_stop_id,
        on_item_focus,
        on_item_shift_tab,
        on_focusable_item_add,
        on_focusable_item_remove,
        collection,
    });

    // Upstream: tabIndex={isTabbingBackOut || focusableItemsCount === 0 ? -1 : 0}
    let tab_index = if is_tabbing_back_out() || focusable_items_count() == 0 {
        "-1"
    } else {
        "0"
    };

    let orientation = (props.orientation)();
    let prevent_scroll = props.prevent_scroll_on_entry_focus;
    let on_entry_focus = props.on_entry_focus;

    // Upstream: onMouseDown={composeEventHandlers(props.onMouseDown, () => {
    //   isClickFocusRef.current = true;
    // })}
    let handle_mousedown = move |_: MouseEvent| {
        is_click_focus.set(true);
    };

    // Upstream: onFocus={composeEventHandlers(props.onFocus, (event) => { ... })}
    //
    // Note on target check: Upstream checks `event.target === event.currentTarget`
    // because React's `onFocus` is implemented as `focusin` (which bubbles).
    // Dioxus's `onfocus` maps to native `focus` (non-bubbling), so the handler
    // only fires when the group div itself receives focus — no guard needed.
    let handle_focus = move |_event: FocusEvent| {
        // Upstream: const isKeyboardFocus = !isClickFocusRef.current;
        let is_keyboard_focus = !*is_click_focus.peek();

        if is_keyboard_focus && !is_tabbing_back_out() {
            // Upstream: dispatches cancelable CustomEvent(ENTRY_FOCUS) on the DOM node,
            // then checks defaultPrevented. We use RovingFocusEntryEvent + Rc<Cell<bool>>.
            let entry_event = RovingFocusEntryEvent::new();
            on_entry_focus.call(entry_event.clone());

            if !entry_event.is_default_prevented() {
                // Upstream: const items = getItems().filter((item) => item.focusable);
                let items = collection.get_items();
                let focusable: Vec<RovingCollectionItem> =
                    items.into_iter().filter(|i| i.data.focusable).collect();

                // Upstream: const activeItem = items.find((item) => item.active);
                let active_item = focusable.iter().find(|i| i.data.active).cloned();
                // Upstream: const currentItem = items.find((item) => item.id === currentTabStopId);
                let current_item = current_tab_stop_id
                    .read()
                    .as_ref()
                    .and_then(|cid| focusable.iter().find(|i| i.data.id == *cid).cloned());

                // Upstream: [activeItem, currentItem, ...items].filter(Boolean)
                let mut candidates: Vec<RovingCollectionItem> = Vec::new();
                if let Some(item) = active_item {
                    candidates.push(item);
                }
                if let Some(item) = current_item {
                    candidates.push(item);
                }
                candidates.extend(focusable);

                // Upstream: focusFirst(candidateNodes, preventScrollOnEntryFocus);
                spawn(async move {
                    focus_first(&candidates, prevent_scroll).await;
                });
            }
        }

        // Upstream: isClickFocusRef.current = false;
        is_click_focus.set(false);
    };

    // Upstream: onBlur={composeEventHandlers(props.onBlur, () => setIsTabbingBackOut(false))}
    let handle_blur = move |_: FocusEvent| {
        is_tabbing_back_out.set(false);
    };

    let base = attributes!(div {
        "data-slot": "roving-focus-group",
        "data-orientation": orientation.map(|o| o.as_str()),
        tabindex: tab_index,
        style: "outline: none;",
        class: props.class,
        onmousedown: handle_mousedown,
        onfocus: handle_focus,
        onblur: handle_blur,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    render_slot(props.r#as, merged, props.children, |attrs, children| {
        rsx! {
            div { ..attrs, {children} }
        }
    })
}

// ---------------------------------------------------------------------------
// RovingFocusGroupItem
// ---------------------------------------------------------------------------

/// Props passed to the `r#as` callback of [`RovingFocusGroupItem`].
///
/// Separates non-event attributes from event handlers so consumers can
/// compose their own handlers with the roving focus handlers explicitly.
/// This matches Radix's `asChild` + `Slot.mergeProps` behaviour.
#[derive(Clone)]
pub struct RovingFocusSlotProps {
    /// Non-event attributes: `tabindex`, `data-orientation`, `data-slot`, etc.
    pub attributes: Vec<Attribute>,
    /// Keyboard handler for arrow-key navigation and Shift+Tab.
    pub on_keydown: Callback<Event<KeyboardData>>,
    /// Focus handler that updates the current tab stop.
    pub on_focus: Callback<Event<FocusData>>,
    /// Mouse-down handler that updates the current tab stop.
    pub on_mousedown: Callback<Event<MouseData>>,
    /// Mount handler for registering the element with the collection.
    pub on_mounted: Callback<Event<MountedData>>,
}

/// Props for [`RovingFocusGroupItem`].
///
/// Upstream: `RovingFocusItemProps`
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct RovingFocusGroupItemProps {
    /// Custom tab stop ID. Auto-generated if not provided.
    /// Upstream: `tabStopId?: string`
    #[props(default)]
    pub tab_stop_id: Option<String>,

    /// Whether this item is focusable. Defaults to `true`.
    /// Upstream: `focusable?: boolean` (default true)
    #[props(default = true)]
    pub focusable: bool,

    /// Whether this item is the "active" item (e.g., selected).
    /// Upstream: `active?: boolean` (default false)
    #[props(default)]
    pub active: bool,

    /// Render as a custom element (Radix `asChild` equivalent).
    ///
    /// The callback receives [`RovingFocusSlotProps`] with attributes and
    /// event handlers separated so they can be composed with the consumer's
    /// own handlers.
    #[props(default)]
    pub r#as: Option<Callback<RovingFocusSlotProps, Element>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Render function receiving `(is_current_tab_stop, has_tab_stop)`.
    /// Matches upstream's `children: (props) => ReactNode` pattern.
    /// When provided, replaces `children`.
    #[props(default)]
    pub children_fn: Option<Callback<(bool, bool), Element>>,

    #[props(default)]
    pub children: Element,
}

/// An item within a [`RovingFocusGroup`].
///
/// Renders a `<span>` with the appropriate `tabindex` based on whether it is
/// the current tab stop. Handles arrow key navigation between items.
///
/// When `r#as` is provided, passes [`RovingFocusSlotProps`] to the callback
/// instead of rendering a default `<span>`, matching Radix's `asChild` pattern.
#[component]
pub fn RovingFocusGroupItem(props: RovingFocusGroupItemProps) -> Element {
    // Upstream: const autoId = useId();
    let auto_id = use_unique_id();
    // Upstream: const id = tabStopId || autoId;
    let id = props
        .tab_stop_id
        .clone()
        .unwrap_or_else(|| auto_id.cloned());

    // Upstream: const context = useRovingFocusContext(ITEM_NAME, __scopeRovingFocusGroup);
    let ctx: RovingCtx = use_context();

    // Upstream: const isCurrentTabStop = context.currentTabStopId === id;
    let is_current_tab_stop = use_memo({
        let id = id.clone();
        move || {
            ctx.current_tab_stop_id
                .read()
                .as_ref()
                .map(|cid| *cid == id)
                .unwrap_or(false)
        }
    });

    // Upstream: Collection.ItemSlot with scope/id/focusable/active
    let mut mounted_ref = use_collection_item(RovingItemData {
        id: id.clone(),
        focusable: props.focusable,
        active: props.active,
    });

    // Upstream: useEffect for focusable item tracking
    // React.useEffect(() => {
    //   if (focusable) { onFocusableItemAdd(); return () => onFocusableItemRemove(); }
    // }, [focusable, onFocusableItemAdd, onFocusableItemRemove]);
    let focusable = props.focusable;
    let on_add = ctx.on_focusable_item_add;
    let on_remove = ctx.on_focusable_item_remove;
    use_effect(move || {
        if focusable {
            on_add.call(());
        }
    });
    use_drop(move || {
        if focusable {
            on_remove.call(());
        }
    });

    // Upstream: tabIndex={isCurrentTabStop ? 0 : -1}
    let tab_index = if is_current_tab_stop() { "0" } else { "-1" };
    // Upstream: data-orientation={context.orientation}
    let orientation = (ctx.orientation)();

    // --- Event handlers ---

    let handle_mounted = use_callback(move |event: Event<MountedData>| {
        mounted_ref.set(Some(event.data()));
    });

    // Upstream: onMouseDown={composeEventHandlers(props.onMouseDown, (event) => {
    //   if (!focusable) event.preventDefault();
    //   else context.onItemFocus(id);
    // })}
    let handle_mousedown = {
        let item_id = id.clone();
        use_callback(move |event: Event<MouseData>| {
            if !focusable {
                event.prevent_default();
            } else {
                ctx.on_item_focus.call(item_id.clone());
            }
        })
    };

    // Upstream: onFocus={composeEventHandlers(props.onFocus, () => context.onItemFocus(id))}
    let handle_focus = {
        let focus_id = id.clone();
        use_callback(move |_: Event<FocusData>| {
            ctx.on_item_focus.call(focus_id.clone());
        })
    };

    // Upstream: onKeyDown={composeEventHandlers(props.onKeyDown, (event) => { ... })}
    let handle_keydown = {
        let item_id = id.clone();
        use_callback(move |event: Event<KeyboardData>| {
            // Upstream: if (event.key === 'Tab' && event.shiftKey) { ... }
            // Note: Shift+Tab check is BEFORE the target check (upstream line 260-263)
            if event.key() == Key::Tab && event.modifiers().shift() {
                ctx.on_item_shift_tab.call(());
                return;
            }

            // Upstream: if (event.target !== event.currentTarget) return;
            //
            // This prevents handling arrow keys from child elements (e.g., input).
            // Dioxus doesn't expose event.target vs event.currentTarget directly.
            // In typical roving focus usage, items don't contain focusable children,
            // so this guard rarely triggers. For the `r#as` path, consumers handle
            // their own event composition.

            // Upstream: const focusIntent = getFocusIntent(event, context.orientation, context.dir);
            let focus_intent = get_focus_intent(&event, (ctx.orientation)(), (ctx.dir)());

            if let Some(intent) = focus_intent {
                // Upstream: if (event.metaKey || event.ctrlKey || event.altKey || event.shiftKey) return;
                if event.modifiers().meta()
                    || event.modifiers().ctrl()
                    || event.modifiers().alt()
                    || event.modifiers().shift()
                {
                    return;
                }
                // Upstream: event.preventDefault();
                event.prevent_default();

                // Upstream: const items = getItems().filter((item) => item.focusable);
                let items = ctx.collection.get_items();
                let focusable_items: Vec<RovingCollectionItem> =
                    items.into_iter().filter(|i| i.data.focusable).collect();

                // Upstream: let candidateNodes = items.map((item) => item.ref.current!);
                let candidates = match intent {
                    // Upstream: (focusIntent === 'first') — candidateNodes stays as-is
                    FocusIntent::First => focusable_items,

                    // Upstream: (focusIntent === 'last') candidateNodes.reverse();
                    FocusIntent::Last => {
                        let mut v = focusable_items;
                        v.reverse();
                        v
                    }

                    // Upstream: (focusIntent === 'prev' || focusIntent === 'next')
                    FocusIntent::Prev | FocusIntent::Next => {
                        // Upstream: if (focusIntent === 'prev') candidateNodes.reverse();
                        let mut candidate_nodes = focusable_items;
                        if matches!(intent, FocusIntent::Prev) {
                            candidate_nodes.reverse();
                        }

                        // Upstream: const currentIndex = candidateNodes.indexOf(event.currentTarget);
                        let current_index =
                            candidate_nodes.iter().position(|i| i.data.id == item_id);

                        match current_index {
                            Some(idx) => {
                                if (ctx.r#loop)() {
                                    // Upstream: wrapArray(candidateNodes, currentIndex + 1)
                                    wrap_array(candidate_nodes, idx + 1)
                                } else {
                                    // Upstream: candidateNodes.slice(currentIndex + 1)
                                    candidate_nodes.split_off(idx + 1)
                                }
                            }
                            // If not found, indexOf returns -1, so startIndex=0 → full array
                            None => candidate_nodes,
                        }
                    }
                };

                // Upstream: setTimeout(() => focusFirst(candidateNodes));
                spawn(async move {
                    focus_first(&candidates, false).await;
                });
            }
        })
    };

    // --- Build non-event attributes ---
    let base = attributes!(span {
        "data-slot": "roving-focus-group-item",
        "data-orientation": orientation.map(|o| o.as_str()),
        tabindex: tab_index,
        class: props.class,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    if let Some(dynamic) = props.r#as {
        dynamic.call(RovingFocusSlotProps {
            attributes: merged,
            on_keydown: handle_keydown,
            on_focus: handle_focus,
            on_mousedown: handle_mousedown,
            on_mounted: handle_mounted,
        })
    } else {
        // Upstream: typeof children === 'function'
        //   ? children({ isCurrentTabStop, hasTabStop: currentTabStopId != null })
        //   : children
        let has_tab_stop = ctx.current_tab_stop_id.read().is_some();
        let rendered_children = match props.children_fn {
            Some(f) => f.call((is_current_tab_stop(), has_tab_stop)),
            None => props.children,
        };

        rsx! {
            span {
                onmounted: move |event| handle_mounted.call(event),
                onmousedown: move |event| handle_mousedown.call(event),
                onfocus: move |event| handle_focus.call(event),
                onkeydown: move |event| handle_keydown.call(event),
                ..merged,
                {rendered_children}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Focus intent helpers — matches upstream utility functions
// ---------------------------------------------------------------------------

/// Upstream: `type FocusIntent = 'first' | 'last' | 'prev' | 'next';`
#[derive(Debug, Clone, Copy)]
enum FocusIntent {
    First,
    Last,
    Prev,
    Next,
}

/// Upstream: `getFocusIntent(event, orientation, dir)`
///
/// Maps keyboard events to focus intents, respecting orientation and direction.
fn get_focus_intent(
    event: &KeyboardEvent,
    orientation: Option<Orientation>,
    dir: Direction,
) -> Option<FocusIntent> {
    // Upstream: const key = getDirectionAwareKey(event.key, dir);
    let key = direction_aware_key(event.key(), dir);

    // Upstream: filter by orientation
    match orientation {
        Some(Orientation::Vertical) if matches!(key, Key::ArrowLeft | Key::ArrowRight) => {
            return None;
        }
        Some(Orientation::Horizontal) if matches!(key, Key::ArrowUp | Key::ArrowDown) => {
            return None;
        }
        _ => {}
    }

    // Upstream: MAP_KEY_TO_FOCUS_INTENT
    match key {
        Key::ArrowLeft | Key::ArrowUp => Some(FocusIntent::Prev),
        Key::ArrowRight | Key::ArrowDown => Some(FocusIntent::Next),
        Key::PageUp | Key::Home => Some(FocusIntent::First),
        Key::PageDown | Key::End => Some(FocusIntent::Last),
        _ => None,
    }
}

/// Upstream: `getDirectionAwareKey(key, dir)`
///
/// Flips ArrowLeft/ArrowRight for RTL direction.
fn direction_aware_key(key: Key, dir: Direction) -> Key {
    if dir != Direction::Rtl {
        return key;
    }
    match key {
        Key::ArrowLeft => Key::ArrowRight,
        Key::ArrowRight => Key::ArrowLeft,
        _ => key,
    }
}

/// Upstream: `focusFirst(candidates, preventScroll = false)`
///
/// Iterates candidate elements and focuses the first one that accepts focus.
/// Uses `MountedData::set_focus` which maps to `element.focus()` on web.
///
/// Note: `preventScroll` is not currently supported via Dioxus's
/// `MountedData::set_focus` API. On wasm, `focus()` is called without
/// `FocusOptions`. Callers pass the flag for API compatibility.
async fn focus_first(candidates: &[RovingCollectionItem], _prevent_scroll: bool) {
    // Upstream implementation:
    // const PREVIOUSLY_FOCUSED_ELEMENT = document.activeElement;
    // for (const candidate of candidates) {
    //   if (candidate === PREVIOUSLY_FOCUSED_ELEMENT) return;
    //   candidate.focus({ preventScroll });
    //   if (document.activeElement !== PREVIOUSLY_FOCUSED_ELEMENT) return;
    // }
    for candidate in candidates {
        if let Some(md) = candidate.mounted.peek().clone() {
            let _ = md.set_focus(true).await;
            return;
        }
    }
}

/// Upstream: `wrapArray(array, startIndex)`
///
/// Wraps an array around itself at a given start index.
/// Example: `wrap_array(vec!['a', 'b', 'c', 'd'], 2) == vec!['c', 'd', 'a', 'b']`
fn wrap_array<T>(mut array: Vec<T>, start_index: usize) -> Vec<T> {
    let len = array.len();
    if len == 0 {
        return array;
    }
    array.rotate_left(start_index % len);
    array
}

// ---------------------------------------------------------------------------
// Aliases — matches upstream exports
// ---------------------------------------------------------------------------

/// Upstream: `const Root = RovingFocusGroup;`
pub use RovingFocusGroup as Root;
/// Upstream: `const Item = RovingFocusGroupItem;`
pub use RovingFocusGroupItem as Item;
