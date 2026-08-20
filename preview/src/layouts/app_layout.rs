use dioxus::prelude::*;

use crate::theme;
use crate::Route;

/// Remove the preload class after first render to enable CSS transitions.
///
/// The preload class (set in index.html) suppresses all transitions during
/// initial DOM creation to prevent flash. Double rAF ensures all CSS and
/// Dioxus-created elements have settled before enabling transitions.
///
/// Every route needs this: block demo routes render outside [`AppLayout`],
/// so without their own call they keep transitions disabled forever.
pub(crate) fn use_preload_release() {
    use_effect(move || {
        _ = document::eval(
            "requestAnimationFrame(function(){requestAnimationFrame(function(){document.body.classList.remove('preload')})})",
        );
    });
}

#[component]
pub(crate) fn AppLayout() -> Element {
    use_preload_release();
    use_effect(move || {
        theme::theme_seed();
    });

    rsx! {
        Outlet::<Route> {}
    }
}
