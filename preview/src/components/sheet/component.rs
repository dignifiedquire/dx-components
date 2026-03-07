use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::dialog::{self, DialogCtx};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

impl SheetSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            SheetSide::Top => "top",
            SheetSide::Right => "right",
            SheetSide::Bottom => "bottom",
            SheetSide::Left => "left",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SheetProps {
    /// The controlled `open` state.
    pub open: ReadSignal<Option<bool>>,
    /// The default `open` state when uncontrolled.
    #[props(default)]
    pub default_open: bool,
    /// Callback when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,
    /// The children.
    pub children: Element,
}

#[component]
pub fn Sheet(props: SheetProps) -> Element {
    rsx! {
        dialog::DialogRoot {
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            {props.children}
        }
    }
}

#[component]
pub fn SheetContent(
    #[props(default = ReadSignal::new(Signal::new(None)))] id: ReadSignal<Option<String>>,
    #[props(default)] side: SheetSide,
    #[props(default)] class: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let class = class
        .map(|c| format!("sheet {c}"))
        .unwrap_or("sheet".to_string());

    rsx! {
        dialog::DialogContent {
            class,
            id,
            "data-slot": "sheet-content",
            "data-side": side.as_str(),
            attributes,
            {children}
            SheetClose { class: "sheet-close",
                svg {
                    class: "sheet-close-icon",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M18 6 6 18" }
                    path { d: "m6 6 12 12" }
                }
            }
        }
    }
}

#[component]
pub fn SheetHeader(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "sheet-header", "data-slot": "sheet-header", ..attributes, {children} }
    }
}

#[component]
pub fn SheetFooter(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "sheet-footer", "data-slot": "sheet-footer", ..attributes, {children} }
    }
}

#[component]
pub fn SheetTitle(
    #[props(default = ReadSignal::new(Signal::new(None)))] id: ReadSignal<Option<String>>,
    #[props(default)] class: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        dialog::DialogTitle {
            id,
            class: "sheet-title",
            "data-slot": "sheet-title",
            attributes,
            {children}
        }
    }
}

#[component]
pub fn SheetDescription(
    #[props(default = ReadSignal::new(Signal::new(None)))] id: ReadSignal<Option<String>>,
    #[props(default)] class: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        dialog::DialogDescription {
            id,
            class: "sheet-description",
            "data-slot": "sheet-description",
            attributes,
            {children}
        }
    }
}

#[component]
pub fn SheetClose(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    children: Element,
) -> Element {
    let ctx: DialogCtx = use_context();

    let mut merged: Vec<Attribute> = attributes! {
        button {
            onclick: move |_| {
                ctx.set_open(false);
            }
        }
    };
    merged.extend(attributes);

    if let Some(dynamic) = r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            button { ..merged, {children} }
        }
    }
}
