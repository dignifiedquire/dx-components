use dioxus::prelude::*;

use crate::theme;
use crate::Route;

#[component]
pub(crate) fn AppLayout() -> Element {
    use_effect(move || {
        theme::theme_seed();
    });

    rsx! {
        Outlet::<Route> {}
    }
}
