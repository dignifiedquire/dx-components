//! Core types — port of `@floating-ui/utils/src/index.ts` lines 1-32
//! and `@floating-ui/core/src/types.ts`.

/// A rectangle with position and dimensions.
/// Source: `@floating-ui/utils` `Rect = Coords & Dimensions`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// The side of the reference element.
/// Source: `@floating-ui/utils` `Side = 'top' | 'right' | 'bottom' | 'left'`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

/// Alignment along the cross axis.
/// Source: `@floating-ui/utils` `Alignment = 'start' | 'end'`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    Start,
    End,
}

/// Placement = Side with optional Alignment.
/// Source: `@floating-ui/utils` `Placement = Side | AlignedPlacement`
/// 12 variants: 4 sides × (base + start + end)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Placement {
    Top,
    TopStart,
    TopEnd,
    Right,
    RightStart,
    RightEnd,
    #[default]
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
}

/// Axis: horizontal (X) or vertical (Y).
/// Source: `@floating-ui/utils` `Axis = 'x' | 'y'`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    X,
    Y,
}

/// Length dimension name.
/// Source: `@floating-ui/utils` `Length = 'width' | 'height'`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Length {
    Width,
    Height,
}

/// A pair of coordinates.
/// Source: `@floating-ui/utils` `Coords = {[key in Axis]: number}`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
}

/// Dimensions of an element.
/// Source: `@floating-ui/utils` `Dimensions = {[key in Length]: number}`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}

/// Values per side (overflow amounts, padding, etc.).
/// Source: `@floating-ui/utils` `SideObject = {[key in Side]: number}`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SideObject {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl SideObject {
    /// Get value by side.
    pub fn get(&self, side: Side) -> f64 {
        match side {
            Side::Top => self.top,
            Side::Right => self.right,
            Side::Bottom => self.bottom,
            Side::Left => self.left,
        }
    }
}

/// A rect with computed edge positions.
/// Source: `@floating-ui/utils` `ClientRectObject = Rect & SideObject`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ClientRectObject {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

/// The reference and floating element rects.
/// Source: `@floating-ui/utils` `ElementRects`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ElementRects {
    pub reference: Rect,
    pub floating: Rect,
}

/// Positioning strategy.
/// Source: `@floating-ui/utils` `Strategy = 'absolute' | 'fixed'`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Strategy {
    #[default]
    Absolute,
    Fixed,
}

/// Padding specification — uniform or per-side.
/// Source: `@floating-ui/utils` `Padding = number | Partial<SideObject>`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Padding {
    Uniform(f64),
    PerSide(SideObject),
}

impl Default for Padding {
    fn default() -> Self {
        Self::Uniform(0.0)
    }
}

impl From<f64> for Padding {
    fn from(v: f64) -> Self {
        Self::Uniform(v)
    }
}

// ---------------------------------------------------------------------------
// Middleware data types (typed, not HashMap<String, Any>)
// Source: core/src/types.ts lines 56-86
// ---------------------------------------------------------------------------

/// Data returned by the offset middleware.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct OffsetData {
    pub x: f64,
    pub y: f64,
    pub placement: Placement,
}

/// Data returned by the flip middleware.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FlipData {
    pub index: usize,
    pub overflows: Vec<PlacementOverflow>,
}

/// Overflow info for a placement (used by flip).
#[derive(Debug, Clone, PartialEq)]
pub struct PlacementOverflow {
    pub placement: Placement,
    pub overflows: Vec<f64>,
}

/// Data returned by the shift middleware.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ShiftData {
    pub x: f64,
    pub y: f64,
    pub enabled_x: bool,
    pub enabled_y: bool,
}

/// Data returned by the arrow middleware.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ArrowData {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub center_offset: f64,
    pub alignment_offset: Option<f64>,
}

/// Data returned by the hide middleware.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct HideData {
    pub reference_hidden: bool,
    pub escaped: bool,
    pub reference_hidden_offsets: Option<SideObject>,
    pub escaped_offsets: Option<SideObject>,
}

/// Data returned by the size middleware.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SizeData {
    pub available_width: f64,
    pub available_height: f64,
}

/// All middleware data collected during a compute_position run.
/// Source: core/src/types.ts lines 56-86
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MiddlewareData {
    pub offset: Option<OffsetData>,
    pub flip: Option<FlipData>,
    pub shift: Option<ShiftData>,
    pub arrow: Option<ArrowData>,
    pub hide: Option<HideData>,
    pub size: Option<SizeData>,
}

// ---------------------------------------------------------------------------
// Middleware state and return types
// Source: core/src/types.ts lines 129-163
// ---------------------------------------------------------------------------

/// State passed to each middleware's compute function.
#[derive(Debug, Clone, PartialEq)]
pub struct MiddlewareState {
    pub x: f64,
    pub y: f64,
    pub initial_placement: Placement,
    pub placement: Placement,
    pub strategy: Strategy,
    pub rects: ElementRects,
    pub middleware_data: MiddlewareData,
}

/// Return value from a middleware's compute function.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MiddlewareReturn {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub reset: Reset,
    // Data is set via the MiddlewareDataUpdate enum
    pub data: MiddlewareDataUpdate,
}

/// How to reset the middleware chain.
/// Source: core/src/types.ts lines 133-138
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Reset {
    #[default]
    None,
    /// reset: true — restart from beginning
    Simple,
    /// reset: { placement } — change placement and restart
    WithPlacement(Placement),
    /// reset: { rects: true } — re-measure rects and restart
    WithRects,
}

/// Typed middleware data update (instead of `data?: { [key: string]: any }`).
#[derive(Debug, Clone, PartialEq, Default)]
pub enum MiddlewareDataUpdate {
    #[default]
    None,
    Offset(OffsetData),
    Flip(FlipData),
    Shift(ShiftData),
    Arrow(ArrowData),
    Hide(HideData),
    Size(SizeData),
}

// ---------------------------------------------------------------------------
// compute_position config and return
// Source: core/src/types.ts lines 88-127
// ---------------------------------------------------------------------------

/// Return value from compute_position.
#[derive(Debug, Clone, PartialEq)]
pub struct ComputePositionReturn {
    pub x: f64,
    pub y: f64,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middleware_data: MiddlewareData,
}

// ---------------------------------------------------------------------------
// DetectOverflow options
// Source: core/src/detectOverflow.ts lines 12-39
// ---------------------------------------------------------------------------

/// Options for overflow detection.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DetectOverflowOptions {
    /// Virtual padding around the clipping boundary.
    pub padding: Padding,
    /// Whether to check overflow using the alternate element's boundary.
    pub alt_boundary: bool,
    /// Which element to check overflow for.
    pub element_context: ElementContext,
}

/// Which element overflow is checked for.
/// Source: core/src/types.ts line 171
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ElementContext {
    Reference,
    #[default]
    Floating,
}

// ---------------------------------------------------------------------------
// Rect helpers
// ---------------------------------------------------------------------------

impl Rect {
    /// Get dimension by axis.
    pub fn axis_length(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.width,
            Axis::Y => self.height,
        }
    }

    /// Get position by axis.
    pub fn axis_pos(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    }

    /// Get dimension by length.
    pub fn length(&self, len: Length) -> f64 {
        match len {
            Length::Width => self.width,
            Length::Height => self.height,
        }
    }
}

impl Coords {
    /// Get coordinate by axis.
    pub fn get(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    }

    /// Set coordinate by axis.
    pub fn set(&mut self, axis: Axis, value: f64) {
        match axis {
            Axis::X => self.x = value,
            Axis::Y => self.y = value,
        }
    }
}
