//! Middleware implementations — port of `@floating-ui/core/src/middleware/`.
//!
//! Uses an enum instead of `Box<dyn Trait>` since the set of middleware is fixed.

pub mod arrow;
pub mod flip;
pub mod hide;
pub mod offset;
pub mod shift;
pub mod size;

use crate::types::*;

pub use arrow::ArrowOptions;
pub use flip::{FlipFallbackStrategy, FlipOptions};
pub use hide::{HideOptions, HideStrategy};
pub use offset::OffsetOptions;
pub use shift::{LimitShift, ShiftOptions};
pub use size::SizeOptions;

/// A middleware that modifies the positioning or provides data.
/// Enum dispatch — no heap allocation, no trait objects.
#[derive(Debug, Clone, PartialEq)]
pub enum Middleware {
    Offset(OffsetOptions),
    Flip(FlipOptions),
    Shift(ShiftOptions),
    Size(SizeOptions),
    Arrow(ArrowOptions),
    Hide(HideOptions),
}

impl Middleware {
    /// The name of this middleware (matches upstream `name` field).
    pub fn name(&self) -> &'static str {
        match self {
            Self::Offset(_) => "offset",
            Self::Flip(_) => "flip",
            Self::Shift(_) => "shift",
            Self::Size(_) => "size",
            Self::Arrow(_) => "arrow",
            Self::Hide(_) => "hide",
        }
    }

    /// Run this middleware's computation.
    ///
    /// `detect_overflow_fn` provides overflow detection — the DOM platform
    /// supplies this by computing clipping rects from the actual DOM.
    pub fn compute(
        &self,
        state: &MiddlewareState,
        detect_overflow_fn: &dyn Fn(&MiddlewareState, &DetectOverflowOptions) -> SideObject,
    ) -> MiddlewareReturn {
        match self {
            Self::Offset(opts) => offset::compute(state, opts),
            Self::Flip(opts) => flip::compute(state, opts, detect_overflow_fn),
            Self::Shift(opts) => shift::compute(state, opts, detect_overflow_fn),
            Self::Size(opts) => size::compute(state, opts, detect_overflow_fn),
            Self::Arrow(opts) => arrow::compute(state, opts),
            Self::Hide(opts) => hide::compute(state, opts, detect_overflow_fn),
        }
    }
}

/// Merge a middleware data update into the accumulated middleware data.
impl MiddlewareData {
    pub fn merge(&mut self, update: &MiddlewareDataUpdate) {
        match update {
            MiddlewareDataUpdate::None => {}
            MiddlewareDataUpdate::Offset(d) => self.offset = Some(*d),
            MiddlewareDataUpdate::Flip(d) => self.flip = Some(d.clone()),
            MiddlewareDataUpdate::Shift(d) => self.shift = Some(*d),
            MiddlewareDataUpdate::Arrow(d) => self.arrow = Some(*d),
            MiddlewareDataUpdate::Hide(d) => self.hide = Some(*d),
            MiddlewareDataUpdate::Size(d) => self.size = Some(*d),
        }
    }
}
