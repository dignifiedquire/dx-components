//! Defines the [`Navbar`] component and its sub-components.

use crate::{
    focus::{
        use_focus_control, use_focus_controlled_item, use_focus_entry, use_focus_provider,
        FocusState,
    },
    use_animated_open, use_id_or, use_unique_id,
};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct NavbarContext {
    // Currently open nav index
    open_nav: Signal<Option<usize>>,
    set_open_nav: Callback<Option<usize>>,
    disabled: bool,

    // Focus state
    focus: FocusState,
}

/// The props for the [`Navbar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct NavbarProps {
    /// Whether the navbar is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether focus should loop around when reaching the end.
    #[props(default = true)]
    pub roving_loop: bool,

    /// Additional attributes to apply to the navbar element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the navbar component.
    pub children: Element,
}

/// # Navbar
///
/// The `Navbar` component creates a navigation bar that allows users to navigate
/// through different sections with keyboard and pointer interactions.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::navbar::{Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarTrigger};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Navbar {
///             aria_label: "Components",
///             NavbarNav { index: 0usize,
///                 NavbarTrigger {
///                     "Inputs"
///                 }
///                 NavbarContent {
///                     NavbarItem {
///                         index: 0usize,
///                         value: "calendar".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=calendar",
///                         "Calendar"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// - `data-slot`: `"navbar"` on the root, `"navbar-menubar"` on the inner menubar.
/// - `data-disabled`: Present when the navbar is disabled.
#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    let mut open_nav = use_signal(|| None);
    let set_open_nav = use_callback(move |idx| open_nav.set(idx));

    let roving_loop_signal = use_signal(move || props.roving_loop);
    let focus = use_focus_provider(roving_loop_signal.into());
    let mut ctx = use_context_provider(|| NavbarContext {
        open_nav,
        set_open_nav,
        disabled: props.disabled,
        focus,
    });
    use_effect(move || {
        let index = ctx.focus.current_focus();
        if ctx.open_nav.peek().is_some() {
            ctx.set_open_nav.call(index);
        }
    });

    let aria_label = props
        .attributes
        .iter()
        .find_map(|attr| (attr.name == "aria-label").then(|| attr.value.clone()));

    rsx! {
        div {
            "data-slot": "navbar",
            role: "navigation",
            display: "content",
            aria_label,
            div {
                role: "menubar",
                "data-slot": "navbar-menubar",
                "data-disabled": if props.disabled { "" } else { None::<&str> },
                tabindex: (!ctx.focus.any_focused()).then_some("0"),
                onfocus: move |_| {
                    ctx.focus.set_focus(Some(ctx.focus.recent_focus_or_default()));
                },
                onkeydown: move |event: Event<KeyboardData>| {
                    match event.key() {
                        Key::Escape => ctx.set_open_nav.call(None),
                        Key::ArrowLeft => ctx.focus.focus_prev(),
                        Key::ArrowRight => ctx.focus.focus_next(),
                        Key::Home => ctx.focus.focus_first(),
                        Key::End => ctx.focus.focus_last(),
                        _ => return,
                    }
                    event.prevent_default();
                },

                ..props.attributes,

                {props.children}
            }
        }
    }
}

#[derive(Clone, Copy)]
struct NavbarNavContext {
    index: usize,
    focus: FocusState,
    is_open: Memo<bool>,
    disabled: bool,
}

impl NavbarNavContext {
    fn focus_next(&mut self) {
        self.focus.focus_next();
    }

    fn focus_prev(&mut self) {
        self.focus.focus_prev();
    }
}

/// The props for the [`NavbarNav`] component.
#[derive(Props, Clone, PartialEq)]
pub struct NavbarNavProps {
    /// The index of this nav item in the navbar.
    pub index: ReadSignal<usize>,

    /// Whether this nav item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional attributes to apply to the nav element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the nav component.
    pub children: Element,
}

