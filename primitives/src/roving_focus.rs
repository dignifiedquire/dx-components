//! Roving focus group — matches `@radix-ui/react-roving-focus`.
//!
//! Manages keyboard focus within a group of items using the roving tabindex
//! technique: only the currently active item has `tabindex="0"`, all others
//! have `tabindex="-1"`. Arrow keys move focus between items.
//!
//! Replaces the legacy `FocusState` with a Radix-aligned API that supports
//! orientation, direction, looping, and integrates with the collection system.

use crate::collection::{use_collection_item, CollectionContext, CollectionItem};
use crate::direction::{Direction, Orientation};
use crate::{use_controlled, use_unique_id};
use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Collection item data
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
struct RovingItemData {
    id: String,
    focusable: bool,
    active: bool,
}

type RovingCollection = CollectionContext<RovingItemData>;
type RovingCollectionItem = CollectionItem<RovingItemData>;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct RovingCtx {
    orientation: ReadSignal<Option<Orientation>>,
    dir: ReadSignal<Direction>,
    r#loop: ReadSignal<bool>,
    current_tab_stop_id: Memo<Option<String>>,
    on_item_focus: Callback<String>,
    on_item_shift_tab: Callback<()>,
    collection: RovingCollection,
}

// ---------------------------------------------------------------------------
// RovingFocusGroup (root)
// ---------------------------------------------------------------------------

/// Props for [`RovingFocusGroup`].
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

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// Manages roving tabindex focus within a group of items.
///
/// Matches Radix's `RovingFocusGroup`. Only the active item has `tabindex="0"`;
/// arrow keys move focus between items based on orientation and direction.
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
    let (current_tab_stop_id, set_current_tab_stop_id) = use_controlled(
        props.current_tab_stop_id,
        props.default_current_tab_stop_id,
        props.on_current_tab_stop_id_change,
    );

    let mut is_tabbing_back_out = use_signal(|| false);
    let focusable_items_count = use_signal(|| 0usize);

    let collection = use_context_provider(RovingCollection::new);

    let on_item_focus = use_callback(move |id: String| {
        set_current_tab_stop_id.call(Some(id));
    });

    let on_item_shift_tab = use_callback(move |_: ()| {
        is_tabbing_back_out.set(true);
    });

    let ctx = use_context_provider(|| RovingCtx {
        orientation: props.orientation,
        dir: props.dir,
        r#loop: props.r#loop,
        current_tab_stop_id,
        on_item_focus,
        on_item_shift_tab,
        collection,
    });

    // Provide focusable item count tracking via context
    use_context_provider(|| RovingFocusableCount {
        count: focusable_items_count,
    });

    let tab_index = if is_tabbing_back_out() || focusable_items_count() == 0 {
        "-1"
    } else {
        "0"
    };

    let orientation = (props.orientation)();

    rsx! {
        div {
            "data-slot": "roving-focus-group",
            "data-orientation": orientation.map(|o| o.as_str()),
            tabindex: tab_index,
            style: "outline: none;",
            class: props.class,

            onfocus: move |_event: FocusEvent| {
                // Only handle direct focus on the group (not bubbled from items)
                // When keyboard-focused (not tabbing back out), focus the active item
                if !is_tabbing_back_out() {
                    let items = ctx.collection.get_items();
                    let focusable: Vec<&RovingCollectionItem> = items
                        .iter()
                        .filter(|i| i.data.focusable)
                        .collect();

                    // Priority: active item > current tab stop > first
                    let target = focusable
                        .iter()
                        .find(|i| i.data.active)
                        .or_else(|| {
                            let current_id = current_tab_stop_id.read();
                            current_id.as_ref().and_then(|id| {
                                focusable.iter().find(|i| i.data.id == *id)
                            })
                        })
                        .or_else(|| focusable.first())
                        .copied();

                    if let Some(item) = target {
                        if let Some(md) = (item.mounted)() {
                            spawn(async move {
                                let _ = md.set_focus(true).await;
                            });
                        }
                    }
                }
            },

            onblur: move |_| {
                is_tabbing_back_out.set(false);
            },

            ..props.attributes,

            {props.children}
        }
    }
}

// Tracking focusable item count
#[derive(Clone, Copy)]
struct RovingFocusableCount {
    count: Signal<usize>,
}

// ---------------------------------------------------------------------------
// RovingFocusGroupItem
// ---------------------------------------------------------------------------

/// Props for [`RovingFocusGroupItem`].
#[allow(missing_docs)]
#[derive(Props, Clone, PartialEq)]
pub struct RovingFocusGroupItemProps {
    /// Custom tab stop ID. Auto-generated if not provided.
    #[props(default)]
    pub tab_stop_id: Option<String>,

    /// Whether this item is focusable. Defaults to `true`.
    #[props(default = true)]
    pub focusable: bool,

