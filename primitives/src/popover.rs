//! Popover primitive — matches `@radix-ui/react-popover`.
//!
//! Displays rich content above the page in the browser top layer, triggered
//! by a button. Content escapes ancestor `overflow`, `transform`, and
//! stacking contexts natively via the `popover` attribute — no portal
//! required. See [`crate::top_layer`] for the underlying mechanism.

use std::rc::Rc;

use dioxus::prelude::*;

use crate::aria_hidden::use_aria_hidden;
use crate::dismissable_layer::{
    use_dismissable_layer, DismissableEvent, DismissableLayerOptions, DismissableSource,
};
use crate::focus_guards::use_focus_guards;
use crate::focus_scope::{AutoFocusEvent, FocusScope};
use crate::popper::{Align, CollisionPadding, Popper, PopperContent, PopperCtx, Side, Sticky};
use crate::presence::{Presence, PresenceContext};
use crate::scroll_lock::use_scroll_lock;
use crate::top_layer::{use_top_layer, TopLayerKind};
use crate::{merge_attributes, use_controlled, use_id_or, use_unique_id};
use dioxus_attributes::attributes;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

/// Context shared by all Popover sub-components.
#[derive(Clone, Copy)]
pub struct PopoverCtx {
    pub(crate) open: Memo<bool>,
    pub(crate) set_open: Callback<bool>,
    pub(crate) is_modal: bool,
    pub(crate) content_id: Signal<String>,
    pub(crate) trigger_ref: Signal<Option<Rc<MountedData>>>,
}

impl PopoverCtx {
    /// Returns whether the popover is open.
    pub fn is_open(&self) -> bool {
        self.open.cloned()
    }

    /// Sets the open state of the popover.
    pub fn set_open(&self, open: bool) {
        self.set_open.call(open);
    }
}

// ---------------------------------------------------------------------------
// PopoverRoot
// ---------------------------------------------------------------------------

/// Props for [`PopoverRoot`].
#[derive(Props, Clone, PartialEq)]
pub struct PopoverRootProps {
    /// Whether the popover is modal (traps focus). Defaults to `false` (matching Radix).
    #[props(default)]
    pub modal: bool,

    /// The controlled `open` state.
    pub open: ReadSignal<Option<bool>>,

    /// The default `open` state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// The children.
    pub children: Element,
}

