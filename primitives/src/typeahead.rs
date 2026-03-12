//! Typeahead search for menus — matches Radix menu typeahead behavior.
//!
//! Provides case-insensitive prefix matching with auto-clear timeout.
//! Simpler than Select's Levenshtein matching (matching Radix menu behavior
//! where menus use basic prefix search, not fuzzy matching).

use std::time::Duration;

use dioxus::prelude::*;

/// An item that can be matched by typeahead.
pub(crate) struct TypeaheadItem {
    /// The text to match against.
    pub text: String,
    /// Index into the focus/roving focus system.
    pub index: usize,
}

/// State for typeahead search within a menu.
#[derive(Clone, Copy)]
pub(crate) struct TypeaheadState {
    buffer: Signal<String>,
    clear_generation: Signal<u64>,
    timeout_ms: u64,
}

impl TypeaheadState {
    /// Feed a character into the typeahead buffer and search for a match.
    ///
    /// Returns the index of the matched item (if any). The buffer auto-clears
    /// after `timeout_ms` of inactivity.
    pub fn search(&mut self, ch: char, items: &[TypeaheadItem]) -> Option<usize> {
        let mut buf = self.buffer.write();
        buf.push(ch);
        let query = buf.clone();
        drop(buf);

        // Cancel previous clear timer via generation counter
        let gen = *self.clear_generation.peek() + 1;
        self.clear_generation.set(gen);

        // Schedule auto-clear
        let mut buffer = self.buffer;
        let clear_generation = self.clear_generation;
        let timeout = self.timeout_ms;
        spawn(async move {
            dioxus_sdk_time::sleep(Duration::from_millis(timeout)).await;
            if *clear_generation.peek() == gen {
                buffer.write().clear();
            }
        });

        // Case-insensitive prefix match
        let query_lower = query.to_lowercase();
        items
            .iter()
            .find(|item| item.text.to_lowercase().starts_with(&query_lower))
            .map(|item| item.index)
    }

    /// Clear the typeahead buffer.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.buffer.write().clear();
    }
}

/// Create typeahead search state.
pub(crate) fn use_typeahead(timeout_ms: u64) -> TypeaheadState {
    let buffer = use_signal(String::new);
    let clear_generation = use_signal(|| 0u64);

    TypeaheadState {
        buffer,
        clear_generation,
        timeout_ms,
    }
}
