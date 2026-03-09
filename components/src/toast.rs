//! Styled toast matching shadcn/ui sonner.
//!
//! Wraps `dioxus_primitives::toast` with Tailwind classes on the viewport and
//! individual toast elements.  Inner elements (title, description, close) are
//! styled via `[data-slot=…]` selectors on the parent toast.

use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use dioxus_primitives::toast as primitives;
use tailwind_fuse::*;

// Re-export everything consumers still need unchanged.
pub use primitives::{
    consume_toast, use_toast, ToastOptions, ToastProps, ToastPropsWithOwner, ToastType, Toasts,
};

// ---------------------------------------------------------------------------
// Toast (styled wrapper)
// ---------------------------------------------------------------------------

fn toast_type_class(t: primitives::ToastType) -> &'static str {
    match t {
        primitives::ToastType::Success => {
            "border-emerald-600 bg-emerald-600 text-white [&_[data-slot=toast-close]]:text-white"
        }
        primitives::ToastType::Error => {
            "border-destructive bg-destructive text-destructive-foreground [&_[data-slot=toast-close]]:text-destructive-foreground"
        }
        primitives::ToastType::Warning => {
            "border-amber-500 bg-amber-500 text-white [&_[data-slot=toast-close]]:text-white"
        }
        primitives::ToastType::Info => {
            "border-border bg-muted text-muted-foreground [&_[data-slot=toast-close]]:text-muted-foreground"
        }
    }
}

const TOAST_BASE: &str = "group pointer-events-auto relative flex w-full items-center justify-between gap-4 overflow-hidden rounded-md border p-4 shadow-lg transition-all [&_[data-slot=toast-content]]:flex [&_[data-slot=toast-content]]:flex-1 [&_[data-slot=toast-content]]:flex-col [&_[data-slot=toast-content]]:gap-1 [&_[data-slot=toast-title]]:text-sm [&_[data-slot=toast-title]]:font-semibold [&_[data-slot=toast-description]]:text-sm [&_[data-slot=toast-description]]:opacity-90 [&_[data-slot=toast-close]]:absolute [&_[data-slot=toast-close]]:top-2 [&_[data-slot=toast-close]]:right-2 [&_[data-slot=toast-close]]:rounded-md [&_[data-slot=toast-close]]:p-1 [&_[data-slot=toast-close]]:border-none [&_[data-slot=toast-close]]:bg-transparent [&_[data-slot=toast-close]]:opacity-0 [&_[data-slot=toast-close]]:transition-opacity [&_[data-slot=toast-close]]:cursor-pointer group-hover:[&_[data-slot=toast-close]]:opacity-70 hover:[&_[data-slot=toast-close]]:!opacity-100 focus:[&_[data-slot=toast-close]]:opacity-100 focus:[&_[data-slot=toast-close]]:outline-none";

/// A styled toast notification.
///
/// Drop-in replacement for `dioxus_primitives::toast::Toast` with shadcn/ui
/// Tailwind classes applied to the root element and inner sub-elements via
/// descendant selectors.
#[component]
pub fn Toast(props: ToastProps) -> Element {
    let class = tw_merge!(TOAST_BASE, toast_type_class(props.toast_type));

    rsx! {
        primitives::Toast {
            id: props.id,
            index: props.index,
            title: props.title,
            description: props.description,
            toast_type: props.toast_type,
            on_close: props.on_close,
            permanent: props.permanent,
            duration: props.duration,
            class: class,
        }
    }
}

// ---------------------------------------------------------------------------
// ToastProvider (styled wrapper)
// ---------------------------------------------------------------------------

const TOAST_VIEWPORT: &str = "fixed top-0 z-[100] flex max-h-screen w-full flex-col-reverse p-4 sm:bottom-0 sm:right-0 sm:top-auto sm:flex-col md:max-w-[420px]";

/// Props for the styled [`ToastProvider`].
#[derive(Props, Clone, PartialEq)]
pub struct ToastProviderProps {
    /// The default duration for non-permanent toasts. Defaults to 5 seconds.
    #[props(default = Some(std::time::Duration::from_secs(5)))]
    pub default_duration: Option<std::time::Duration>,

    /// The maximum number of toasts to display at once. Defaults to 10.
    #[props(default = 10)]
    pub max_toasts: usize,

    /// Additional class names for the viewport.
    #[props(default)]
    pub class: Option<String>,

    /// The children of the toast provider component.
    pub children: Element,
}

/// A styled toast provider that applies shadcn/ui Tailwind classes to the
/// viewport and renders styled toast notifications.
#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    let viewport_class = tw_merge!(TOAST_VIEWPORT, props.class);

    rsx! {
        primitives::ToastProvider {
            default_duration: props.default_duration,
            max_toasts: props.max_toasts,
            render_toast: Callback::new(|p: ToastPropsWithOwner| {
                rsx! { {DynamicNode::Component(p.into_vcomponent(Toast))} }
            }),
            class: viewport_class,
            // The list inside uses its own default styles via data-slot selectors
            // applied from the TOAST_BASE constant.
            {props.children}
        }
    }
}
