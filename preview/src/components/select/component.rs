use dioxus::prelude::*;
use dioxus_primitives::select::{
    self, SelectContentProps, SelectGroupProps, SelectItemIndicatorProps, SelectItemProps,
    SelectLabelProps, SelectProps, SelectSeparatorProps, SelectTriggerProps, SelectValueProps,
};

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        select::Select::<T> {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            name: props.name,
            placeholder: props.placeholder,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            {props.children}
        }
    }
}

#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
    rsx! {
        select::SelectTrigger { class: "select-trigger", attributes: props.attributes,
            {props.children}
            svg {
                class: "select-expand-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
    rsx! {
        select::SelectValue { attributes: props.attributes }
    }
}

#[component]
pub fn SelectContent(props: SelectContentProps) -> Element {
    rsx! {
        select::SelectContent {
            class: "select-list",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// Backward-compatible alias.
#[component]
pub fn SelectList(props: SelectContentProps) -> Element {
    SelectContent(props)
}

#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    rsx! {
        select::SelectGroup {
            class: "select-group",
            disabled: props.disabled,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectLabel(props: SelectLabelProps) -> Element {
    rsx! {
        select::SelectLabel {
            class: "select-group-label",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// Backward-compatible alias.
#[component]
pub fn SelectGroupLabel(props: SelectLabelProps) -> Element {
    SelectLabel(props)
}

#[component]
pub fn SelectItem<T: Clone + PartialEq + 'static>(props: SelectItemProps<T>) -> Element {
    rsx! {
        select::SelectItem::<T> {
            class: "select-option",
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// Backward-compatible alias.
#[component]
pub fn SelectOption<T: Clone + PartialEq + 'static>(props: SelectItemProps<T>) -> Element {
    SelectItem(props)
}

#[component]
pub fn SelectItemIndicator(props: SelectItemIndicatorProps) -> Element {
    rsx! {
        select::SelectItemIndicator {
            attributes: props.attributes,
            svg {
                class: "select-check-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M5 13l4 4L19 7" }
            }
        }
    }
}

#[component]
pub fn SelectSeparator(props: SelectSeparatorProps) -> Element {
    rsx! {
        select::SelectSeparator { attributes: props.attributes }
    }
}
