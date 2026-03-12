//! Scroll lock utility — prevents body scrolling when modal dialogs are open.
//!
//! Radix uses `react-remove-scroll` (which sets `overflow: hidden` on `<body>`).
//! We use `document::eval` because Dioxus has no API for modifying `<body>` styles.
//!
//! A global counter ensures nested modals work correctly: only the first lock
//! sets `overflow: hidden`, and only the last unlock restores the original value.

use dioxus::prelude::*;

/// Prevents body scrolling while `active` is true.
///
/// Uses a global counter so nested modals work correctly. Only the first
/// lock modifies the body style, and only the last unlock restores it.
///
/// ## Radix deviation
/// Radix uses `react-remove-scroll` which handles iOS momentum scroll,
/// scroll bar gap compensation, and other edge cases. This implementation
/// only sets `overflow: hidden` on `<body>`, which covers the primary use
/// case. `document::eval` is required because Dioxus has no API for
/// modifying `<body>` element styles.
pub(crate) fn use_scroll_lock(active: Memo<bool>) {
    use_effect(move || {
        if active() {
            // Increment global counter and lock if first
            document::eval(
                r#"
                if (!window.__dxScrollLockCount) {
                    window.__dxScrollLockCount = 0;
                    window.__dxScrollLockOrigOverflow = document.body.style.overflow;
                }
                window.__dxScrollLockCount++;
                if (window.__dxScrollLockCount === 1) {
                    document.body.style.overflow = 'hidden';
                }
                "#,
            );
        }
    });

    // Cleanup: decrement counter and restore if last
    crate::use_effect_cleanup(move || {
        if active.peek().clone() {
            document::eval(
                r#"
                if (window.__dxScrollLockCount) {
                    window.__dxScrollLockCount--;
                    if (window.__dxScrollLockCount <= 0) {
                        document.body.style.overflow = window.__dxScrollLockOrigOverflow || '';
                        delete window.__dxScrollLockCount;
                        delete window.__dxScrollLockOrigOverflow;
                    }
                }
                "#,
            );
        }
    });
}
