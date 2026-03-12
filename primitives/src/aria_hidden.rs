//! aria-hidden utility — hides sibling elements from assistive technology
//! when a modal dialog is open.
//!
//! Radix uses the `aria-hidden` npm package (`hideOthers`) to set
//! `aria-hidden="true"` on all siblings of the modal content.
//!
//! We use `document::eval` because Dioxus has no API for querying or
//! modifying sibling DOM elements outside the component tree.

use dioxus::prelude::*;

/// Hides all sibling elements of the element with the given `id` from
/// assistive technology by setting `aria-hidden="true"` on them.
///
/// Uses a `data-dxc-aria-hidden` marker attribute to track modified
/// elements and restore them on cleanup.
///
/// ## Radix deviation
/// Radix uses the `aria-hidden` npm package which handles multiple
/// concurrent hide requests with reference counting. This implementation
/// uses a simpler marker-attribute approach: each call tags elements with
/// `data-dxc-aria-hidden="<id>"` and only restores elements with the
/// matching id. `document::eval` is required because Dioxus has no API
/// for querying or modifying arbitrary sibling DOM elements.
pub(crate) fn use_aria_hidden(id: Memo<String>, active: Memo<bool>) {
    use_effect(move || {
        if active() {
            let element_id = id();
            let js = format!(
                r#"
                (function() {{
                    var el = document.getElementById('{element_id}');
                    if (!el) return;
                    // Walk up to find the portal container or body
                    var container = el.parentElement;
                    if (!container) return;
                    var siblings = container.children;
                    for (var i = 0; i < siblings.length; i++) {{
                        var sib = siblings[i];
                        if (sib === el) continue;
                        if (sib.getAttribute('aria-hidden')) continue;
                        sib.setAttribute('aria-hidden', 'true');
                        sib.setAttribute('data-dxc-aria-hidden', '{element_id}');
                    }}
                }})();
                "#
            );
            document::eval(&js);
        }
    });

    // Cleanup: restore aria-hidden on elements we modified
    crate::use_effect_cleanup(move || {
        if *active.peek() {
            let element_id = id.peek().clone();
            let js = format!(
                r#"
                (function() {{
                    var els = document.querySelectorAll('[data-dxc-aria-hidden="{element_id}"]');
                    for (var i = 0; i < els.length; i++) {{
                        els[i].removeAttribute('aria-hidden');
                        els[i].removeAttribute('data-dxc-aria-hidden');
                    }}
                }})();
                "#
            );
            document::eval(&js);
        }
    });
}
