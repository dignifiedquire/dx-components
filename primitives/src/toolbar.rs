//! Defines the [`Toolbar`] component and its sub-components, which provide a container to group related buttons and controls with keyboard navigation.

use dioxus::prelude::*;
use std::rc::Rc;
use tailwind_fuse::*;

#[derive(Clone, Copy)]
struct ToolbarCtx {
    // State
    disabled: ReadSignal<bool>,

    // Focus management
    focused_index: Signal<Option<usize>>,

    // Orientation
    horizontal: ReadSignal<bool>,
}

impl ToolbarCtx {
    fn set_focus(&mut self, index: Option<usize>) {
        self.focused_index.set(index);
    }

    fn is_focused(&self, index: usize) -> bool {
        (self.focused_index)() == Some(index)
    }

    fn orientation(&self) -> &'static str {
        if (self.horizontal)() {
            "horizontal"
        } else {
            "vertical"
        }
    }
}

/// The props for the [`Toolbar`] component
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarProps {
    /// Whether the toolbar is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the toolbar is horizontal (true) or vertical (false)
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub horizontal: ReadSignal<bool>,

    /// ARIA label for the toolbar
    #[props(default)]
    pub aria_label: Option<String>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes for the toolbar
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the toolbar, which should include multiple [`ToolbarButton`] components.
    pub children: Element,
}

