//! Styled Carousel matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::carousel` primitive with
//! Tailwind classes — matching the shadcn/ui carousel component.

use dioxus::prelude::*;
use dioxus_primitives::carousel as primitives;
use tailwind_fuse::*;

// Re-export context and types
pub use primitives::{CarouselCtx, CarouselOrientation, use_carousel};

// ---------------------------------------------------------------------------
// Carousel (root)
// ---------------------------------------------------------------------------

/// Props for the styled [`Carousel`].
#[derive(Props, Clone, PartialEq)]
pub struct CarouselProps {
    /// Carousel orientation.
    #[props(default)]
    pub orientation: CarouselOrientation,

    /// Total number of slides.
    #[props(default = 1)]
    pub total_slides: usize,

    /// Initial slide index.
    #[props(default)]
    pub initial_index: usize,

    /// Callback when slide changes.
    #[props(default)]
    pub on_slide_change: Callback<usize>,

    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled Carousel root — matches shadcn.
#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let class = tw_merge!("relative", props.class);

    rsx! {
        primitives::Carousel {
            orientation: props.orientation,
            total_slides: props.total_slides,
            initial_index: props.initial_index,
            on_slide_change: props.on_slide_change,
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CarouselContent
// ---------------------------------------------------------------------------

/// Props for the styled [`CarouselContent`].
#[derive(Props, Clone, PartialEq)]
pub struct CarouselContentProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled CarouselContent — matches shadcn.
#[component]
pub fn CarouselContent(props: CarouselContentProps) -> Element {
    let ctx = use_carousel();
    let orient_class = match ctx.orientation {
        CarouselOrientation::Horizontal => "-ml-4",
        CarouselOrientation::Vertical => "-mt-4 flex-col",
    };

    let class = tw_merge!("flex", orient_class, props.class);

    rsx! {
        div {
            "data-slot": "carousel-viewport",
            class: "overflow-hidden",

            primitives::CarouselContent {
                class: class,
                attributes: props.attributes,
                {props.children}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// CarouselItem
// ---------------------------------------------------------------------------

/// Props for the styled [`CarouselItem`].
#[derive(Props, Clone, PartialEq)]
pub struct CarouselItemProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Styled CarouselItem — matches shadcn.
#[component]
pub fn CarouselItem(props: CarouselItemProps) -> Element {
    let ctx = use_carousel();
    let orient_class = match ctx.orientation {
        CarouselOrientation::Horizontal => "pl-4",
        CarouselOrientation::Vertical => "pt-4",
    };

    let class = tw_merge!(
        "min-w-0 shrink-0 grow-0 basis-full",
        orient_class,
        props.class,
    );

    rsx! {
        primitives::CarouselItem {
            class: class,
            attributes: props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CarouselPrevious
// ---------------------------------------------------------------------------

/// Props for the styled [`CarouselPrevious`].
#[derive(Props, Clone, PartialEq)]
pub struct CarouselPreviousProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Custom content.
    #[props(default)]
    pub children: Element,
}

/// Styled CarouselPrevious — matches shadcn.
#[component]
pub fn CarouselPrevious(props: CarouselPreviousProps) -> Element {
    let ctx = use_carousel();
    let orient_class = match ctx.orientation {
        CarouselOrientation::Horizontal => "top-1/2 -left-12 -translate-y-1/2",
        CarouselOrientation::Vertical => "left-1/2 -top-12 -translate-x-1/2 rotate-90",
    };

    let class = tw_merge!("absolute size-8 rounded-full", orient_class, props.class,);

    let has_children = props.children != Ok(VNode::placeholder());

    if has_children {
        rsx! {
            primitives::CarouselPrevious {
                class: class,
                attributes: props.attributes,
                {props.children}
            }
        }
    } else {
        rsx! {
            primitives::CarouselPrevious {
                class: class,
                attributes: props.attributes,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// CarouselNext
// ---------------------------------------------------------------------------

/// Props for the styled [`CarouselNext`].
#[derive(Props, Clone, PartialEq)]
pub struct CarouselNextProps {
    /// Additional Tailwind classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Custom content.
    #[props(default)]
    pub children: Element,
}

/// Styled CarouselNext — matches shadcn.
#[component]
pub fn CarouselNext(props: CarouselNextProps) -> Element {
    let ctx = use_carousel();
    let orient_class = match ctx.orientation {
        CarouselOrientation::Horizontal => "top-1/2 -right-12 -translate-y-1/2",
        CarouselOrientation::Vertical => "left-1/2 -bottom-12 -translate-x-1/2 rotate-90",
    };

    let class = tw_merge!("absolute size-8 rounded-full", orient_class, props.class,);

    let has_children = props.children != Ok(VNode::placeholder());

    if has_children {
        rsx! {
            primitives::CarouselNext {
                class: class,
                attributes: props.attributes,
                {props.children}
            }
        }
    } else {
        rsx! {
            primitives::CarouselNext {
                class: class,
                attributes: props.attributes,
            }
        }
    }
}
