use dioxus::prelude::*;

use crate::theme;
use crate::Route;

#[component]
pub(crate) fn AppLayout() -> Element {
    use_effect(move || {
        theme::theme_seed();
        // Remove the preload class after first render to enable CSS transitions.
        // The preload class (set in index.html) suppresses all transitions during
        // initial DOM creation to prevent flash. Double rAF ensures all CSS and
        // Dioxus-created elements have settled before enabling transitions.
        _ = document::eval(
            "requestAnimationFrame(function(){requestAnimationFrame(function(){document.body.classList.remove('preload')})})",
        );
    });

    rsx! {
        Outlet::<Route> {}
    }
}
