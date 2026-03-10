//! Carousel primitive — slide-based content viewer.
//!
//! Provides a composable carousel with prev/next navigation,
//! keyboard controls, and ARIA attributes.
//!
//! ## Architecture
//!
//! - [`Carousel`] — Root component with orientation and state management
//! - [`CarouselContent`] — Scrollable container for slides
//! - [`CarouselItem`] — Individual slide
//! - [`CarouselPrevious`] — Navigate to previous slide
//! - [`CarouselNext`] — Navigate to next slide
//!
//! ## Example
//!
//! ```rust,no_run
//! # use dioxus::prelude::*;
//! # use dioxus_primitives::carousel::*;
//! fn Demo() -> Element {
//!     rsx! {
//!         Carousel {
//!             CarouselContent {
//!                 CarouselItem { "Slide 1" }
//!                 CarouselItem { "Slide 2" }
//!                 CarouselItem { "Slide 3" }
//!             }
//!             CarouselPrevious {}
//!             CarouselNext {}
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Carousel orientation.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CarouselOrientation {
    /// Horizontal scrolling (default).
    #[default]
    Horizontal,
    /// Vertical scrolling.
    Vertical,
}

impl CarouselOrientation {
    /// Returns the orientation as a string for data attributes.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

/// Context shared by Carousel sub-components.
#[derive(Clone, Debug, PartialEq)]
pub struct CarouselCtx {
    /// Carousel orientation.
    pub orientation: CarouselOrientation,
    /// Currently active slide index.
    pub current_index: usize,
    /// Total number of slides.
    pub total_slides: usize,
    /// Whether can scroll to previous.
    pub can_scroll_prev: bool,
    /// Whether can scroll to next.
    pub can_scroll_next: bool,
}

/// Access the nearest [`Carousel`] context.
pub fn use_carousel() -> CarouselCtx {
    use_context::<Signal<CarouselCtx>>().cloned()
}

// ---------------------------------------------------------------------------
// Carousel (root)
// ---------------------------------------------------------------------------

/// Props for [`Carousel`].
#[derive(Props, Clone, PartialEq)]
pub struct CarouselProps {
    /// Carousel orientation.
    #[props(default)]
    pub orientation: CarouselOrientation,

    /// Total number of slides (used for navigation state).
    #[props(default = 1)]
    pub total_slides: usize,

    /// Initial slide index.
    #[props(default)]
    pub initial_index: usize,

    /// Callback when the current slide changes.
    #[props(default)]
    pub on_slide_change: Callback<usize>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Root carousel component.
#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let mut current_index = use_signal(|| props.initial_index);

    let total = props.total_slides;
    let idx = current_index();

    let ctx = CarouselCtx {
        orientation: props.orientation,
        current_index: idx,
        total_slides: total,
        can_scroll_prev: idx > 0,
        can_scroll_next: idx + 1 < total,
    };

    use_context_provider(|| Signal::new(ctx.clone()));
    use_context_provider(|| current_index);
    use_context_provider(|| props.on_slide_change);

    // Update context when state changes
    let mut ctx_signal = use_context::<Signal<CarouselCtx>>();
    if *ctx_signal.peek() != ctx {
        ctx_signal.set(ctx);
    }

    let orientation = props.orientation;

    rsx! {
        div {
            "data-slot": "carousel",
            "data-orientation": orientation.as_str(),
            role: "region",
            aria_roledescription: "carousel",
            class: props.class,
            onkeydown: move |e: KeyboardEvent| {
                match (orientation, e.key()) {
                    (CarouselOrientation::Horizontal, Key::ArrowLeft)
                    | (CarouselOrientation::Vertical, Key::ArrowUp) => {
                        let idx = current_index();
                        if idx > 0 {
                            current_index.set(idx - 1);
                            props.on_slide_change.call(idx - 1);
                        }
                    }
                    (CarouselOrientation::Horizontal, Key::ArrowRight)
                    | (CarouselOrientation::Vertical, Key::ArrowDown) => {
                        let idx = current_index();
                        if idx + 1 < total {
                            current_index.set(idx + 1);
                            props.on_slide_change.call(idx + 1);
                        }
                    }
                    _ => {}
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CarouselContent
// ---------------------------------------------------------------------------

/// Props for [`CarouselContent`].
#[derive(Props, Clone, PartialEq)]
pub struct CarouselContentProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (CarouselItems).
    pub children: Element,
}

/// Scrollable container for carousel items.
#[component]
pub fn CarouselContent(props: CarouselContentProps) -> Element {
    let ctx = use_context::<Signal<CarouselCtx>>();
    let orientation = ctx.read().orientation;

    rsx! {
        div {
            "data-slot": "carousel-content",
            "data-orientation": orientation.as_str(),
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CarouselItem
// ---------------------------------------------------------------------------

/// Props for [`CarouselItem`].
#[derive(Props, Clone, PartialEq)]
pub struct CarouselItemProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// Individual carousel slide.
#[component]
pub fn CarouselItem(props: CarouselItemProps) -> Element {
    rsx! {
        div {
            "data-slot": "carousel-item",
            role: "group",
            aria_roledescription: "slide",
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// CarouselPrevious
// ---------------------------------------------------------------------------

/// Props for [`CarouselPrevious`].
#[derive(Props, Clone, PartialEq)]
pub struct CarouselPreviousProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Custom content. Defaults to "Previous".
    #[props(default)]
    pub children: Element,
}

/// Button to navigate to the previous slide.
#[component]
pub fn CarouselPrevious(props: CarouselPreviousProps) -> Element {
    let ctx = use_context::<Signal<CarouselCtx>>();
    let mut current_index = use_context::<Signal<usize>>();
    let on_slide_change = use_context::<Callback<usize>>();

    let can_scroll = ctx.read().can_scroll_prev;
    let has_children = props.children != Ok(VNode::placeholder());

    rsx! {
        button {
            "data-slot": "carousel-previous",
            r#type: "button",
            disabled: !can_scroll,
            aria_label: "Previous slide",
            class: props.class,
            onclick: move |_| {
                let idx = current_index();
                if idx > 0 {
                    current_index.set(idx - 1);
                    on_slide_change.call(idx - 1);
                }
            },
            ..props.attributes,
            if has_children {
                {props.children}
            } else {
                "Previous"
            }
        }
    }
}

// ---------------------------------------------------------------------------
// CarouselNext
// ---------------------------------------------------------------------------

/// Props for [`CarouselNext`].
#[derive(Props, Clone, PartialEq)]
pub struct CarouselNextProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Custom content. Defaults to "Next".
    #[props(default)]
    pub children: Element,
}

/// Button to navigate to the next slide.
#[component]
pub fn CarouselNext(props: CarouselNextProps) -> Element {
    let ctx = use_context::<Signal<CarouselCtx>>();
    let mut current_index = use_context::<Signal<usize>>();
    let on_slide_change = use_context::<Callback<usize>>();

    let can_scroll = ctx.read().can_scroll_next;
    let total = ctx.read().total_slides;
    let has_children = props.children != Ok(VNode::placeholder());

    rsx! {
        button {
            "data-slot": "carousel-next",
            r#type: "button",
            disabled: !can_scroll,
            aria_label: "Next slide",
            class: props.class,
            onclick: move |_| {
                let idx = current_index();
                if idx + 1 < total {
                    current_index.set(idx + 1);
                    on_slide_change.call(idx + 1);
                }
            },
            ..props.attributes,
            if has_children {
                {props.children}
            } else {
                "Next"
            }
        }
    }
}
