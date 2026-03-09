//! Styled select matching shadcn/ui.
//!
//! Wraps `dioxus_primitives::select` with shadcn Tailwind classes.

use dioxus::prelude::*;
use dioxus_core::AttributeValue::Text;
use dioxus_primitives::select as primitives;
pub use dioxus_primitives::select::{Select, SelectGroup, SelectProps, SelectValue};
use dx_icons_lucide::{IconCheck, IconChevronDown};
use tailwind_fuse::*;

/// Push a `class` attribute onto an attribute vec.
fn push_class(attrs: &mut Vec<Attribute>, class: String) {
    attrs.push(Attribute {
        name: "class",
        value: Text(class),
        namespace: None,
        volatile: false,
    });
}

// ---------------------------------------------------------------------------
// SelectTrigger
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SelectTriggerProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
    let class = tw_merge!(
        "flex w-fit items-center justify-between gap-2 rounded-md border border-input bg-transparent px-3 py-2 text-sm whitespace-nowrap shadow-xs transition-[color,box-shadow] outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 data-[placeholder]:text-muted-foreground data-[size=default]:h-9 data-[size=sm]:h-8 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center *:data-[slot=select-value]:gap-2 dark:bg-input/30 dark:hover:bg-input/50 dark:aria-invalid:ring-destructive/40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-muted-foreground",
        props.class,
    );

    let mut attrs = props.attributes;
    push_class(&mut attrs, class);

    rsx! {
        primitives::SelectTrigger {
            attributes: attrs,
            {props.children}
            IconChevronDown { class: "size-4 opacity-50" }
        }
    }
}

// ---------------------------------------------------------------------------
// SelectContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SelectContentProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn SelectContent(props: SelectContentProps) -> Element {
    let class = tw_merge!(
        "relative z-50 max-h-(--radix-select-content-available-height) min-w-[8rem] origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto rounded-md border bg-popover text-popover-foreground shadow-md data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
        props.class,
    );

    let mut attrs = props.attributes;
    push_class(&mut attrs, class);

    rsx! {
        primitives::SelectContent {
            attributes: attrs,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// SelectItem
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SelectItemProps<T: Clone + PartialEq + 'static> {
    pub value: ReadSignal<T>,

    #[props(default)]
    pub text_value: ReadSignal<Option<String>>,

    #[props(default)]
    pub disabled: bool,

    pub index: ReadSignal<usize>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn SelectItem<T: Clone + PartialEq + 'static>(props: SelectItemProps<T>) -> Element {
    let class = tw_merge!(
        "relative flex w-full cursor-default items-center gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-muted-foreground *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2",
        props.class,
    );

    let mut attrs = props.attributes;
    push_class(&mut attrs, class);

    rsx! {
        primitives::SelectItem::<T> {
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            index: props.index,
            attributes: attrs,

            primitives::SelectItemIndicator {
                class: "absolute right-2 flex size-3.5 items-center justify-center",
                IconCheck { class: "size-4" }
            }
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// SelectLabel
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SelectLabelProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn SelectLabel(props: SelectLabelProps) -> Element {
    let class = tw_merge!("px-2 py-1.5 text-xs text-muted-foreground", props.class);

    let mut attrs = props.attributes;
    push_class(&mut attrs, class);

    rsx! {
        primitives::SelectLabel {
            attributes: attrs,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// SelectSeparator
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct SelectSeparatorProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SelectSeparator(props: SelectSeparatorProps) -> Element {
    let class = tw_merge!("pointer-events-none -mx-1 my-1 h-px bg-border", props.class,);

    let mut attrs = props.attributes;
    push_class(&mut attrs, class);

    rsx! {
        primitives::SelectSeparator {
            attributes: attrs,
        }
    }
}
