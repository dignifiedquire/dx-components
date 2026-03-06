use dioxus::prelude::*;

use crate::ui::navbar::Navbar;
use crate::Route;

/// Layout for the homepage — navbar + full-width content, no sidebar.
#[component]
pub(crate) fn HomeLayout() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/dx-components-theme.css"),
        }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        Navbar {}
        Outlet::<Route> {}
    }
}