/// # NavbarNav
///
/// A single navigation dropdown within a navbar.
///
/// ## Styling
///
/// - `data-state`: `"open"` or `"closed"`.
/// - `data-disabled`: Present when the nav is disabled.
#[component]
pub fn NavbarNav(props: NavbarNavProps) -> Element {
    let mut ctx: NavbarContext = use_context();
    let is_open = use_memo(move || (ctx.open_nav)() == Some(props.index.cloned()));
    let roving_loop_signal = use_signal(|| true);
    let focus = use_focus_provider(roving_loop_signal.into());
    let index_val = props.index.cloned();
    let mut nav_ctx = use_context_provider(|| NavbarNavContext {
        index: index_val,
        focus,
        is_open,
        disabled: props.disabled,
    });

    use_effect(move || {
        if !is_open() {
            nav_ctx.focus.blur();
        }
    });

    use_focus_entry(ctx.focus, props.index);

    let disabled = ctx.disabled || props.disabled;

    rsx! {
        div {
            "data-slot": "navbar-nav",
            role: "menu",
            "data-state": if is_open() { "open" } else { "closed" },
            "data-disabled": if disabled { "" } else { None::<&str> },

            onmouseenter: move |_| {
                if !disabled {
                    let index = Some(nav_ctx.index);
                    if (ctx.open_nav)().is_some() {
                        ctx.focus.set_focus(index);
                    } else {
                        ctx.set_open_nav.call(index);
                    }
                }
            },
            onmouseleave: move |_| {
                if is_open() {
                    ctx.focus.set_focus(None);
                }
            },
            onkeydown: move |event: Event<KeyboardData>| {
                match event.key() {
                    Key::Enter if !disabled => {
                        ctx.set_open_nav.call((!is_open()).then_some(nav_ctx.index));
                    }
                    Key::ArrowDown if !disabled => {
                        if !is_open() {
                            ctx.set_open_nav.call(Some(props.index.cloned()));
                        }
                        nav_ctx.focus_next();
                    },
                    Key::ArrowUp if !disabled => {
                        if is_open() {
                            nav_ctx.focus_prev();
                        }
                    },
                    _ => return,
                }
                event.prevent_default();
            },

            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`NavbarTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct NavbarTriggerProps {
    /// Additional attributes to apply to the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the trigger component.
    pub children: Element,
}

/// # NavbarTrigger
///
/// A button that opens and closes a [`NavbarNav`].
#[component]
pub fn NavbarTrigger(props: NavbarTriggerProps) -> Element {
    let mut ctx: NavbarContext = use_context();
    let nav_ctx: NavbarNavContext = use_context();
    let index_signal: ReadSignal<usize> = use_signal(move || nav_ctx.index).into();
    let onmounted = use_focus_control(ctx.focus, index_signal);
    let is_focused =
        move || ctx.focus.current_focus() == Some(nav_ctx.index) && !nav_ctx.focus.any_focused();
    let disabled = ctx.disabled || nav_ctx.disabled;
    let is_open = nav_ctx.is_open;

    rsx! {
        button {
            "data-slot": "navbar-trigger",
            "data-state": if is_open() { "open" } else { "closed" },
            onmounted,
            onpointerdown: move |_| {
                if !disabled {
                    let new_open = if is_open() { None } else { Some(nav_ctx.index) };
                    ctx.set_open_nav.call(new_open);
                }
            },
            onblur: move |_| {
                if is_focused() {
                    ctx.focus.set_focus(None);
                    ctx.set_open_nav.call(None);
                }
            },
            role: "menuitem",
            r#type: "button",
            tabindex: if is_focused() { "0" } else { "-1" },
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`NavbarContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct NavbarContentProps {
    /// The id of the content element.
    #[props(default)]
    pub id: Option<String>,

    /// Additional attributes to apply to the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the content component.
    pub children: Element,
}

/// # NavbarContent
///
/// The dropdown content of a [`NavbarNav`].
///
/// ## Styling
///
/// - `data-state`: `"open"` or `"closed"`.
/// - `data-open-menu-direction`: Direction of the open menu relative to this content.
#[component]
pub fn NavbarContent(props: NavbarContentProps) -> Element {
    let ctx: NavbarContext = use_context();
    let nav_ctx: NavbarNavContext = use_context();
    let index = nav_ctx.index;
    let open_direction = match (ctx.open_nav)() {
        Some(open_index) if open_index > index => "start",
        Some(open_index) if open_index < index => "end",
        Some(_) => "open",
        None => "closed",
    };

    let unique_id = use_unique_id();
    let id_signal = use_signal(move || props.id.clone());
    let id = use_id_or(unique_id, id_signal.into());

    let render = use_animated_open(id, nav_ctx.is_open);

    rsx! {
        if render() {
            div {
                id,
                "data-slot": "navbar-content",
                role: "menu",
                "data-state": if (nav_ctx.is_open)() { "open" } else { "closed" },
                "data-open-menu-direction": "{open_direction}",
                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// The props for the [`NavbarItem`] component.
#[derive(Props, Clone, PartialEq)]
pub struct NavbarItemProps {
    /// The index of this item within the nav.
    pub index: ReadSignal<usize>,

    /// The value associated with this nav item.
    pub value: String,

    /// Whether this nav item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Callback fired when the item is selected.
    #[props(default)]
    pub on_select: Callback<String>,

    /// A class to apply to the anchor tag when the target route is active.
    pub active_class: Option<String>,

    /// When [`true`], the `target` route will be opened in a new tab.
    #[props(default)]
    pub new_tab: bool,

    /// The onclick event handler.
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// The onmounted event handler.
    pub onmounted: Option<EventHandler<MountedEvent>>,

    #[props(default)]
    /// Whether only the onclick handler should execute.
    pub onclick_only: bool,

    /// The rel attribute for the generated HTML anchor tag.
    pub rel: Option<String>,

    /// The navigation target.
    #[props(into)]
    pub to: NavigationTarget,

    /// Additional attributes to apply to the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render within the generated HTML anchor tag.
    pub children: Element,
}

/// # NavbarItem
///
/// A link within a navbar.
///
/// ## Styling
///
/// - `data-disabled`: Present when the item is disabled.
#[component]
pub fn NavbarItem(mut props: NavbarItemProps) -> Element {
    let mut ctx: NavbarContext = use_context();
    let mut nav_ctx: Option<NavbarNavContext> = try_use_context();

    let disabled = ctx.disabled || props.disabled;
    let focused = move || {
        nav_ctx.map_or_else(
            || ctx.focus.is_focused(props.index.cloned()),
            |nav_ctx| nav_ctx.focus.is_focused(props.index.cloned()) && (nav_ctx.is_open)(),
        )
    };

    let mut onmounted = use_focus_controlled_item(props.index);

    props.attributes.push(onkeydown({
        let value = props.value.clone();
        let to = props.to.clone();
        move |event: Event<KeyboardData>| {
            if event.key() == Key::Enter || event.key() == Key::Character(" ".to_string()) {
                if !disabled {
                    props.on_select.call(value.clone());
                    ctx.set_open_nav.call(None);
                    let navigator = navigator();
                    navigator.push(to.clone());
                }
                event.prevent_default();
                event.stop_propagation();
            }
        }
    }));

    props.attributes.push(onpointerdown(move |_| {
        if let Some(mut nav_ctx) = nav_ctx {
            nav_ctx.focus.set_focus(Some(props.index.cloned()));
        }
    }));

    props.attributes.push(onblur(move |_| {
        if focused() {
            if let Some(nav_ctx) = &mut nav_ctx {
                nav_ctx.focus.blur();
            }
            ctx.focus.set_focus(None);
            ctx.set_open_nav.call(None);
        }
    }));

    let tabindex = if focused()
        || (nav_ctx.is_none() && ctx.focus.recent_focus_or_default() == props.index.cloned())
    {
        "0"
    } else {
        "-1"
    };

    rsx! {
        Link {
            "data-slot": "navbar-item",
            active_class: props.active_class,
            new_tab: props.new_tab,
            onclick_only: props.onclick_only,
            rel: props.rel,
            to: props.to,
            role: "menuitem",
            "data-disabled": if disabled { "" } else { None::<&str> },
            tabindex,

            onclick: {
                let value = props.value.clone();
                move |mouse_event| {
                    if !disabled {
                        props.on_select.call(value.clone());
                        ctx.set_open_nav.call(None);
                    }
                    if let Some(onclick) = props.onclick {
                        onclick.call(mouse_event);
                    }
                }
            },

            onmounted: move |evt: MountedEvent| {
                onmounted(evt.clone());
                if let Some(onmounted) = &props.onmounted {
                    onmounted.call(evt);
                }
            },

            attributes: props.attributes,
            {props.children}
        }
    }
}
