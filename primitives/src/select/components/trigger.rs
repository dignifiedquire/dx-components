//! SelectTrigger component implementation.

use dioxus::prelude::*;

use super::super::context::SelectContext;

/// The props for the [`SelectTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectTriggerProps {
    /// Additional attributes for the trigger button
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the trigger
    pub children: Element,
}

/// The trigger button for the Select component. Renders as `<button>` with `role="combobox"`.
#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
    let mut ctx = use_context::<SelectContext>();
    let mut open = ctx.open;
    let is_disabled = ctx.disabled;
    let has_value = !(ctx.value)().is_empty();

    rsx! {
        button {
            "data-slot": "select-trigger",
            r#type: "button",
            role: "combobox",
            disabled: is_disabled,
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": if is_disabled { "" } else { None::<&str> },
            "data-placeholder": if !has_value { "" } else { None::<&str> },

            aria_expanded: open(),
            aria_controls: ctx.list_id,
            aria_autocomplete: "none",

            onclick: move |_| {
                if !is_disabled {
                    open.toggle();
                }
            },
            onkeydown: move |event| {
                if is_disabled {
                    return;
                }
                match event.key() {
                    Key::ArrowUp => {
                        open.set(true);
                        ctx.initial_focus
                            .set(ctx.focus_state.item_count().checked_sub(1));
                        event.prevent_default();
                        event.stop_propagation();
                    }
                    Key::ArrowDown => {
                        open.set(true);
                        ctx.initial_focus
                            .set((ctx.focus_state.item_count() > 0).then_some(0));
                        event.prevent_default();
                        event.stop_propagation();
                    }
                    _ => {}
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}
