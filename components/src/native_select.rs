//! Styled native select matching shadcn/ui.
//!
//! Pure HTML `<select>` with Tailwind styling — no Radix primitive needed.

use dioxus::prelude::*;
use dx_icons_lucide::IconChevronDown;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// NativeSelectSize
// ---------------------------------------------------------------------------

/// Size variant for the native select.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum NativeSelectSize {
    #[default]
    Default,
    Sm,
}

// ---------------------------------------------------------------------------
// NativeSelect
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct NativeSelectProps {
    /// Size variant.
    #[props(default)]
    pub size: NativeSelectSize,

    /// Change event handler.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,

    /// Additional Tailwind classes for the wrapper.
    #[props(default)]
    pub class: Option<String>,

    /// Additional Tailwind classes for the select element.
    #[props(default)]
    pub select_class: Option<String>,

    /// Attributes to extend the select element.
    #[props(extends = select, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn NativeSelect(props: NativeSelectProps) -> Element {
    let wrapper_class = tw_merge!(
        "group/native-select relative w-fit has-[select:disabled]:opacity-50",
        props.class,
    );

    let size_class = match props.size {
        NativeSelectSize::Default => "",
        NativeSelectSize::Sm => "data-[size=sm]:h-8 data-[size=sm]:py-1",
    };

    let select_class = tw_merge!(
        "h-9 w-full min-w-0 appearance-none rounded-md border border-input bg-transparent px-3 py-2 pr-9 text-sm shadow-xs transition-[color,box-shadow] outline-none selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:bg-input/30 dark:hover:bg-input/50 dark:aria-invalid:ring-destructive/40",
        size_class,
        props.select_class,
    );

    let size_attr = match props.size {
        NativeSelectSize::Default => None,
        NativeSelectSize::Sm => Some("sm"),
    };

    rsx! {
        div {
            "data-slot": "native-select-wrapper",
            class: wrapper_class,
            select {
                "data-slot": "native-select",
                "data-size": size_attr,
                class: select_class,
                onchange: move |e| if let Some(handler) = &props.onchange { handler.call(e) },
                ..props.attributes,
                {props.children}
            }
            IconChevronDown {
                class: "pointer-events-none absolute top-1/2 right-3.5 size-4 -translate-y-1/2 text-muted-foreground opacity-50 select-none",
            }
        }
    }
}

// ---------------------------------------------------------------------------
// NativeSelectOption
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct NativeSelectOptionProps {
    #[props(extends = option, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn NativeSelectOption(props: NativeSelectOptionProps) -> Element {
    rsx! {
        option {
            "data-slot": "native-select-option",
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// NativeSelectOptGroup
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct NativeSelectOptGroupProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = optgroup, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn NativeSelectOptGroup(props: NativeSelectOptGroupProps) -> Element {
    rsx! {
        optgroup {
            "data-slot": "native-select-optgroup",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}
