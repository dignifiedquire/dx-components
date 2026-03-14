//! Rust port of `@floating-ui/core` + `@floating-ui/dom`.
//!
//! Provides floating element positioning with collision detection,
//! matching the upstream floating-ui TypeScript library 1:1.
//!
//! # Architecture
//!
//! - `types` + `utils` — Foundation types and helpers (port of `@floating-ui/utils`)
//! - `core` — Pure math computation (port of `@floating-ui/core`)
//! - `dom` — DOM platform with web-sys (port of `@floating-ui/dom`) [wasm32 only]

pub mod core;
pub mod types;
pub mod utils;

#[cfg(target_arch = "wasm32")]
pub mod dom;

// Re-export commonly used items
pub use core::compute_position::compute_position;
pub use core::middleware::{
    ArrowOptions, FlipFallbackStrategy, FlipOptions, HideOptions, HideStrategy, LimitShift,
    Middleware, OffsetOptions, ShiftOptions, SizeOptions,
};
pub use types::*;
pub use utils::{get_alignment, get_padding_object, get_side, rect_to_client_rect};
