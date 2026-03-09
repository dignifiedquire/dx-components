//! Styled textarea matching shadcn/ui.
//!
//! Pure HTML + Tailwind component.
//! No primitive dependency — renders a native `<textarea>`.

use dioxus::prelude::*;
use tailwind_fuse::*;

/// The props for the styled [`Textarea`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TextareaProps {
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

    /// Attributes to extend the textarea element.
    #[props(extends = textarea, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled Textarea — matches shadcn exactly.
#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let class = tw_merge!(
        "flex field-sizing-content min-h-16 w-full rounded-md border border-input bg-transparent px-3 py-2 text-base shadow-xs transition-[color,box-shadow] outline-none placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 md:text-sm dark:bg-input/30 dark:aria-invalid:ring-destructive/40",
        props.class,
    );

    rsx! {
        textarea {
            "data-slot": "textarea",
            class: class,
            oninput: move |e| if let Some(handler) = &props.oninput { handler.call(e) },
            onchange: move |e| if let Some(handler) = &props.onchange { handler.call(e) },
            onfocus: move |e| if let Some(handler) = &props.onfocus { handler.call(e) },
            onblur: move |e| if let Some(handler) = &props.onblur { handler.call(e) },
            ..props.attributes,
        }
    }
}
