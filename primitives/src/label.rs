//! Label primitive — matches `@radix-ui/react-label`.
//!
//! Renders a `<label>` element that prevents text selection on double-click
//! (unless clicking inside a button, input, select, or textarea).

use std::rc::Rc;

use dioxus::prelude::*;

/// Props for [`Label`].
#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    /// The id of the element this label is associated with.
    #[props(default)]
    pub html_for: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// An accessible label for form controls.
///
/// Matches Radix's `Label`. Prevents text selection on double-click to avoid
/// accidental selection when rapidly interacting with the associated control.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::label::Label;
/// rsx! {
///     Label { html_for: "name", "Name" }
///     input { id: "name", placeholder: "Enter your name" }
/// };
/// ```
#[component]
pub fn Label(props: LabelProps) -> Element {
    let mut node_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    // Upstream: onMouseDown — prevent text selection on double-click
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;

        crate::use_effect_with_cleanup(move || {
            let mounted = node_ref.read().clone();
            let el = mounted
                .as_ref()
                .and_then(|m| m.downcast::<web_sys::Element>().cloned());

            // Track whether the last mousedown was a multi-click (shared between closures).
            let is_multi_click = Rc::new(std::cell::Cell::new(false));

            let fns: Option<(js_sys::Function, js_sys::Function)> = el.map(|el| {
                // mousedown: set multi-click flag and preventDefault (Chromium/Firefox)
                let mc = is_multi_click.clone();
                let mousedown = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                    // Upstream: if (target.closest('button, input, select, textarea')) return;
                    if let Some(target) = event.target() {
                        if let Ok(target_el) = target.dyn_into::<web_sys::Element>() {
                            if target_el
                                .closest("button, input, select, textarea")
                                .ok()
                                .flatten()
                                .is_some()
                            {
                                mc.set(false);
                                return;
                            }
                        }
                    }
                    let multi = !event.default_prevented() && event.detail() > 1;
                    mc.set(multi);
                    if multi {
                        event.prevent_default();
                    }
                })
                    as Box<dyn FnMut(web_sys::MouseEvent)>);

                // selectstart: catch WebKit which fires selection after mousedown
                let mc2 = is_multi_click.clone();
                let selectstart = Closure::wrap(Box::new(move |event: web_sys::Event| {
                    if mc2.get() {
                        event.prevent_default();
                    }
                })
                    as Box<dyn FnMut(web_sys::Event)>);

                let _ = el.add_event_listener_with_callback(
                    "mousedown",
                    mousedown.as_ref().unchecked_ref(),
                );
                let _ = el.add_event_listener_with_callback(
                    "selectstart",
                    selectstart.as_ref().unchecked_ref(),
                );

                let md_fn = mousedown
                    .as_ref()
                    .unchecked_ref::<js_sys::Function>()
                    .clone();
                let ss_fn = selectstart
                    .as_ref()
                    .unchecked_ref::<js_sys::Function>()
                    .clone();
                mousedown.forget();
                selectstart.forget();
                (md_fn, ss_fn)
            });

            move || {
                if let (Some(m), Some((md_fn, ss_fn))) = (mounted.as_ref(), fns.as_ref()) {
                    if let Some(el) = m.downcast::<web_sys::Element>() {
                        let _ = el.remove_event_listener_with_callback("mousedown", md_fn);
                        let _ = el.remove_event_listener_with_callback("selectstart", ss_fn);
                    }
                }
            }
        });
    }

    rsx! {
        label {
            "data-slot": "label",
            r#for: props.html_for,
            class: props.class,
            onmounted: move |e| node_ref.set(Some(e.data())),
            ..props.attributes,
            {props.children}
        }
    }
}

/// Upstream alias.
///
/// `const Root = Label;`
pub use Label as Root;
