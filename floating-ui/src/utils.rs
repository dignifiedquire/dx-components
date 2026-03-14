//! Utility functions — port of `@floating-ui/utils/src/index.ts` lines 34-199
//! and `@floating-ui/core/src/constants.ts`.

use crate::types::*;

// ---------------------------------------------------------------------------
// Constants
// Source: core/src/constants.ts line 1
// Source: utils/src/index.ts line 48-53
// ---------------------------------------------------------------------------

/// Sides where offset multiplier is -1 (top and left are "origin" sides).
pub fn is_origin_side(side: Side) -> bool {
    matches!(side, Side::Left | Side::Top)
}

/// Source: utils/src/index.ts line 48-53
pub const OPPOSITE_SIDE: [(Side, Side); 4] = [
    (Side::Left, Side::Right),
    (Side::Right, Side::Left),
    (Side::Bottom, Side::Top),
    (Side::Top, Side::Bottom),
];

pub fn opposite_side(side: Side) -> Side {
    match side {
        Side::Left => Side::Right,
        Side::Right => Side::Left,
        Side::Bottom => Side::Top,
        Side::Top => Side::Bottom,
    }
}

// ---------------------------------------------------------------------------
// Placement helpers
// Source: utils/src/index.ts lines 65-175
// ---------------------------------------------------------------------------

/// Get the side from a placement.
/// Source: utils/src/index.ts line 65
pub fn get_side(placement: Placement) -> Side {
    match placement {
        Placement::Top | Placement::TopStart | Placement::TopEnd => Side::Top,
        Placement::Right | Placement::RightStart | Placement::RightEnd => Side::Right,
        Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => Side::Bottom,
        Placement::Left | Placement::LeftStart | Placement::LeftEnd => Side::Left,
    }
}

/// Get the alignment from a placement (`None` for base placements).
/// Source: utils/src/index.ts line 69
pub fn get_alignment(placement: Placement) -> Option<Alignment> {
    match placement {
        Placement::TopStart
        | Placement::RightStart
        | Placement::BottomStart
        | Placement::LeftStart => Some(Alignment::Start),
        Placement::TopEnd | Placement::RightEnd | Placement::BottomEnd | Placement::LeftEnd => {
            Some(Alignment::End)
        }
        _ => None,
    }
}

/// Get the opposite axis.
/// Source: utils/src/index.ts line 73
pub fn get_opposite_axis(axis: Axis) -> Axis {
    match axis {
        Axis::X => Axis::Y,
        Axis::Y => Axis::X,
    }
}

/// Get the length dimension for an axis.
/// Source: utils/src/index.ts line 77
pub fn get_axis_length(axis: Axis) -> Length {
    match axis {
        Axis::Y => Length::Height,
        Axis::X => Length::Width,
    }
}

/// Get the axis that runs along the side of the floating element.
/// Source: utils/src/index.ts line 81
pub fn get_side_axis(placement: Placement) -> Axis {
    match get_side(placement) {
        Side::Top | Side::Bottom => Axis::Y,
        Side::Left | Side::Right => Axis::X,
    }
}

/// Get the axis that runs along the alignment of the floating element.
/// Source: utils/src/index.ts line 86
pub fn get_alignment_axis(placement: Placement) -> Axis {
    get_opposite_axis(get_side_axis(placement))
}

/// Get the two alignment sides for a placement.
/// Source: utils/src/index.ts line 90-113
pub fn get_alignment_sides(placement: Placement, rects: &ElementRects, rtl: bool) -> (Side, Side) {
    let alignment = get_alignment(placement);
    let alignment_axis = get_alignment_axis(placement);
    let length = get_axis_length(alignment_axis);

    let ref_len = rects.reference.length(length);
    let float_len = rects.floating.length(length);

    let mut main_alignment_side = if alignment_axis == Axis::X {
        if alignment
            == Some(if rtl {
                Alignment::End
            } else {
                Alignment::Start
            })
        {
            Side::Right
        } else {
            Side::Left
        }
    } else if alignment == Some(Alignment::Start) {
        Side::Bottom
    } else {
        Side::Top
    };

    if ref_len > float_len {
        main_alignment_side = opposite_side(main_alignment_side);
    }

    (main_alignment_side, opposite_side(main_alignment_side))
}

/// Get the opposite placement (e.g., Bottom → Top, BottomStart → TopStart).
/// Source: utils/src/index.ts line 172
pub fn get_opposite_placement(placement: Placement) -> Placement {
    let side = get_side(placement);
    let opp_side = opposite_side(side);
    let alignment = get_alignment(placement);
    Placement::from_side_align(opp_side, alignment)
}

