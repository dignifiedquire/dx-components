//! Styled dropdown menu matching shadcn/ui.
//!
//! Wraps `dioxus_primitives::dropdown_menu` with shadcn Tailwind classes.

use dioxus::prelude::*;
use dioxus_primitives::dropdown_menu as primitives;
pub use dioxus_primitives::dropdown_menu::{
    DropdownMenuGroup, DropdownMenuItemIndicator, DropdownMenuPortal, DropdownMenuRadioGroup,
    DropdownMenuSub,
};
use dx_icons_lucide::{IconCheck, IconChevronRight, IconCircle};
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Shared menu class constants
// ---------------------------------------------------------------------------

const MENU_CONTENT: &str = "z-50 max-h-(--radix-dropdown-menu-content-available-height) min-w-[8rem] origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto rounded-md border bg-popover p-1 text-popover-foreground shadow-md data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95";

const MENU_ITEM: &str = "relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[inset]:pl-8 data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 data-[variant=destructive]:focus:text-destructive dark:data-[variant=destructive]:focus:bg-destructive/20 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-muted-foreground data-[variant=destructive]:*:[svg]:text-destructive!";

const MENU_CHECKBOX_ITEM: &str = "relative flex cursor-default items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";

const MENU_RADIO_ITEM: &str = "relative flex cursor-default items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";

const MENU_LABEL: &str = "px-2 py-1.5 text-sm font-medium data-[inset]:pl-8";
const MENU_SEPARATOR: &str = "-mx-1 my-1 h-px bg-border";
const MENU_SHORTCUT: &str = "ml-auto text-xs tracking-widest text-muted-foreground";
const MENU_SUB_TRIGGER: &str = "flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[inset]:pl-8 data-[state=open]:bg-accent data-[state=open]:text-accent-foreground [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-muted-foreground";
const MENU_SUB_CONTENT: &str = "z-50 min-w-[8rem] origin-(--radix-dropdown-menu-content-transform-origin) overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-lg data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95";

// ---------------------------------------------------------------------------
// DropdownMenu (Root — no DOM)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuProps {
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
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    rsx! {
        primitives::DropdownMenuRoot {
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuTrigger
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuTriggerProps {
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    rsx! {
        primitives::DropdownMenuTrigger {
            r#as: props.r#as,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuContentProps {
    #[props(default)]
    pub force_mount: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element {
    let class = tw_merge!(MENU_CONTENT, props.class);

    rsx! {
        primitives::DropdownMenuContent {
            force_mount: props.force_mount,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuItem
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuItemProps {
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
pub fn DropdownMenuItem(props: DropdownMenuItemProps) -> Element {
    let class = tw_merge!(MENU_ITEM, props.class);

    rsx! {
        primitives::DropdownMenuItem {
            disabled: props.disabled,
            on_select: props.on_select,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuCheckboxItem
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuCheckboxItemProps {
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
pub fn DropdownMenuCheckboxItem(props: DropdownMenuCheckboxItemProps) -> Element {
    let class = tw_merge!(MENU_CHECKBOX_ITEM, props.class);

    rsx! {
        primitives::DropdownMenuCheckboxItem {
            checked: props.checked,
            disabled: props.disabled,
            on_checked_change: props.on_checked_change,
            class: class,
            attributes: props.attributes,

            primitives::DropdownMenuItemIndicator {
                class: "absolute left-2 inline-flex size-3.5 items-center justify-center",
                IconCheck { class: "size-4" }
            }
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuRadioItem
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuRadioItemProps {
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
pub fn DropdownMenuRadioItem(props: DropdownMenuRadioItemProps) -> Element {
    let class = tw_merge!(MENU_RADIO_ITEM, props.class);

    rsx! {
        primitives::DropdownMenuRadioItem {
            value: props.value,
            disabled: props.disabled,
            class: class,
            attributes: props.attributes,

            primitives::DropdownMenuItemIndicator {
                class: "absolute left-2 inline-flex size-3.5 items-center justify-center",
                IconCircle { class: "size-2 fill-current" }
            }
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuLabel
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuLabelProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn DropdownMenuLabel(props: DropdownMenuLabelProps) -> Element {
    let class = tw_merge!(MENU_LABEL, props.class);

    rsx! {
        primitives::DropdownMenuLabel {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuSeparator
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuSeparatorProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn DropdownMenuSeparator(props: DropdownMenuSeparatorProps) -> Element {
    let class = tw_merge!(MENU_SEPARATOR, props.class);

    rsx! {
        primitives::DropdownMenuSeparator {
            class: class,
            attributes: props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuShortcut
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuShortcutProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn DropdownMenuShortcut(props: DropdownMenuShortcutProps) -> Element {
    let class = tw_merge!(MENU_SHORTCUT, props.class);

    rsx! {
        primitives::DropdownMenuShortcut {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuSubTrigger
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuSubTriggerProps {
    #[props(default)]
    pub disabled: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn DropdownMenuSubTrigger(props: DropdownMenuSubTriggerProps) -> Element {
    let class = tw_merge!(MENU_SUB_TRIGGER, props.class);

    rsx! {
        primitives::DropdownMenuSubTrigger {
            disabled: props.disabled,
            class: class,
            attributes: props.attributes,
            {props.children}
            IconChevronRight { class: "ml-auto size-4" }
        }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuSubContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuSubContentProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn DropdownMenuSubContent(props: DropdownMenuSubContentProps) -> Element {
    let class = tw_merge!(MENU_SUB_CONTENT, props.class);

    rsx! {
        primitives::DropdownMenuSubContent {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
