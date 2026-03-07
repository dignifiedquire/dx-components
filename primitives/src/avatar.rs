//! Avatar primitive — matches `@radix-ui/react-avatar`.
//!
//! Displays a user avatar image with fallback support. The image loading
//! state machine drives whether the image or fallback is shown.

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Image loading status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageLoadingStatus {
    /// Initial state, no load attempted.
    Idle,
    /// Image is currently loading.
    Loading,
    /// Image loaded successfully.
    Loaded,
    /// Image failed to load.
    Error,
}

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct AvatarCtx {
    status: Signal<ImageLoadingStatus>,
}

// ---------------------------------------------------------------------------
// Avatar
// ---------------------------------------------------------------------------

/// Props for [`Avatar`].
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (typically [`AvatarImage`] and [`AvatarFallback`]).
    pub children: Element,
}

/// Root container for an avatar with image + fallback support.
///
/// Matches Radix's `Avatar`. Provides context for image loading status
/// so [`AvatarImage`] and [`AvatarFallback`] coordinate display.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::avatar::{Avatar, AvatarImage, AvatarFallback};
/// rsx! {
///     Avatar {
///         AvatarImage { src: "https://example.com/avatar.jpg", alt: "User" }
///         AvatarFallback { "JD" }
///     }
/// };
/// ```
#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let status = use_signal(|| ImageLoadingStatus::Idle);
    use_context_provider(|| AvatarCtx { status });

    rsx! {
        span {
            "data-slot": "avatar",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarImage
// ---------------------------------------------------------------------------

/// Props for [`AvatarImage`].
#[derive(Props, Clone, PartialEq)]
pub struct AvatarImageProps {
    /// The image source URL.
    pub src: String,

    /// Alt text for the image.
    #[props(default)]
    pub alt: Option<String>,

    /// Callback when the loading status changes.
    #[props(default)]
    pub on_loading_status_change: Option<Callback<ImageLoadingStatus>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The avatar image. Only renders when the image has loaded successfully.
///
/// Must be used inside an [`Avatar`] component.
#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    let mut ctx: AvatarCtx = use_context();

    // Set loading state when src is provided
    use_effect(move || {
        ctx.status.set(ImageLoadingStatus::Loading);
        if let Some(cb) = &props.on_loading_status_change {
            cb.call(ImageLoadingStatus::Loading);
        }
    });

    let status = (ctx.status)();

    if status != ImageLoadingStatus::Loaded {
        // Still loading — render the img so onload/onerror can fire,
        // but only if we haven't errored
        if status == ImageLoadingStatus::Error {
            return rsx!({});
        }

        return rsx! {
            img {
                "data-slot": "avatar-image",
                src: props.src.clone(),
                alt: props.alt.clone().unwrap_or_default(),
                class: props.class.clone(),
                style: "display: none;",
                onload: move |_| {
                    ctx.status.set(ImageLoadingStatus::Loaded);
                    if let Some(cb) = &props.on_loading_status_change {
                        cb.call(ImageLoadingStatus::Loaded);
                    }
                },
                onerror: move |_| {
                    ctx.status.set(ImageLoadingStatus::Error);
                    if let Some(cb) = &props.on_loading_status_change {
                        cb.call(ImageLoadingStatus::Error);
                    }
                },
                ..props.attributes,
            }
        };
    }

    rsx! {
        img {
            "data-slot": "avatar-image",
            src: props.src.clone(),
            alt: props.alt.clone().unwrap_or_default(),
            class: props.class,
            ..props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarFallback
// ---------------------------------------------------------------------------

/// Props for [`AvatarFallback`].
#[derive(Props, Clone, PartialEq)]
pub struct AvatarFallbackProps {
    /// Delay in milliseconds before showing the fallback. Useful to avoid
    /// a flash of fallback content for fast-loading images.
    #[props(default)]
    pub delay_ms: Option<u64>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (typically text initials or an icon).
    pub children: Element,
}

/// Fallback content shown when the avatar image hasn't loaded.
///
/// Matches Radix's `AvatarFallback`. Supports an optional `delay_ms` to
/// avoid flashing the fallback for images that load quickly.
///
/// Must be used inside an [`Avatar`] component.
#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    let ctx: AvatarCtx = use_context();
    let mut can_render = use_signal(|| props.delay_ms.is_none());

    // Handle delayed rendering
    use_effect(move || {
        if let Some(delay) = props.delay_ms {
            spawn(async move {
                dioxus_sdk_time::sleep(std::time::Duration::from_millis(delay)).await;
                can_render.set(true);
            });
        }
    });

    let status = (ctx.status)();
    if !can_render() || status == ImageLoadingStatus::Loaded {
        return rsx!({});
    }

    rsx! {
        span {
            "data-slot": "avatar-fallback",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}