/// Get opposite alignment placement (e.g., BottomStart → BottomEnd).
/// Source: utils/src/index.ts line 125
pub fn get_opposite_alignment_placement(placement: Placement) -> Placement {
    let side = get_side(placement);
    let alignment = get_alignment(placement);
    let opp_alignment = alignment.map(|a| match a {
        Alignment::Start => Alignment::End,
        Alignment::End => Alignment::Start,
    });
    Placement::from_side_align(side, opp_alignment)
}

/// Get expanded placements: [opposite-align, opposite-side, opposite-side+opposite-align].
/// Source: utils/src/index.ts line 115
pub fn get_expanded_placements(placement: Placement) -> Vec<Placement> {
    let opposite = get_opposite_placement(placement);
    vec![
        get_opposite_alignment_placement(placement),
        opposite,
        get_opposite_alignment_placement(opposite),
    ]
}

/// Get opposite axis placements for fallback.
/// Source: utils/src/index.ts line 152-170
pub fn get_opposite_axis_placements(
    placement: Placement,
    flip_alignment: bool,
    direction: Alignment,
    rtl: bool,
) -> Vec<Placement> {
    let alignment = get_alignment(placement);
    let side = get_side(placement);
    let is_start = direction == Alignment::Start;

    let list = get_side_list(side, is_start, rtl);

    let mut result: Vec<Placement> = if let Some(align) = alignment {
        list.iter()
            .map(|&s| Placement::from_side_align(s, Some(align)))
            .collect()
    } else {
        list.iter()
            .map(|&s| Placement::from_side_align(s, None))
            .collect()
    };

    if alignment.is_some() && flip_alignment {
        let extra: Vec<Placement> = result
            .iter()
            .map(|&p| get_opposite_alignment_placement(p))
            .collect();
        result.extend(extra);
    }

    result
}