    /// Whether this item is the "active" item (e.g., selected).
    #[props(default)]
    pub active: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// An item within a [`RovingFocusGroup`].
///
/// Renders a `<span>` with the appropriate `tabindex` based on whether it is
/// the current tab stop. Handles arrow key navigation between items.
#[component]
pub fn RovingFocusGroupItem(props: RovingFocusGroupItemProps) -> Element {
    let auto_id = use_unique_id();
    let id = props
        .tab_stop_id
        .clone()
        .unwrap_or_else(|| auto_id.cloned());

    let ctx: RovingCtx = use_context();
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

    // Register with collection
    let mut mounted_ref = use_collection_item(RovingItemData {
        id: id.clone(),
        focusable: props.focusable,
        active: props.active,
    });

    // Track focusable items count
    let mut focusable_count: RovingFocusableCount = use_context();
    let focusable = props.focusable;
    use_effect(move || {
        if focusable {
            *focusable_count.count.write() += 1;
        }
    });
    use_drop(move || {
        if focusable {
            *focusable_count.count.write() -= 1;
        }
    });

    let tab_index = if is_current_tab_stop() { "0" } else { "-1" };
    let orientation = (ctx.orientation)();
    let item_id = id.clone();
    let focus_id = id.clone();

    rsx! {
        span {
            "data-slot": "roving-focus-group-item",
            "data-orientation": orientation.map(|o| o.as_str()),
            tabindex: tab_index,
            class: props.class,

            onmounted: move |event: Event<MountedData>| {
                mounted_ref.set(Some(event.data()));
            },

            onmousedown: move |event: MouseEvent| {
                if !focusable {
                    event.prevent_default();
                } else {
                    ctx.on_item_focus.call(item_id.clone());
                }
            },

            onfocus: {
                let focus_id = focus_id.clone();
                move |_| {
                    ctx.on_item_focus.call(focus_id.clone());
                }
            },

            onkeydown: {
                let item_id = id.clone();
                move |event: KeyboardEvent| {
                    if event.key() == Key::Tab && event.modifiers().shift() {
                        ctx.on_item_shift_tab.call(());
                        return;
                    }

                    let focus_intent = get_focus_intent(
                        &event,
                        (ctx.orientation)(),
                        (ctx.dir)(),
                    );

                    if let Some(intent) = focus_intent {
                        if event.modifiers().meta()
                            || event.modifiers().ctrl()
                            || event.modifiers().alt()
                            || event.modifiers().shift()
                        {
                            return;
                        }
                        event.prevent_default();

                        let items = ctx.collection.get_items();
                        let focusable_items: Vec<&RovingCollectionItem> = items
                            .iter()
                            .filter(|i| i.data.focusable)
                            .collect();

                        let candidates = match intent {
                            FocusIntent::First => focusable_items,
                            FocusIntent::Last => {
                                let mut v = focusable_items;
                                v.reverse();
                                v
                            }
                            FocusIntent::Next | FocusIntent::Prev => {
                                let current_idx = focusable_items
                                    .iter()
                                    .position(|i| i.data.id == item_id);

                                let is_prev = matches!(intent, FocusIntent::Prev);
                                let do_loop = (ctx.r#loop)();

                                if let Some(idx) = current_idx {
                                    if is_prev {
                                        if idx == 0 {
                                            if do_loop {
                                                vec![*focusable_items.last().unwrap()]
                                            } else {
                                                vec![]
                                            }
                                        } else {
                                            vec![focusable_items[idx - 1]]
                                        }
                                    } else if idx + 1 >= focusable_items.len() {
                                        if do_loop {
                                            vec![focusable_items[0]]
                                        } else {
                                            vec![]
                                        }
                                    } else {
                                        vec![focusable_items[idx + 1]]
                                    }
                                } else {
                                    focusable_items
                                }
                            }
                        };

                        // Focus the first candidate
                        for candidate in candidates {
                            if let Some(md) = (candidate.mounted)() {
                                spawn(async move {
                                    let _ = md.set_focus(true).await;
                                });
                                break;
                            }
                        }
                    }
                }
            },

            ..props.attributes,

            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// Focus intent helpers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
enum FocusIntent {
    First,
    Last,
    Prev,
    Next,
}

fn get_focus_intent(
    event: &KeyboardEvent,
    orientation: Option<Orientation>,
    dir: Direction,
) -> Option<FocusIntent> {
    let key = direction_aware_key(event.key(), dir);

    match orientation {
        Some(Orientation::Vertical) if matches!(key, Key::ArrowLeft | Key::ArrowRight) => {
            return None;
        }
        Some(Orientation::Horizontal) if matches!(key, Key::ArrowUp | Key::ArrowDown) => {
            return None;
        }
        _ => {}
    }

    match key {
        Key::ArrowLeft | Key::ArrowUp => Some(FocusIntent::Prev),
        Key::ArrowRight | Key::ArrowDown => Some(FocusIntent::Next),
        Key::Home => Some(FocusIntent::First),
        Key::End => Some(FocusIntent::Last),
        _ => None,
    }
}

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
