//! Styled navbar matching shadcn/ui menubar patterns.
//!
//! Wraps the unstyled `dioxus_primitives::navbar` primitive and includes
//! a CSS stylesheet that targets `data-slot` attributes.

use dioxus::prelude::*;
use dioxus_primitives::navbar as primitives;

pub use dioxus_primitives::navbar::{NavbarContent, NavbarItem, NavbarNav, NavbarTrigger};

/// The props for the styled [`Navbar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct NavbarProps {
    /// Whether the navbar is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether focus should loop around when reaching the end.
    #[props(default = true)]
    pub roving_loop: bool,

    /// Attributes to extend the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the navbar.
    pub children: Element,
}

/// Styled Navbar — includes CSS for data-slot styling.
#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./navbar.css") }
        primitives::Navbar {
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}