/// Helper for get_opposite_axis_placements.
/// Source: utils/src/index.ts line 138-150
fn get_side_list(side: Side, is_start: bool, rtl: bool) -> Vec<Side> {
    match side {
        Side::Top | Side::Bottom => {
            if rtl {
                if is_start {
                    vec![Side::Right, Side::Left]
                } else {
                    vec![Side::Left, Side::Right]
                }
            } else if is_start {
                vec![Side::Left, Side::Right]
            } else {
                vec![Side::Right, Side::Left]
            }
        }
        Side::Left | Side::Right => {
            if is_start {
                vec![Side::Top, Side::Bottom]
            } else {
                vec![Side::Bottom, Side::Top]
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Placement construction
// ---------------------------------------------------------------------------

impl Placement {
    /// Construct from side and optional alignment.
    pub fn from_side_align(side: Side, alignment: Option<Alignment>) -> Self {
        match (side, alignment) {
            (Side::Top, None) => Self::Top,
            (Side::Top, Some(Alignment::Start)) => Self::TopStart,
            (Side::Top, Some(Alignment::End)) => Self::TopEnd,
            (Side::Right, None) => Self::Right,
            (Side::Right, Some(Alignment::Start)) => Self::RightStart,
            (Side::Right, Some(Alignment::End)) => Self::RightEnd,
            (Side::Bottom, None) => Self::Bottom,
            (Side::Bottom, Some(Alignment::Start)) => Self::BottomStart,
            (Side::Bottom, Some(Alignment::End)) => Self::BottomEnd,
            (Side::Left, None) => Self::Left,
            (Side::Left, Some(Alignment::Start)) => Self::LeftStart,
            (Side::Left, Some(Alignment::End)) => Self::LeftEnd,
        }
    }

    /// Get the side.
    pub fn side(self) -> Side {
        get_side(self)
    }

    /// Get the alignment.
    pub fn alignment(self) -> Option<Alignment> {
        get_alignment(self)
    }

    /// Get the string representation (e.g. "bottom-start").
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::TopStart => "top-start",
            Self::TopEnd => "top-end",
            Self::Right => "right",
            Self::RightStart => "right-start",
            Self::RightEnd => "right-end",
            Self::Bottom => "bottom",
            Self::BottomStart => "bottom-start",
            Self::BottomEnd => "bottom-end",
            Self::Left => "left",
            Self::LeftStart => "left-start",
            Self::LeftEnd => "left-end",
        }
    }
}

impl Side {
    /// Get the string representation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }

    /// Get the opposite side.
    pub fn opposite(self) -> Self {
        opposite_side(self)
    }
}

impl Alignment {
    /// Get the string representation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

// ---------------------------------------------------------------------------
// Math helpers
// Source: utils/src/index.ts lines 55-63
// ---------------------------------------------------------------------------

/// Clamp a value between a minimum and maximum.
/// Source: utils/src/index.ts line 55
pub fn clamp(start: f64, value: f64, end: f64) -> f64 {
    start.max(value.min(end))
}

// ---------------------------------------------------------------------------
// Padding helpers
// Source: utils/src/index.ts lines 177-185
// ---------------------------------------------------------------------------

/// Convert padding to a SideObject.
/// Source: utils/src/index.ts line 181
pub fn get_padding_object(padding: Padding) -> SideObject {
    match padding {
        Padding::Uniform(v) => SideObject {
            top: v,
            right: v,
            bottom: v,
            left: v,
        },
        Padding::PerSide(obj) => obj,
    }
}

// ---------------------------------------------------------------------------
// Rect conversion
// Source: utils/src/index.ts line 187
// ---------------------------------------------------------------------------

/// Convert a Rect to a ClientRectObject (add computed edges).
/// Source: utils/src/index.ts line 187
pub fn rect_to_client_rect(rect: Rect) -> ClientRectObject {
    ClientRectObject {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
        top: rect.y,
        left: rect.x,
        right: rect.x + rect.width,
        bottom: rect.y + rect.height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_side() {
        assert_eq!(get_side(Placement::Top), Side::Top);
        assert_eq!(get_side(Placement::BottomStart), Side::Bottom);
        assert_eq!(get_side(Placement::LeftEnd), Side::Left);
    }

    #[test]
    fn test_get_alignment() {
        assert_eq!(get_alignment(Placement::Top), None);
        assert_eq!(get_alignment(Placement::TopStart), Some(Alignment::Start));
        assert_eq!(get_alignment(Placement::BottomEnd), Some(Alignment::End));
    }

    #[test]
    fn test_get_side_axis() {
        assert_eq!(get_side_axis(Placement::Top), Axis::Y);
        assert_eq!(get_side_axis(Placement::Bottom), Axis::Y);
        assert_eq!(get_side_axis(Placement::Left), Axis::X);
        assert_eq!(get_side_axis(Placement::Right), Axis::X);
    }

    #[test]
    fn test_opposite_placement() {
        assert_eq!(get_opposite_placement(Placement::Top), Placement::Bottom);
        assert_eq!(
            get_opposite_placement(Placement::BottomStart),
            Placement::TopStart
        );
        assert_eq!(
            get_opposite_placement(Placement::LeftEnd),
            Placement::RightEnd
        );
    }

    #[test]
    fn test_opposite_alignment() {
        assert_eq!(
            get_opposite_alignment_placement(Placement::TopStart),
            Placement::TopEnd
        );
        assert_eq!(
            get_opposite_alignment_placement(Placement::LeftEnd),
            Placement::LeftStart
        );
        assert_eq!(
            get_opposite_alignment_placement(Placement::Bottom),
            Placement::Bottom
        );
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(0.0, 5.0, 10.0), 5.0);
        assert_eq!(clamp(0.0, -1.0, 10.0), 0.0);
        assert_eq!(clamp(0.0, 15.0, 10.0), 10.0);
    }

    #[test]
    fn test_rect_to_client_rect() {
        let rect = Rect {
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 50.0,
        };
        let cr = rect_to_client_rect(rect);
        assert_eq!(cr.top, 20.0);
        assert_eq!(cr.left, 10.0);
        assert_eq!(cr.right, 110.0);
        assert_eq!(cr.bottom, 70.0);
    }

    #[test]
    fn test_expanded_placements() {
        let expanded = get_expanded_placements(Placement::BottomStart);
        assert_eq!(
            expanded,
            vec![
                Placement::BottomEnd, // opposite alignment
                Placement::TopStart,  // opposite side
                Placement::TopEnd,    // opposite side + opposite alignment
            ]
        );
    }

    #[test]
    fn test_from_side_align() {
        assert_eq!(Placement::from_side_align(Side::Top, None), Placement::Top);
        assert_eq!(
            Placement::from_side_align(Side::Bottom, Some(Alignment::Start)),
            Placement::BottomStart
        );
        assert_eq!(
            Placement::from_side_align(Side::Right, Some(Alignment::End)),
            Placement::RightEnd
        );
    }
}
