//! Line-by-line port of `@radix-ui/react-use-effect-event`.
//!
//! Upstream provides a shim for React's experimental `useEffectEvent` hook.
//! It keeps a ref to the latest callback so the returned function is
//! referentially stable across renders:
//!
//! ```js
//! export function useEffectEvent<T extends AnyFunction>(callback?: T): T {
//!   const ref = React.useRef(...);
//!   useInsertionEffect(() => { ref.current = callback; });
//!   return React.useMemo(() => ((...args) => ref.current?.(...args)) as T, []);
//! }
//! ```
//!
//! ## Dioxus equivalence
//!
//! Dioxus's `use_callback` already returns a referentially stable `Callback<T>`
//! that always invokes the latest closure. This module re-exports it as
//! `use_effect_event` so that upstream call-sites map 1:1.

pub use dioxus::prelude::use_callback as use_effect_event;
