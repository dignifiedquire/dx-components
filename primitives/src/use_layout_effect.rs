//! Line-by-line port of `@radix-ui/react-use-layout-effect`.
//!
//! Upstream provides an SSR-safe version of `React.useLayoutEffect` that
//! replaces itself with a no-op on the server to suppress React's SSR warning:
//!
//! ```js
//! const useLayoutEffect = globalThis?.document ? React.useLayoutEffect : () => {};
//! ```
//!
//! ## Dioxus equivalence
//!
//! Dioxus's `use_effect` does not emit SSR warnings and already behaves
//! correctly on both client and server. This module re-exports `use_effect`
//! as `use_layout_effect` so that upstream call-sites map 1:1.

pub use dioxus::prelude::use_effect as use_layout_effect;