/// # Toolbar
///
/// The `Toolbar` component creates an container for grouping related buttons and controls. It supports keyboard navigation with arrow keys between adjacent [`ToolbarButton`]s.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::toolbar::{Toolbar, ToolbarButton, ToolbarSeparator};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Toolbar { aria_label: "Text formatting",
///             ToolbarButton {
///                 index: 0usize,
///                 on_click: move |_| tracing::info!("Bold clicked"),
///                 "Bold"
///             }
///             ToolbarSeparator {}
///             ToolbarButton {
///                 index: 1usize,
///                 on_click: move |_| tracing::info!("Italic clicked"),
///                 "Italic"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Toolbar`] component defines the following data attributes you can use to control styling:
/// - `data-orientation`: Indicates the orientation of the toolbar. Values are `horizontal` or `vertical`.
/// - `data-disabled`: Indicates if the toolbar is disabled. Values are `true` or `false`.
#[component]
pub fn Toolbar(props: ToolbarProps) -> Element {
    let mut ctx = use_context_provider(|| ToolbarCtx {
        disabled: props.disabled,
        focused_index: Signal::new(None),
        horizontal: props.horizontal,
    });

    let class = tw_merge!(
        "flex items-center gap-1 rounded-md border bg-background p-1",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "toolbar",
            class: class,
            role: "toolbar",
            "data-orientation": ctx.orientation(),
            "data-disabled": (props.disabled)(),
            aria_label: props.aria_label,

            onfocusout: move |_| ctx.set_focus(None),
            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`ToolbarButton`] component
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarButtonProps {
    /// Index of the button in the toolbar. This is used to define the focus order for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// Whether the button is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Callback when the button is clicked
    #[props(default)]
    pub on_click: Callback<()>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes for the button
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the button
    pub children: Element,
}

/// # ToolbarButton
///
/// A button component within a [`Toolbar`] with focus controlled by the toolbar context for keyboard navigation.
///
/// This must be used inside a [`Toolbar`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::toolbar::{Toolbar, ToolbarButton, ToolbarSeparator};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Toolbar { aria_label: "Text formatting",
///             ToolbarButton {
///                 index: 0usize,
///                 on_click: move |_| tracing::info!("Bold clicked"),
///                 "Bold"
///             }
///             ToolbarSeparator {}
///             ToolbarButton {
///                 index: 1usize,
///                 on_click: move |_| tracing::info!("Italic clicked"),
///                 "Italic"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`ToolbarButton`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the button is disabled. Values are `true` or `false`.
#[component]
pub fn ToolbarButton(props: ToolbarButtonProps) -> Element {
    let mut ctx: ToolbarCtx = use_context();

    // Handle button ref for focus management
    let mut button_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    // Check if this button is focused
    let is_focused = use_memo(move || ctx.is_focused((props.index)()));

    // Set focus when needed
    use_effect(move || {
        if is_focused() {
            if let Some(md) = button_ref() {
                spawn(async move {
                    let _ = md.set_focus(true).await;
                });
            }
        }
    });

    let class = tw_merge!(
        "inline-flex items-center justify-center gap-2 rounded-md px-3 py-1.5 text-sm font-medium whitespace-nowrap transition-[color,box-shadow] outline-none hover:bg-muted hover:text-muted-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        props.class,
    );

    rsx! {
        button {
            "data-slot": "toolbar-button",
            class: class,
            type: "button",
            tabindex: "0",
            disabled: (ctx.disabled)() || (props.disabled)(),
            "data-disabled": (ctx.disabled)() || (props.disabled)(),

            onmounted: move |data: Event<MountedData>| button_ref.set(Some(data.data())),
            onfocus: move |_| ctx.set_focus(Some((props.index)())),

            onclick: move |_| {
                if !(ctx.disabled)() && !(props.disabled)() {
                    props.on_click.call(());
                }
            },

            onkeydown: move |event: Event<KeyboardData>| {
                let key = event.key();
                let horizontal = (ctx.horizontal)();
                let mut prevent_default = true;
                match key {
                    Key::ArrowUp if !horizontal => {
                        let index = (props.index)();
                        if index > 0 {
                            ctx.set_focus(Some(index - 1));
                        }
                    }
                    Key::ArrowDown if !horizontal => {
                        let index = (props.index)();
                        ctx.set_focus(Some(index + 1));
                    }
                    Key::ArrowLeft if horizontal => {
                        let index = (props.index)();
                        if index > 0 {
                            ctx.set_focus(Some(index - 1));
                        }
                    }
                    Key::ArrowRight if horizontal => {
                        let index = (props.index)();
                        ctx.set_focus(Some(index + 1));
                    }
                    Key::Home => {
                        ctx.set_focus(Some(0));
                    }
                    Key::End => {
                        ctx.set_focus(Some(100));
                    }
                    _ => prevent_default = false,
                };
                if prevent_default {
                    event.prevent_default();
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`ToolbarSeparator`] component
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarSeparatorProps {
    /// Whether the separator is horizontal (true) or vertical (false)
    #[props(default)]
    pub horizontal: Option<bool>,

    /// If the separator is decorative and should not be classified
    /// as a separator to the ARIA standard.
    #[props(default = false)]
    pub decorative: bool,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes for the separator
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # ToolbarSeparator
///
/// A separator within a [`Toolbar`] that helps divide different sections. The separator can be horizontal or vertical and can be marked as decorative.
///
/// This must be used inside a [`Toolbar`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::toolbar::{Toolbar, ToolbarButton, ToolbarSeparator};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Toolbar { aria_label: "Text formatting",
///             ToolbarButton {
///                 index: 0usize,
///                 on_click: move |_| tracing::info!("Bold clicked"),
///                 "Bold"
///             }
///             ToolbarSeparator {}
///             ToolbarButton {
///                 index: 1usize,
///                 on_click: move |_| tracing::info!("Italic clicked"),
///                 "Italic"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`ToolbarSeparator`] component defines the following data attributes you can use to control styling:
/// - `data-orientation`: Indicates the orientation of the separator. Values are `horizontal` or `vertical`.
#[component]
pub fn ToolbarSeparator(props: ToolbarSeparatorProps) -> Element {
    let ctx: ToolbarCtx = use_context();

    // If horizontal is explicitly set, use that, otherwise invert the toolbar orientation
    let horizontal = props.horizontal.unwrap_or(!(ctx.horizontal)());

    let orientation = match horizontal {
        true => "horizontal",
        false => "vertical",
    };

    let class = tw_merge!(
        "shrink-0 bg-border data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "toolbar-separator",
            class: class,
            role: if !props.decorative { "separator" } else { "none" },
            aria_orientation: if !props.decorative { orientation },
            "data-orientation": orientation,
            ..props.attributes,
        }
    }
}
