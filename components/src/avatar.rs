//! Styled avatar matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::avatar` primitives with
//! Tailwind classes — matching the shadcn/ui avatar component 1:1.
//!
//! Also adds three HTML-only components not in the primitive:
//! [`AvatarBadge`], [`AvatarGroup`], and [`AvatarGroupCount`].

use dioxus::prelude::*;
use dioxus_primitives::avatar as primitives;
pub use dioxus_primitives::avatar::ImageLoadingStatus;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// AvatarSize
// ---------------------------------------------------------------------------

/// Size variants for the styled avatar.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AvatarSize {
    /// Small avatar (24px).
    Sm,
    /// Default avatar (32px).
    #[default]
    Default,
    /// Large avatar (40px).
    Lg,
}

impl AvatarSize {
    fn as_data_attr(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Default => "default",
            Self::Lg => "lg",
        }
    }
}

// ---------------------------------------------------------------------------
// Avatar (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`Avatar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// Size variant.
    #[props(default)]
    pub size: AvatarSize,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (typically [`AvatarImage`] and [`AvatarFallback`]).
    pub children: Element,
}

/// Styled Avatar root — matches shadcn exactly.
#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let class = tw_merge!(
        "group/avatar relative flex size-8 shrink-0 overflow-hidden rounded-full select-none data-[size=lg]:size-10 data-[size=sm]:size-6",
        props.class,
    );

    rsx! {
        primitives::Avatar {
            class: class,
            "data-size": props.size.as_data_attr(),
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarImage (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`AvatarImage`] component.
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

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the image element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Styled AvatarImage — matches shadcn exactly.
#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    let class = tw_merge!("aspect-square size-full", props.class);

    rsx! {
        primitives::AvatarImage {
            src: props.src,
            alt: props.alt,
            on_loading_status_change: props.on_loading_status_change,
            class: class,
            attributes: props.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarFallback (styled)
// ---------------------------------------------------------------------------

/// The props for the styled [`AvatarFallback`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AvatarFallbackProps {
    /// Delay in milliseconds before showing the fallback.
    #[props(default)]
    pub delay_ms: Option<u64>,

    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the fallback element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (typically text initials or an icon).
    pub children: Element,
}

/// Styled AvatarFallback — matches shadcn exactly.
#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    let class = tw_merge!(
        "flex size-full items-center justify-center rounded-full bg-muted text-sm text-muted-foreground group-data-[size=sm]/avatar:text-xs",
        props.class,
    );

    rsx! {
        primitives::AvatarFallback {
            delay_ms: props.delay_ms,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarBadge (HTML-only, not in primitive)
// ---------------------------------------------------------------------------

/// The props for the styled [`AvatarBadge`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AvatarBadgeProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the badge element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (typically an icon).
    #[props(default)]
    pub children: Element,
}

/// Badge overlay positioned at the bottom-right of the avatar.
#[component]
pub fn AvatarBadge(props: AvatarBadgeProps) -> Element {
    let class = tw_merge!(
        "absolute right-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full bg-primary text-primary-foreground ring-2 ring-background select-none group-data-[size=sm]/avatar:size-2 group-data-[size=sm]/avatar:[&>svg]:hidden group-data-[size=default]/avatar:size-2.5 group-data-[size=default]/avatar:[&>svg]:size-2 group-data-[size=lg]/avatar:size-3 group-data-[size=lg]/avatar:[&>svg]:size-2",
        props.class,
    );

    rsx! {
        span {
            "data-slot": "avatar-badge",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarGroup (HTML-only, not in primitive)
// ---------------------------------------------------------------------------

/// The props for the styled [`AvatarGroup`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AvatarGroupProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the group element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (typically multiple [`Avatar`] components).
    pub children: Element,
}

/// Container for grouped avatars with overlapping layout.
#[component]
pub fn AvatarGroup(props: AvatarGroupProps) -> Element {
    let class = tw_merge!(
        "group/avatar-group flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background",
        props.class,
    );

    rsx! {
        div {
            "data-slot": "avatar-group",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarGroupCount (HTML-only, not in primitive)
// ---------------------------------------------------------------------------

/// The props for the styled [`AvatarGroupCount`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AvatarGroupCountProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the count element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (typically a count like "+3").
    pub children: Element,
}

/// Count badge for avatar groups showing overflow count.
#[component]
pub fn AvatarGroupCount(props: AvatarGroupCountProps) -> Element {
    let class = tw_merge!(
        "relative flex size-8 shrink-0 items-center justify-center rounded-full bg-muted text-sm text-muted-foreground ring-2 ring-background group-has-data-[size=lg]/avatar-group:size-10 group-has-data-[size=sm]/avatar-group:size-6 [&>svg]:size-4 group-has-data-[size=lg]/avatar-group:[&>svg]:size-5 group-has-data-[size=sm]/avatar-group:[&>svg]:size-3",
        props.class,
    );

    rsx! {
        span {
            "data-slot": "avatar-group-count",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}
