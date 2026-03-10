//! Styled menubar matching shadcn/ui.
//!
//! Wraps `dioxus_primitives::menubar` with shadcn Tailwind classes.

use dioxus::prelude::*;
use dioxus_primitives::menubar as primitives;
pub use dioxus_primitives::menubar::{
    MenubarGroup, MenubarItemIndicator, MenubarMenu, MenubarPortal, MenubarRadioGroup, MenubarSub,
};
use dx_icons_lucide::{IconCheck, IconChevronRight, IconCircle};
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Shared class constants
// ---------------------------------------------------------------------------

const MENUBAR_CONTENT: &str = "z-50 min-w-[12rem] origin-(--radix-menubar-content-transform-origin) overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95";

const MENU_ITEM: &str = "relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[inset]:pl-8 data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 data-[variant=destructive]:focus:text-destructive dark:data-[variant=destructive]:focus:bg-destructive/20 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-muted-foreground data-[variant=destructive]:*:[svg]:text-destructive!";

const MENU_CHECKBOX_ITEM: &str = "relative flex cursor-default items-center gap-2 rounded-xs py-1.5 pr-2 pl-8 text-sm outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";

const MENU_RADIO_ITEM: &str = "relative flex cursor-default items-center gap-2 rounded-xs py-1.5 pr-2 pl-8 text-sm outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";

const MENU_LABEL: &str = "px-2 py-1.5 text-sm font-medium data-[inset]:pl-8";
const MENU_SEPARATOR: &str = "-mx-1 my-1 h-px bg-border";
const MENU_SHORTCUT: &str = "ml-auto text-xs tracking-widest text-muted-foreground";
const MENU_SUB_TRIGGER: &str = "flex cursor-default items-center rounded-sm px-2 py-1.5 text-sm outline-none select-none focus:bg-accent focus:text-accent-foreground data-[inset]:pl-8 data-[state=open]:bg-accent data-[state=open]:text-accent-foreground";
const MENU_SUB_CONTENT: &str = "z-50 min-w-[8rem] origin-(--radix-menubar-content-transform-origin) overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-lg data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95";

// ---------------------------------------------------------------------------
// Menubar (Root)
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct MenubarProps {
    #[props(default)]
    pub disabled: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn Menubar(props: MenubarProps) -> Element {
    let class = tw_merge!(
        "flex h-9 items-center gap-1 rounded-md border bg-background p-1 shadow-xs",
        props.class,
    );

    rsx! {
        primitives::MenubarRoot {
            disabled: props.disabled,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarTrigger
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct MenubarTriggerProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    let class = tw_merge!(
        "flex cursor-pointer items-center rounded-sm px-2 py-1 text-sm font-medium outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[state=open]:bg-accent data-[state=open]:text-accent-foreground",
        props.class,
    );

    rsx! {
        primitives::MenubarTrigger {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct MenubarContentProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    let class = tw_merge!(MENUBAR_CONTENT, props.class);

    rsx! {
        primitives::MenubarContent {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarItem
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct MenubarItemProps {
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
pub fn MenubarItem(props: MenubarItemProps) -> Element {
    let class = tw_merge!(MENU_ITEM, props.class);

    rsx! {
        primitives::MenubarItem {
            disabled: props.disabled,
            on_select: props.on_select,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarCheckboxItem
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct MenubarCheckboxItemProps {
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
pub fn MenubarCheckboxItem(props: MenubarCheckboxItemProps) -> Element {
    let class = tw_merge!(MENU_CHECKBOX_ITEM, props.class);

    rsx! {
        primitives::MenubarCheckboxItem {
            checked: props.checked,
            disabled: props.disabled,
            on_checked_change: props.on_checked_change,
            class: class,
            attributes: props.attributes,

            primitives::MenubarItemIndicator {
                class: "absolute left-2 inline-flex size-3.5 items-center justify-center",
                IconCheck { class: "size-4" }
            }
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarRadioItem
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct MenubarRadioItemProps {
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
pub fn MenubarRadioItem(props: MenubarRadioItemProps) -> Element {
    let class = tw_merge!(MENU_RADIO_ITEM, props.class);

    rsx! {
        primitives::MenubarRadioItem {
            value: props.value,
            disabled: props.disabled,
            class: class,
            attributes: props.attributes,

            primitives::MenubarItemIndicator {
                class: "absolute left-2 inline-flex size-3.5 items-center justify-center",
                IconCircle { class: "size-2 fill-current" }
            }
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarLabel
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct MenubarLabelProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn MenubarLabel(props: MenubarLabelProps) -> Element {
    let class = tw_merge!(MENU_LABEL, props.class);

    rsx! {
        primitives::MenubarLabel {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarSeparator
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct MenubarSeparatorProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn MenubarSeparator(props: MenubarSeparatorProps) -> Element {
    let class = tw_merge!(MENU_SEPARATOR, props.class);

    rsx! {
        primitives::MenubarSeparator {
            class: class,
            attributes: props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarShortcut
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct MenubarShortcutProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn MenubarShortcut(props: MenubarShortcutProps) -> Element {
    let class = tw_merge!(MENU_SHORTCUT, props.class);

    rsx! {
        primitives::MenubarShortcut {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarSubTrigger
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct MenubarSubTriggerProps {
    #[props(default)]
    pub disabled: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn MenubarSubTrigger(props: MenubarSubTriggerProps) -> Element {
    let class = tw_merge!(MENU_SUB_TRIGGER, props.class);

    rsx! {
        primitives::MenubarSubTrigger {
            disabled: props.disabled,
            class: class,
            attributes: props.attributes,
            {props.children}
            IconChevronRight { class: "ml-auto size-4" }
        }
    }
}

// ---------------------------------------------------------------------------
// MenubarSubContent
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct MenubarSubContentProps {
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn MenubarSubContent(props: MenubarSubContentProps) -> Element {
    let class = tw_merge!(MENU_SUB_CONTENT, props.class);

    rsx! {
        primitives::MenubarSubContent {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}