/// The root of the popover. Manages state and provides context.
///
/// Wraps children in [`Popper`] for positioning.
#[component]
pub fn PopoverRoot(props: PopoverRootProps) -> Element {
    let content_id = use_unique_id();
    let trigger_ref = use_signal(|| None);

    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    use_context_provider(|| PopoverCtx {
        open,
        set_open,
        is_modal: props.modal,
        content_id,
        trigger_ref,
    });

    rsx! {
        Popper {
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// PopoverTrigger
// ---------------------------------------------------------------------------

/// Props for [`PopoverTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct PopoverTriggerProps {
    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children.
    pub children: Element,
}

/// A button that toggles the popover. Also sets the Popper anchor ref.
#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    let ctx: PopoverCtx = use_context();
    let popper_ctx: PopperCtx = use_context();
    let open = ctx.open;
    let set_open = ctx.set_open;
    let mut trigger_ref = ctx.trigger_ref;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "popover-trigger",
            "data-state": if open() { "open" } else { "closed" },
            aria_haspopup: "dialog",
            aria_expanded: open(),
            aria_controls: ctx.content_id,
            class: props.class,
            onclick: move |_| set_open.call(!open()),
            onmounted: move |e| {
                let data = e.data();
                trigger_ref.set(Some(data.clone()));
                popper_ctx.set_anchor_ref(data);
            },
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// PopoverContent
// ---------------------------------------------------------------------------

/// Props for [`PopoverContent`].
#[derive(Props, Clone, PartialEq)]
pub struct PopoverContentProps {
    /// The ID of the content element.
    pub id: ReadSignal<Option<String>>,

    /// When true, the content is always rendered in the DOM.
    #[props(default)]
    pub force_mount: bool,

    /// Side of the trigger to place the popover. Defaults to `Bottom`.
    #[props(default)]
    pub side: Side,

    /// Offset from the trigger edge in pixels. Defaults to 0.
    #[props(default)]
    pub side_offset: f64,

    /// Alignment relative to the trigger. Defaults to `Center`.
    #[props(default)]
    pub align: Align,

    /// Offset along the alignment axis. Defaults to 0.
    #[props(default)]
    pub align_offset: f64,

    /// Whether to avoid viewport edge collisions. Defaults to `true`.
    #[props(default = true)]
    pub avoid_collisions: bool,

    /// Collision padding in pixels. Defaults to 0.
    #[props(default)]
    pub collision_padding: CollisionPadding,

    /// Whether the content stays aligned with the anchor when it would
    /// otherwise be pushed off. Upstream: `sticky` (default `"partial"`).
    #[props(default)]
    pub sticky: Sticky,

    /// Hide the content when the anchor is fully clipped out of view.
    /// Upstream: `hideWhenDetached` (default `false`).
    #[props(default)]
    pub hide_when_detached: bool,

    /// Called before focus moves into the content on open. Call
    /// [`AutoFocusEvent::prevent_default`] to manage focus yourself.
    /// Upstream: `onOpenAutoFocus`.
    #[props(default)]
    pub on_open_auto_focus: Callback<AutoFocusEvent>,

    /// Called before focus returns to the trigger on close. Can be prevented.
    /// Upstream: `onCloseAutoFocus`.
    #[props(default)]
    pub on_close_auto_focus: Callback<AutoFocusEvent>,

    /// Called when Escape is pressed while this is the topmost layer. Can be
    /// prevented. Upstream: `onEscapeKeyDown`.
    #[props(default)]
    pub on_escape_key_down: Callback<DismissableEvent>,

    /// Called on a pointer-down outside the content. Can be prevented.
    /// Upstream: `onPointerDownOutside`.
    #[props(default)]
    pub on_pointer_down_outside: Callback<DismissableEvent>,

    /// Called when focus moves outside the content. Can be prevented.
    /// Upstream: `onFocusOutside`.
    #[props(default)]
    pub on_focus_outside: Callback<DismissableEvent>,

    /// Called on any outside interaction, pointer or focus. Can be prevented.
    /// Upstream: `onInteractOutside`.
    #[props(default)]
    pub on_interact_outside: Callback<DismissableEvent>,

    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children.
    pub children: Element,
}

/// The content panel of the popover.
///
/// Mirrors upstream's composition: [`Presence`] gates mounting so exit
/// animations can finish, then [`FocusScope`] + a dismissable layer +
/// [`PopperContent`] supply focus management, dismissal and positioning.
///
/// Renders with `role="dialog"`, `data-state`, `data-side` and `data-align`.
/// Modal popovers additionally lock scroll and `aria-hidden` the rest of the
/// page.
#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
    let ctx: PopoverCtx = use_context();
    let open = ctx.open;
    let id = use_id_or(ctx.content_id, props.id);

    rsx! {
        Presence {
            present: props.force_mount || open(),
            id,
            PopoverContentImpl {
                content_id: id,
                side: props.side,
                side_offset: props.side_offset,
                align: props.align,
                align_offset: props.align_offset,
                avoid_collisions: props.avoid_collisions,
                collision_padding: props.collision_padding,
                sticky: props.sticky,
                hide_when_detached: props.hide_when_detached,
                on_open_auto_focus: props.on_open_auto_focus,
                on_close_auto_focus: props.on_close_auto_focus,
                on_escape_key_down: props.on_escape_key_down,
                on_pointer_down_outside: props.on_pointer_down_outside,
                on_focus_outside: props.on_focus_outside,
                on_interact_outside: props.on_interact_outside,
                class: props.class,
                attributes: props.attributes,
                children: props.children,
            }
        }
    }
}

/// Props for [`PopoverContentImpl`].
#[derive(Props, Clone, PartialEq)]
struct PopoverContentImplProps {
    content_id: Memo<String>,
    side: Side,
    side_offset: f64,
    align: Align,
    align_offset: f64,
    avoid_collisions: bool,
    collision_padding: CollisionPadding,
    sticky: Sticky,
    hide_when_detached: bool,
    on_open_auto_focus: Callback<AutoFocusEvent>,
    on_close_auto_focus: Callback<AutoFocusEvent>,
    on_escape_key_down: Callback<DismissableEvent>,
    on_pointer_down_outside: Callback<DismissableEvent>,
    on_focus_outside: Callback<DismissableEvent>,
    on_interact_outside: Callback<DismissableEvent>,
    class: Option<String>,
    attributes: Vec<Attribute>,
    children: Element,
}

/// Everything that only exists while the content is mounted.
///
/// Upstream splits this the same way (`PopoverContentImpl` behind `Presence`,
/// with `PopoverContentModal` / `PopoverContentNonModal` choosing the props).
/// The split matters beyond tidiness: this component sits *inside* `Presence`,
/// so it can read [`PresenceContext`] and keep the element in the top layer
/// until the exit animation has finished rather than the instant `open` flips.
#[component]
fn PopoverContentImpl(props: PopoverContentImplProps) -> Element {
    let ctx: PopoverCtx = use_context();
    let open = ctx.open;
    let set_open = ctx.set_open;
    let is_modal = ctx.is_modal;
    let trigger_ref = ctx.trigger_ref;
    let id = props.content_id;

    // Upstream: `useFocusGuards()` — the overlay can be the last element in the
    // DOM, so tabbing past it must not fall out of the document.
    use_focus_guards();

    // Upstream modal branch: `<RemoveScroll>` plus `hideOthers(content)`, which
    // its comment calls "the better supported equivalent to setting
    // aria-modal". We therefore emit no `aria-modal` either.
    let modal_active = use_memo(move || is_modal && open());
    use_scroll_lock(modal_active);
    use_aria_hidden(id, modal_active);

    // Upstream refs. Modal keeps `isRightClickOutsideRef`; non-modal keeps
    // `hasInteractedOutsideRef` and `hasPointerDownOutsideRef`.
    let mut is_right_click_outside = use_signal(|| false);
    let mut has_interacted_outside = use_signal(|| false);
    let mut has_pointer_down_outside = use_signal(|| false);

    let focus_trigger = use_callback(move |_: ()| {
        // Synchronous on purpose: this runs from FocusScope's unmount cleanup,
        // where a spawned task would be dropped with the scope before it ran.
        if let Some(trigger) = trigger_ref.read().as_ref() {
            crate::focus_mounted(trigger);
        }
    });

    let user_close_auto_focus = props.on_close_auto_focus;
    let on_close_auto_focus = use_callback(move |event: AutoFocusEvent| {
        // Both branches run the consumer's handler first and honour its
        // `prevent_default()`, which is what `composeEventHandlers` does.
        user_close_auto_focus.call(event.clone());
        if event.is_default_prevented() {
            return;
        }
        if is_modal {
            // Upstream: preventDefault, then focus the trigger unless the
            // dismissal came from a right-click (whose context menu should
            // keep focus where the user put it).
            event.prevent_default();
            if !is_right_click_outside() {
                focus_trigger.call(());
            }
        } else {
            // Upstream: focus the trigger only when the close was not caused by
            // interacting elsewhere, then always prevent the scope's own
            // restore so we either focused manually or leave it to the browser.
            if !has_interacted_outside() {
                focus_trigger.call(());
            }
            event.prevent_default();
            has_interacted_outside.set(false);
            has_pointer_down_outside.set(false);
        }
    });

    let user_pointer_down_outside = props.on_pointer_down_outside;
    let on_pointer_down_outside = use_callback(move |event: DismissableEvent| {
        // Upstream composes with `checkForDefaultPrevented: false`: the flag is
        // recorded even if the consumer prevented dismissal.
        user_pointer_down_outside.call(event.clone());
        if is_modal {
            is_right_click_outside.set(event.is_right_click());
        }
    });

    let user_focus_outside = props.on_focus_outside;
    let on_focus_outside = use_callback(move |event: DismissableEvent| {
        user_focus_outside.call(event.clone());
        if is_modal {
            // While focus is trapped a `focusout` can still fire; upstream
            // prevents it unconditionally so it never dismisses.
            event.prevent_default();
        }
    });

    let user_interact_outside = props.on_interact_outside;
    let on_interact_outside = use_callback(move |event: DismissableEvent| {
        user_interact_outside.call(event.clone());
        if is_modal {
            return;
        }
        if !event.is_default_prevented() {
            has_interacted_outside.set(true);
            if event.source() == DismissableSource::PointerDown {
                has_pointer_down_outside.set(true);
            }
        }

        // Never dismiss from a click on the trigger: the trigger already
        // toggles, so dismissing here would close and immediately reopen.
        let target_is_trigger = trigger_ref
            .read()
            .as_ref()
            .map(|trigger| event.target_is_within_mounted(trigger))
            .unwrap_or(false);
        if target_is_trigger {
            event.prevent_default();
        }

        // Safari: a trigger inside a `tabindex=0` container produces a pointer
        // -down outside on the trigger followed by a focus outside on the
        // container. Ignore the second when we have already seen the first.
        if event.source() == DismissableSource::FocusIn && has_pointer_down_outside() {
            event.prevent_default();
        }
    });

    let layer = use_dismissable_layer(
        id,
        DismissableLayerOptions {
            disable_outside_pointer_events: is_modal,
            on_escape_key_down: props.on_escape_key_down,
            on_pointer_down_outside,
            on_focus_outside,
            on_interact_outside,
            on_dismiss: use_callback(move |_: ()| set_open.call(false)),
        },
    );

    // The `popover` attribute goes on the positioning wrapper so `showPopover()`
    // lifts the floated element into the top layer with its transform intact.
    // It is driven by `Presence`'s animation-aware `present`, not by `open`:
    // hiding on `open` would set `display: none` before the exit animation
    // could run, which is why `data-[state=closed]:animate-out` never played.
    let presence: PresenceContext = use_context();
    let present = presence.present;
    let mut wrapper_mounted = use_signal(|| None::<Rc<MountedData>>);
    use_top_layer(
        wrapper_mounted.into(),
        present.into(),
        set_open,
        TopLayerKind::PopoverManual,
    );

    let data_state = if open() { "open" } else { "closed" };
    // Upstream: `trapFocus={context.open}` in the modal branch, `false` in the
    // non-modal one — never trap once closed, since closed != unmounted while
    // animating out.
    let trapped = is_modal && open();

    // The dismissable layer must be the styled content box itself. See
    // `use_dismissable_layer`: a nested layer div would sit inside the padding
    // and a pointer-down on that padding would read as an outside interaction.
    let content_attrs = attributes!(div {
        id: id,
        "data-slot": "popover-content",
        "data-state": data_state,
        role: "dialog",
        onpointerdown: move |e: Event<PointerData>| layer.on_pointer_down.call(e),
        onfocusin: move |e: Event<FocusData>| layer.on_focus_in.call(e),
        onfocusout: move |e: Event<FocusData>| layer.on_focus_out.call(e),
    });
    let merged = merge_attributes(vec![content_attrs, props.attributes]);

    let pointer_events_style = layer.pointer_events_style;
    let content_style =
        (!pointer_events_style.is_empty()).then(|| pointer_events_style.to_string());

    rsx! {
        PopperContent {
            side: props.side,
            side_offset: props.side_offset,
            align: props.align,
            align_offset: props.align_offset,
            avoid_collisions: props.avoid_collisions,
            collision_padding: props.collision_padding,
            sticky: props.sticky,
            hide_when_detached: props.hide_when_detached,
            css_var_prefix: "popover",
            class: props.class,
            content_attributes: merged,
            content_style,
            wrapper_attributes: attributes!(div { popover: "manual" }),
            on_wrapper_mounted: move |evt: Event<MountedData>| {
                wrapper_mounted.set(Some(evt.data()));
            },

            FocusScope {
                trapped,
                r#loop: true,
                on_mount_auto_focus: props.on_open_auto_focus,
                on_unmount_auto_focus: on_close_auto_focus,
                {props.children}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// PopoverClose
// ---------------------------------------------------------------------------

/// Props for [`PopoverClose`].
#[derive(Props, Clone, PartialEq)]
pub struct PopoverCloseProps {
    /// Additional classes.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children.
    pub children: Element,
}

/// A button that closes the popover.
///
/// Matches Radix's `PopoverClose`.
#[component]
pub fn PopoverClose(props: PopoverCloseProps) -> Element {
    let ctx: PopoverCtx = use_context();
    let set_open = ctx.set_open;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "popover-close",
            class: props.class,
            onclick: move |_| set_open.call(false),
            ..props.attributes,
            {props.children}
        }
    }
}
