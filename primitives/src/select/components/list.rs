//! SelectContent (formerly SelectList) component implementation.

use crate::popper::{Align, PopperContent, PopperContentCtx, Side};
use crate::portal::Portal;
use crate::{
    select::context::SelectListContext, use_animated_open, use_effect, use_id_or, use_unique_id,
};
use dioxus::prelude::*;

use super::super::context::SelectContext;

/// The props for the [`SelectContent`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectContentProps {
    /// The ID of the content for ARIA attributes
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Side of the trigger to place content. Defaults to `Bottom`.
    #[props(default)]
    pub side: Side,

    /// Offset from the trigger edge in pixels. Defaults to 0.
    #[props(default)]
    pub side_offset: f64,

    /// Alignment relative to the trigger. Defaults to `Center`.
    #[props(default)]
    pub align: Align,

    /// Offset along the alignment axis. Defaults to 0.
    #[props(default)]
    pub align_offset: f64,

    /// Whether to avoid viewport edge collisions. Defaults to `true`.
    #[props(default = true)]
    pub avoid_collisions: bool,

    /// Collision padding in pixels. Defaults to 0.
    #[props(default)]
    pub collision_padding: f64,

    /// Additional attributes for the content
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the content
    pub children: Element,
}

/// Backward-compatible alias.
pub type SelectListProps = SelectContentProps;

/// The dropdown content container for the Select component that contains the
/// [`SelectItem`](super::option::SelectItem)s. Only rendered when the select is open.
#[component]
pub fn SelectContent(props: SelectContentProps) -> Element {
    let mut ctx = use_context::<SelectContext>();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);
    use_effect(move || {
        ctx.list_id.set(Some(id()));
    });

    let open = ctx.open;

    let render = use_animated_open(id, open);
    let render = use_memo(render);

    use_context_provider(|| SelectListContext {
        render: render.into(),
    });

    use_effect(move || {
        if render() {
            ctx.focus_state.set_focus(ctx.initial_focus.cloned());
        } else {
            ctx.initial_focus.set(None);
        }
    });

    rsx! {
        if render() {
            Portal {
                PopperContent {
                    side: props.side,
                    side_offset: props.side_offset,
                    align: props.align,
                    align_offset: props.align_offset,
                    avoid_collisions: props.avoid_collisions,
                    collision_padding: props.collision_padding,
                    css_var_prefix: "select",

                    SelectContentInner {
                        id,
                        attributes: props.attributes,
                        children: props.children,
                    }
                }
            }
        } else {
            {props.children}
        }
    }
}

/// Inner component that reads [`PopperContentCtx`] for `data-side`/`data-align`
/// and renders the listbox div with keyboard/focus handling.
#[derive(Props, Clone, PartialEq)]
struct SelectContentInnerProps {
    id: Memo<String>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
fn SelectContentInner(props: SelectContentInnerProps) -> Element {
    let mut ctx = use_context::<SelectContext>();
    let popper = use_context::<PopperContentCtx>();
    let side = (popper.placed_side)();
    let align = (popper.placed_align)();

    let mut open = ctx.open;
    let mut listbox_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let focused = move || open() && !ctx.focus_state.any_focused();

    use_effect(move || {
        let Some(listbox_ref) = listbox_ref() else {
            return;
        };
        if focused() {
            spawn(async move {
                _ = listbox_ref.set_focus(true);
            });
        }
    });

    let onkeydown = move |event: KeyboardEvent| {
        let key = event.key();
        let code = event.code();

        // Learn from keyboard events for adaptive matching
        if let Key::Character(actual_char) = &key {
            if let Some(actual_char) = actual_char.chars().next() {
                ctx.learn_from_keyboard_event(&code.to_string(), actual_char);
            }
        }

        let mut arrow_key_navigation = |event: KeyboardEvent| {
            // Clear the typeahead buffer
            ctx.typeahead_buffer.take();
            event.prevent_default();
            event.stop_propagation();
        };

        match key {
            Key::Character(new_text) => {
                if new_text == " " {
                    ctx.select_current_item();
                    event.prevent_default();
                    event.stop_propagation();
                    return;
                }

                ctx.add_to_typeahead_buffer(&new_text);
            }
            Key::ArrowUp => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_prev();
            }
            Key::End => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_last();
            }
            Key::ArrowDown => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_next();
            }
            Key::Home => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_first();
            }
            Key::Enter => {
                ctx.select_current_item();
                open.set(false);
                event.prevent_default();
                event.stop_propagation();
            }
            Key::Escape => {
                open.set(false);
                event.prevent_default();
                event.stop_propagation();
            }
            _ => {}
        }
    };

    let active_descendant = use_memo(move || {
        let focus_idx = ctx.focus_state.current_focus()?;
        let options = ctx.options.read();
        options
            .iter()
            .find(|opt| opt.tab_index == focus_idx)
            .map(|opt| opt.id.clone())
    });

    rsx! {
        div {
            id: props.id,
            role: "listbox",
            "data-slot": "select-content",
            "data-side": side.as_str(),
            "data-align": align.as_str(),
            tabindex: if focused() { "0" } else { "-1" },
            "data-state": if open() { "open" } else { "closed" },
            aria_activedescendant: active_descendant,

            onmounted: move |evt| listbox_ref.set(Some(evt.data())),
            onkeydown,
            onblur: move |_| {
                if focused() {
                    open.set(false);
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}

/// Backward-compatible alias for [`SelectContent`].
#[component]
pub fn SelectList(props: SelectContentProps) -> Element {
    SelectContent(props)
}
