//! `aria-hidden` utility — hides everything outside a modal overlay from
//! assistive technology.
//!
//! Port of the `aria-hidden` npm package's `hideOthers(element)`, which Radix
//! calls from `DialogContent` and the modal branch of `PopoverContent`. It is
//! the better-supported equivalent of `aria-modal="true"`: rather than
//! annotating the overlay, it marks every element that is *not* an ancestor or
//! descendant of the overlay with `aria-hidden="true"`, so screen readers
//! cannot reach the page behind it.
//!
//! ## Differences from upstream
//!
//! - **`document::eval` instead of DOM walking in Rust**: Dioxus exposes no API
//!   for querying or mutating elements outside the component tree, so the walk
//!   runs as a small script. The same reason [`crate::scroll_lock`] uses eval.
//! - **Marker attribute instead of a JS ref-count**: the npm package keeps a
//!   counter per element in module state. We tag each element we hide with
//!   `data-dxc-aria-hidden="<overlay id>"` and only unhide elements carrying
//!   our own id, which keeps nested overlays from unhiding each other's
//!   elements. An element already hidden by the application (its own
//!   `aria-hidden`, no marker) is left untouched on cleanup.

use dioxus::prelude::*;

/// Hide everything outside the element with the given `id` from assistive
/// technology while `active` is true.
///
/// Walks from the element up to `<body>`, setting `aria-hidden="true"` on every
/// sibling along the way — so ancestors and descendants of the overlay stay
/// visible to AT and nothing else does. Restores the previous state on cleanup.
pub(crate) fn use_aria_hidden(id: Memo<String>, active: Memo<bool>) {
    use_effect(move || {
        let element_id = id();
        if active() {
            let js = format!(
                r#"
                (function() {{
                    var el = document.getElementById('{element_id}');
                    if (!el) return;
                    var marker = 'data-dxc-aria-hidden';
                    // Walk up to <body>, hiding siblings at every level. This is
                    // `hideOthers(el)`: everything that is neither an ancestor
                    // nor a descendant of `el` becomes aria-hidden.
                    var node = el;
                    while (node && node.parentElement) {{
                        var parent = node.parentElement;
                        var children = parent.children;
                        for (var i = 0; i < children.length; i++) {{
                            var sib = children[i];
                            if (sib === node) continue;
                            if (sib.hasAttribute(marker)) continue;
                            // Leave alone what the app already hid itself.
                            if (sib.getAttribute('aria-hidden') === 'true') continue;
                            // Never hide live regions or scripts.
                            if (sib.tagName === 'SCRIPT' || sib.tagName === 'STYLE') continue;
                            sib.setAttribute('aria-hidden', 'true');
                            sib.setAttribute(marker, '{element_id}');
                        }}
                        node = parent;
                        if (parent === document.body) break;
                    }}
                }})()
                "#
            );
            _ = document::eval(&js);
        } else {
            _ = document::eval(&unhide_script(&element_id));
        }
    });

    use_drop(move || {
        // `id.peek()` — running inside drop, outside a reactive context.
        _ = document::eval(&unhide_script(&id.peek().clone()));
    });
}

/// Undo exactly the elements this overlay hid, identified by the marker.
fn unhide_script(element_id: &str) -> String {
    format!(
        r#"
        (function() {{
            var marker = 'data-dxc-aria-hidden';
            var hidden = document.querySelectorAll('[' + marker + '="{element_id}"]');
            for (var i = 0; i < hidden.length; i++) {{
                hidden[i].removeAttribute('aria-hidden');
                hidden[i].removeAttribute(marker);
            }}
        }})()
        "#
    )
}
