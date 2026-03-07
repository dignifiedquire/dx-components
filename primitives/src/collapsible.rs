//! Collapsible primitive — matches `@radix-ui/react-collapsible`.
//!
//! A container that can be expanded or collapsed to show or hide its content.

use crate::{
    use_collapsible_content_dimensions, use_controlled, use_id_or, use_presence, use_unique_id,
};
use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct CollapsibleCtx {
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: bool,
    content_id: Signal<String>,
}

// ---------------------------------------------------------------------------
// Collapsible (root)
// ---------------------------------------------------------------------------

/// Props for [`Collapsible`].
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleProps {
    /// The controlled `open` state. Pass `None` for uncontrolled.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// The default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Whether the collapsible is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Callback fired when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// A container that can be expanded or collapsed.
///
/// Matches Radix's `Collapsible`. Sets `data-state="open"|"closed"` and
/// `data-disabled` when disabled.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::collapsible::*;
/// rsx! {
///     Collapsible {
///         CollapsibleTrigger { "Toggle" }
///         CollapsibleContent { "Hidden content" }
///     }
/// };
/// ```
#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    let content_id = use_unique_id();
    use_context_provider(|| CollapsibleCtx {
        open,
        set_open,
        disabled: props.disabled,
        content_id,
    });

    rsx! {
        div {
            "data-slot": "collapsible",
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": if props.disabled { "" },
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CollapsibleTrigger
// ---------------------------------------------------------------------------

/// Props for [`CollapsibleTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleTriggerProps {
    /// Override the auto-generated trigger ID.
    #[props(default)]
    pub id: Option<String>,

    // Explicit event props — required because `extends = GlobalAttributes`
    // does not capture event handlers (Dioxus limitation).
    /// Callback fired when the trigger is mounted.
    #[props(default)]
    pub onmounted: Callback<Event<MountedData>>,
    /// Callback fired when the trigger receives focus.
    #[props(default)]
    pub onfocus: Callback<Event<FocusData>>,
    /// Callback fired when a key is pressed on the trigger.
    #[props(default)]
    pub onkeydown: Callback<Event<KeyboardData>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// The button that toggles the collapsible open/closed state.
///
/// Matches Radix's `CollapsibleTrigger`. Renders a `<button>` with
/// `aria-expanded`, `aria-controls`, `data-state`, and `data-disabled`.
#[component]
pub fn CollapsibleTrigger(props: CollapsibleTriggerProps) -> Element {
    let ctx: CollapsibleCtx = use_context();
    let open = ctx.open;
    let disabled = ctx.disabled;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "collapsible-trigger",
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": if disabled { "" },
            disabled: disabled,
            aria_controls: ctx.content_id,
            aria_expanded: open(),
            class: props.class,

            onclick: move |_| {
                if !disabled {
                    ctx.set_open.call(!open());
                }
            },

            onmounted: props.onmounted,
            onfocus: props.onfocus,
            onkeydown: props.onkeydown,

            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CollapsibleContent
// ---------------------------------------------------------------------------

/// Props for [`CollapsibleContent`].
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleContentProps {
    /// Override the auto-generated content ID.
    #[props(default)]
    pub id: Option<String>,

    /// When `true`, children stay mounted in the DOM even when collapsed.
    /// Useful for animation libraries that need to control mount/unmount.
    #[props(default)]
    pub force_mount: bool,

    /// Additional inline styles, merged with computed CSS variable styles.
    ///
    /// Used by AccordionContent to alias `--radix-accordion-content-height/width`.
    #[props(default)]
    pub style: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// The collapsible content that shows/hides.
///
/// Matches Radix's `CollapsibleContent`. Sets `--radix-collapsible-content-height`
/// and `--radix-collapsible-content-width` CSS custom properties for animations.
/// The element is always in the DOM; visibility is controlled via the `hidden` attribute.
///
/// ## Data attributes
///
/// - `data-state`: `"open"` or `"closed"`
/// - `data-disabled`: Present when parent is disabled
#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element {
    let ctx: CollapsibleCtx = use_context();
    let user_id: ReadSignal<Option<String>> = use_hook(|| Signal::new(props.id.clone())).into();
    let id = use_id_or(ctx.content_id, user_id);
    let open = ctx.open;

    let mut presence = use_presence(open, id);
    let dims = use_collapsible_content_dimensions(id, open);
    let style = dims.style(props.style.as_deref());

    // Radix: isOpen = context.open || isPresent
    let is_open = open() || presence.is_present();

    rsx! {
        div {
            id: id,
            "data-slot": "collapsible-content",
            "data-state": presence.data_state(),
            "data-disabled": if ctx.disabled { "" },
            class: props.class,
            style: "{style}",
            hidden: !is_open,

            onanimationend: move |_| {
                presence.on_animation_end();
            },

            ..props.attributes,

            if is_open || props.force_mount {
                {props.children}
            }
        }
    }
}
