//! Port of `@floating-ui/dom/src/autoUpdate.ts` (246 lines).
//!
//! Sets up scroll/resize listeners to trigger position updates.
//! Uses `web-sys` Rust bindings — zero JavaScript strings.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

use super::utils;

/// Options for auto-update behavior.
/// Source: autoUpdate.ts `AutoUpdateOptions` (lines 9-39)
#[derive(Debug, Clone)]
pub struct AutoUpdateOptions {
    /// Update on ancestor scroll. Default: true
    pub ancestor_scroll: bool,
    /// Update on ancestor resize. Default: true
    pub ancestor_resize: bool,
    /// Update on element resize (ResizeObserver). Default: true
    pub element_resize: bool,
    /// Update on layout shift (IntersectionObserver). Default: true
    pub layout_shift: bool,
    /// Update on every animation frame. Default: false
    pub animation_frame: bool,
}

impl Default for AutoUpdateOptions {
    fn default() -> Self {
        Self {
            ancestor_scroll: true,
            ancestor_resize: true,
            element_resize: true,
            layout_shift: true,
            animation_frame: false,
        }
    }
}

/// Set up auto-update listeners for a floating element.
///
/// Returns a cleanup closure that removes all listeners.
///
/// Source: autoUpdate.ts lines 148-246
pub fn auto_update(
    reference: &Element,
    floating: &Element,
    update: impl FnMut() + 'static,
    options: AutoUpdateOptions,
) -> Box<dyn FnOnce()> {
    let update = std::rc::Rc::new(std::cell::RefCell::new(update));

    let mut cleanups: Vec<Box<dyn FnOnce()>> = Vec::new();

    // Collect overflow ancestors for scroll/resize listeners
    let ancestors = if options.ancestor_scroll || options.ancestor_resize {
        let mut anc = utils::get_overflow_ancestors(reference.unchecked_ref());
        anc.extend(utils::get_overflow_ancestors(floating.unchecked_ref()));
        anc
    } else {
        Vec::new()
    };

    // Ancestor scroll listeners
    if options.ancestor_scroll {
        for ancestor in &ancestors {
            let cb = Closure::<dyn FnMut()>::new({
                let update = update.clone();
                move || update.borrow_mut()()
            });
            let opts = web_sys::AddEventListenerOptions::new();
            opts.set_passive(true);
            let _ = ancestor.add_event_listener_with_callback_and_add_event_listener_options(
                "scroll",
                cb.as_ref().unchecked_ref(),
                &opts,
            );
            let ancestor_clone = ancestor.clone();
            cleanups.push(Box::new(move || {
                let _ = ancestor_clone
                    .remove_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref());
            }));
        }
    }

    // Ancestor resize listeners
    if options.ancestor_resize {
        for ancestor in &ancestors {
            let cb = Closure::<dyn FnMut()>::new({
                let update = update.clone();
                move || update.borrow_mut()()
            });
            let _ =
                ancestor.add_event_listener_with_callback("resize", cb.as_ref().unchecked_ref());
            let ancestor_clone = ancestor.clone();
            cleanups.push(Box::new(move || {
                let _ = ancestor_clone
                    .remove_event_listener_with_callback("resize", cb.as_ref().unchecked_ref());
            }));
        }
    }

    // Element resize via ResizeObserver
    // Source: autoUpdate.ts lines 184-210
    // Prevents update loops by temporarily unobserving floating during reference resize
    if options.element_resize {
        let update_for_ro = update.clone();
        let reference_for_ro = reference.clone();
        let floating_for_ro = floating.clone();
        let observer_cell: std::rc::Rc<std::cell::RefCell<Option<web_sys::ResizeObserver>>> =
            std::rc::Rc::new(std::cell::RefCell::new(None));
        let observer_cell_inner = observer_cell.clone();

        let cb = Closure::<dyn FnMut(js_sys::Array)>::new(move |entries: js_sys::Array| {
            // If reference triggered the resize, temporarily unobserve floating
            // to prevent infinite loops with size middleware
            let first_target = entries.get(0);
            if let Ok(entry) = first_target.dyn_into::<web_sys::ResizeObserverEntry>() {
                if entry.target() == reference_for_ro {
                    if let Some(ref obs) = *observer_cell_inner.borrow() {
                        obs.unobserve(&floating_for_ro);
                        // Re-observe after a frame
                        let obs_clone = obs.clone();
                        let floating_clone = floating_for_ro.clone();
                        let reobserve = Closure::<dyn FnMut()>::once(move || {
                            obs_clone.observe(&floating_clone);
                        });
                        let _ = web_sys::window()
                            .unwrap()
                            .request_animation_frame(reobserve.as_ref().unchecked_ref());
                        reobserve.forget();
                    }
                }
            }
            update_for_ro.borrow_mut()();
        });

        let ro = web_sys::ResizeObserver::new(cb.as_ref().unchecked_ref());
        if let Ok(observer) = ro {
            if !options.animation_frame {
                observer.observe(reference);
            }
            observer.observe(floating);
            *observer_cell.borrow_mut() = Some(observer.clone());
            cleanups.push(Box::new(move || {
                observer.disconnect();
                drop(cb);
            }));
        }
    }

    // Animation frame loop
    // Source: autoUpdate.ts lines 215-228
    if options.animation_frame {
        let update_for_raf = update.clone();
        let reference_clone = reference.clone();
        // Shared state: running flag + last frame ID for cancellation
        let frame_id = std::rc::Rc::new(std::cell::Cell::new(0i32));
        let running = std::rc::Rc::new(std::cell::Cell::new(true));
        let frame_id_cleanup = frame_id.clone();
        let running_cleanup = running.clone();

        // Store prev rect as shared state to avoid Closure::forget per frame
        let prev_rect: std::rc::Rc<std::cell::RefCell<Option<[f64; 4]>>> =
            std::rc::Rc::new(std::cell::RefCell::new(None));

        type RafClosure = std::rc::Rc<std::cell::RefCell<Option<Closure<dyn FnMut()>>>>;
        let frame_loop: RafClosure = std::rc::Rc::new(std::cell::RefCell::new(None));
        let frame_loop_clone = frame_loop.clone();

        let cb = Closure::<dyn FnMut()>::new(move || {
            if !running.get() {
                return;
            }
            let rect = reference_clone.get_bounding_client_rect();
            let cur = [rect.x(), rect.y(), rect.width(), rect.height()];
            if let Some(prev) = *prev_rect.borrow() {
                if prev != cur {
                    update_for_raf.borrow_mut()();
                }
            }
            *prev_rect.borrow_mut() = Some(cur);

            // Schedule next frame
            if let Some(ref fl) = *frame_loop_clone.borrow() {
                let fid = web_sys::window()
                    .unwrap()
                    .request_animation_frame(fl.as_ref().unchecked_ref())
                    .unwrap_or(0);
                frame_id.set(fid);
            }
        });

        // Store and kick off
        *frame_loop.borrow_mut() = Some(cb);
        if let Some(ref fl) = *frame_loop.borrow() {
            let fid = web_sys::window()
                .unwrap()
                .request_animation_frame(fl.as_ref().unchecked_ref())
                .unwrap_or(0);
            frame_id_cleanup.set(fid);
        }

        cleanups.push(Box::new(move || {
            running_cleanup.set(false);
            let _ = web_sys::window()
                .unwrap()
                .cancel_animation_frame(frame_id_cleanup.get());
            // Drop the closure to free memory
            drop(frame_loop);
        }));
    }

    // Fire initial update
    update.borrow_mut()();

    Box::new(move || {
        for cleanup in cleanups {
            cleanup();
        }
    })
}
