//! Styled Carousel matching shadcn/ui.
//!
//! Wraps the unstyled `dioxus_primitives::carousel` primitive with
//! Tailwind classes — matching the shadcn/ui carousel component.

use dioxus::prelude::*;
use dioxus_primitives::carousel as primitives;
use dx_icons_lucide::{IconChevronLeft, IconChevronRight};
use tailwind_fuse::*;

/// shadcn `Button variant="outline" size="icon"` flattened to a class
/// string (we don't route carousel buttons through the styled `Button`
/// because the primitive renders its own `<button>`). Matches the
/// registry `ui/carousel.tsx` which uses `size="icon"` (`size-8`) +
/// `absolute size-8 rounded-full` — the radix-flavor file uses
/// `icon-sm` (size-7) but that reads too small as a free-floating nav
/// control, so we follow the registry sizing here.
const NAV_BUTTON_CLASS: &str = "inline-flex shrink-0 items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-all cursor-pointer outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:border-input dark:bg-input/30 dark:hover:bg-input/50 size-8 absolute touch-manipulation rounded-full";

// Re-export context and types
pub use primitives::{CarouselApi, CarouselCtx, CarouselOrientation, use_carousel};

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

    /// Slides visible per viewport. Set to match the `basis-1/N` utility on
    /// your `CarouselItem`s (`2` for `basis-1/2`, `3` for `basis-1/3`, …) so
    /// the prev/next boundary and per-step distance are correct.
    #[props(default = 1)]
    pub slides_per_view: usize,

    /// Initial slide index.
    #[props(default)]
    pub initial_index: usize,

    /// Callback when slide changes.
    #[props(default)]
    pub on_slide_change: Callback<usize>,

    /// Called with a [`CarouselApi`] snapshot whenever state changes —
    /// mirrors shadcn's `setApi`.
    #[props(default)]
    pub on_api: Callback<CarouselApi>,

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
            slides_per_view: props.slides_per_view,
            initial_index: props.initial_index,
            on_slide_change: props.on_slide_change,
            on_api: props.on_api,
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

    let class = tw_merge!(
        "flex transition-transform duration-300 ease-in-out",
        orient_class,
        props.class
    );

    // Each step shifts by one slide width = 100 / slides_per_view percent
    // of the flex track, so multi-visible carousels move one item at a
    // time instead of a whole viewport.
    let spv = ctx.slides_per_view.max(1) as f64;
    let offset = ctx.current_index as f64 * -100.0 / spv;
    let transform = match ctx.orientation {
        CarouselOrientation::Horizontal => format!("transform: translateX({offset}%);"),
        CarouselOrientation::Vertical => format!("transform: translateY({offset}%);"),
    };

    rsx! {
        div {
            "data-slot": "carousel-viewport",
            // `overflow-clip`, NOT `overflow-hidden`: `hidden` still leaves
            // the element programmatically/touch-scrollable (it only hides
            // the scrollbar), so a trackpad swipe would scroll past our
            // transform-driven boundary. `clip` fully clips and creates no
            // scroll container. shadcn uses `overflow-hidden` because embla
            // intercepts the drag; we have no embla, so we must hard-clip.
            class: "overflow-clip",

            primitives::CarouselContent {
                class: class,
                style: "{transform}",
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

    let class = tw_merge!(NAV_BUTTON_CLASS, orient_class, props.class);

    let has_children = props.children != Ok(VNode::placeholder());

    rsx! {
        primitives::CarouselPrevious {
            class: class,
            attributes: props.attributes,
            if has_children {
                {props.children}
            } else {
                IconChevronLeft { class: "cn-rtl-flip size-4" }
                span { class: "sr-only", "Previous slide" }
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

    let class = tw_merge!(NAV_BUTTON_CLASS, orient_class, props.class);

    let has_children = props.children != Ok(VNode::placeholder());

    rsx! {
        primitives::CarouselNext {
            class: class,
            attributes: props.attributes,
            if has_children {
                {props.children}
            } else {
                IconChevronRight { class: "cn-rtl-flip size-4" }
                span { class: "sr-only", "Next slide" }
            }
        }
    }
}
