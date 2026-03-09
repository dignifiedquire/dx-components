//! Styled input matching shadcn/ui.
//!
//! Pure HTML + Tailwind component.
//! No primitive dependency — renders a native `<input>`.

use dioxus::prelude::*;
use tailwind_fuse::*;

/// The props for the styled [`Input`] component.
#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// Input event handler.
    #[props(default)]
    pub oninput: Option<EventHandler<FormEvent>>,

    /// Change event handler.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,

    /// Focus event handler.
    #[props(default)]
    pub onfocus: Option<EventHandler<FocusEvent>>,

    /// Blur event handler.
    #[props(default)]
    pub onblur: Option<EventHandler<FocusEvent>>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the input element.
    #[props(extends = input, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled Input — matches shadcn exactly.
#[component]
pub fn Input(props: InputProps) -> Element {
    let class = tw_merge!(
        "h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none selection:bg-primary selection:text-primary-foreground file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm dark:bg-input/30",
        "focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50",
        "aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40",
        props.class,
    );

    rsx! {
        input {
            "data-slot": "input",
            class: class,
            oninput: move |e| if let Some(handler) = &props.oninput { handler.call(e) },
            onchange: move |e| if let Some(handler) = &props.onchange { handler.call(e) },
            onfocus: move |e| if let Some(handler) = &props.onfocus { handler.call(e) },
            onblur: move |e| if let Some(handler) = &props.onblur { handler.call(e) },
            ..props.attributes,
        }
    }
}
