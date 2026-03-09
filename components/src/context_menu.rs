//! Styled context menu matching shadcn/ui.
//!
//! Wraps `dioxus_primitives::context_menu` with shadcn Tailwind classes.

use dioxus::prelude::*;
use dioxus_primitives::context_menu as primitives;
pub use dioxus_primitives::context_menu::{
    ContextMenuGroup, ContextMenuItemIndicator, ContextMenuPortal, ContextMenuRadioGroup,
    ContextMenuSub,
};
use dx_icons_lucide::{IconCheck, IconChevronRight, IconCircle};
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Shared class constants
// ---------------------------------------------------------------------------

const MENU_CONTENT: &str = "z-50 max-h-(--radix-context-menu-content-available-height) min-w-[8rem] origin-(--radix-context-menu-content-transform-origin) overflow-x-hidden overflow-y-auto rounded-md border bg-popover p-1 text-popover-foreground shadow-md data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95";

const MENU_ITEM: &str = "relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[inset]:pl-8 data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 data-[variant=destructive]:focus:text-destructive dark:data-[variant=destructive]:focus:bg-destructive/20 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-muted-foreground data-[variant=destructive]:*:[svg]:text-destructive!";

const MENU_CHECKBOX_ITEM: &str = "relative flex cursor-default items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";

const MENU_RADIO_ITEM: &str = "relative flex cursor-default items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";

const MENU_LABEL: &str = "px-2 py-1.5 text-sm font-medium text-foreground data-[inset]:pl-8";
const MENU_SEPARATOR: &str = "-mx-1 my-1 h-px bg-border";
const MENU_SHORTCUT: &str = "ml-auto text-xs tracking-widest text-muted-foreground";
const MENU_SUB_TRIGGER: &str = "flex cursor-default items-center rounded-sm px-2 py-1.5 text-sm outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[inset]:pl-8 data-[state=open]:bg-accent data-[state=open]:text-accent-foreground [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-muted-foreground";
const MENU_SUB_CONTENT: &str = "z-50 min-w-[8rem] origin-(--radix-context-menu-content-transform-origin) overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-lg data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95";

// ---------------------------------------------------------------------------
// ContextMenu (Root — no DOM)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuProps {
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    #[props(default)]
    pub default_open: bool,

    #[props(default)]
    pub on_open_change: Callback<bool>,

    #[props(default)]
    pub disabled: bool,

    pub children: Element,
}

#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    rsx! {
        primitives::ContextMenuRoot {
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuTrigger
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuTriggerProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ContextMenuTrigger(props: ContextMenuTriggerProps) -> Element {
    rsx! {
        primitives::ContextMenuTrigger {
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuContentProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ContextMenuContent(props: ContextMenuContentProps) -> Element {
    let class = tw_merge!(MENU_CONTENT, props.class);

    rsx! {
        primitives::ContextMenuContent {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuItem
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuItemProps {
    #[props(default)]
    pub disabled: bool,

    #[props(default)]
    pub on_select: EventHandler<()>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ContextMenuItem(props: ContextMenuItemProps) -> Element {
    let class = tw_merge!(MENU_ITEM, props.class);

    rsx! {
        primitives::ContextMenuItem {
            disabled: props.disabled,
            on_select: props.on_select,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuCheckboxItem
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuCheckboxItemProps {
    #[props(default)]
    pub checked: ReadSignal<bool>,

    #[props(default)]
    pub disabled: bool,

    #[props(default)]
    pub on_checked_change: Callback<bool>,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ContextMenuCheckboxItem(props: ContextMenuCheckboxItemProps) -> Element {
    let class = tw_merge!(MENU_CHECKBOX_ITEM, props.class);

    rsx! {
        primitives::ContextMenuCheckboxItem {
            checked: props.checked,
            disabled: props.disabled,
            on_checked_change: props.on_checked_change,
            class: class,
            attributes: props.attributes,

            primitives::ContextMenuItemIndicator {
                class: "absolute left-2 inline-flex size-3.5 items-center justify-center",
                IconCheck { class: "size-4" }
            }
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuRadioItem
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuRadioItemProps {
    pub value: String,

    #[props(default)]
    pub disabled: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ContextMenuRadioItem(props: ContextMenuRadioItemProps) -> Element {
    let class = tw_merge!(MENU_RADIO_ITEM, props.class);

    rsx! {
        primitives::ContextMenuRadioItem {
            value: props.value,
            disabled: props.disabled,
            class: class,
            attributes: props.attributes,

            primitives::ContextMenuItemIndicator {
                class: "absolute left-2 inline-flex size-3.5 items-center justify-center",
                IconCircle { class: "size-2 fill-current" }
            }
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuLabel
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuLabelProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ContextMenuLabel(props: ContextMenuLabelProps) -> Element {
    let class = tw_merge!(MENU_LABEL, props.class);

    rsx! {
        primitives::ContextMenuLabel {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuSeparator
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuSeparatorProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ContextMenuSeparator(props: ContextMenuSeparatorProps) -> Element {
    let class = tw_merge!(MENU_SEPARATOR, props.class);

    rsx! {
        primitives::ContextMenuSeparator {
            class: class,
            attributes: props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuShortcut
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuShortcutProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ContextMenuShortcut(props: ContextMenuShortcutProps) -> Element {
    let class = tw_merge!(MENU_SHORTCUT, props.class);

    rsx! {
        primitives::ContextMenuShortcut {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuSubTrigger
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuSubTriggerProps {
    #[props(default)]
    pub disabled: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ContextMenuSubTrigger(props: ContextMenuSubTriggerProps) -> Element {
    let class = tw_merge!(MENU_SUB_TRIGGER, props.class);

    rsx! {
        primitives::ContextMenuSubTrigger {
            disabled: props.disabled,
            class: class,
            attributes: props.attributes,
            {props.children}
            IconChevronRight { class: "ml-auto size-4" }
        }
    }
}

// ---------------------------------------------------------------------------
// ContextMenuSubContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuSubContentProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn ContextMenuSubContent(props: ContextMenuSubContentProps) -> Element {
    let class = tw_merge!(MENU_SUB_CONTENT, props.class);

    rsx! {
        primitives::ContextMenuSubContent {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
