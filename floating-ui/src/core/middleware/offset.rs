//! Port of `@floating-ui/core/src/middleware/offset.ts` (110 lines).

use crate::types::*;
use crate::utils::*;

/// Options for the offset middleware.
/// Source: offset.ts `OffsetValue` type (lines 12-39)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct OffsetOptions {
    /// Distance from the reference edge (along the side axis).
    /// Default: 0
    pub main_axis: f64,
    /// Skidding offset (along the alignment axis).
    /// Default: 0
    pub cross_axis: f64,
    /// Overrides `cross_axis` for aligned placements, inverts for `end` alignment.
    /// Default: None (use cross_axis)
    pub alignment_axis: Option<f64>,
}

impl From<f64> for OffsetOptions {
    fn from(v: f64) -> Self {
        Self {
            main_axis: v,
            ..Default::default()
        }
    }
}

/// Convert offset options to x/y coordinate deltas based on placement.
/// Source: offset.ts `convertValueToCoords` (lines 45-76)
pub fn convert_value_to_coords(placement: Placement, options: &OffsetOptions) -> Coords {
    let side = get_side(placement);
    let alignment = get_alignment(placement);
    let is_vertical = get_side_axis(placement) == Axis::Y;
    let main_axis_multi = if is_origin_side(side) { -1.0 } else { 1.0 };
    let cross_axis_multi = 1.0; // rtl would be -1.0 for vertical

    let main_axis = options.main_axis;
    let mut cross_axis = options.cross_axis;

    if let (Some(_align), Some(alignment_axis)) = (alignment, options.alignment_axis) {
        cross_axis = if _align == Alignment::End {
            -alignment_axis
        } else {
            alignment_axis
        };
    }

    if is_vertical {
        Coords {
            x: cross_axis * cross_axis_multi,
            y: main_axis * main_axis_multi,
        }
    } else {
        Coords {
            x: main_axis * main_axis_multi,
            y: cross_axis * cross_axis_multi,
        }
    }
}

/// Compute offset middleware result.
/// Source: offset.ts lines 85-110
pub fn compute(state: &MiddlewareState, options: &OffsetOptions) -> MiddlewareReturn {
    let diff = convert_value_to_coords(state.placement, options);

    // Skip if placement matches and arrow caused a non-zero alignment offset.
    // In JS, `alignmentOffset` is checked for truthiness — 0 is falsy.
    // Source: offset.ts lines 94-98
    if let Some(ref offset_data) = state.middleware_data.offset {
        if state.placement == offset_data.placement {
            if let Some(ref arrow_data) = state.middleware_data.arrow {
                if arrow_data.alignment_offset.is_some_and(|v| v != 0.0) {
                    return MiddlewareReturn::default();
                }
            }
        }
    }

    MiddlewareReturn {
        x: Some(state.x + diff.x),
        y: Some(state.y + diff.y),
        data: MiddlewareDataUpdate::Offset(OffsetData {
            x: diff.x,
            y: diff.y,
            placement: state.placement,
        }),
        reset: Reset::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_bottom_main_axis() {
        let diff = convert_value_to_coords(
            Placement::Bottom,
            &OffsetOptions {
                main_axis: 8.0,
                ..Default::default()
            },
        );
        // Bottom is not an origin side, so main_axis_multi = 1
        // is_vertical = true, so y = mainAxis * 1 = 8
        assert_eq!(diff.x, 0.0);
        assert_eq!(diff.y, 8.0);
    }

    #[test]
    fn test_offset_top_main_axis() {
        let diff = convert_value_to_coords(
            Placement::Top,
            &OffsetOptions {
                main_axis: 8.0,
                ..Default::default()
            },
        );
        // Top is an origin side, so main_axis_multi = -1
        // y = 8 * -1 = -8
        assert_eq!(diff.y, -8.0);
    }

    #[test]
    fn test_offset_right_main_axis() {
        let diff = convert_value_to_coords(
            Placement::Right,
            &OffsetOptions {
                main_axis: 8.0,
                ..Default::default()
            },
        );
        // Right is not origin, horizontal, so x = 8 * 1 = 8
        assert_eq!(diff.x, 8.0);
        assert_eq!(diff.y, 0.0);
    }

    #[test]
    fn test_offset_alignment_axis() {
        let opts = OffsetOptions {
            main_axis: 4.0,
            alignment_axis: Some(8.0),
            ..Default::default()
        };
        let diff = convert_value_to_coords(Placement::BottomStart, &opts);
        // alignment = Start, so cross_axis = alignment_axis = 8
        // is_vertical = true: x = cross * 1 = 8, y = main * 1 = 4
        assert_eq!(diff.x, 8.0);
        assert_eq!(diff.y, 4.0);
    }

    #[test]
    fn test_offset_alignment_axis_end() {
        let opts = OffsetOptions {
            main_axis: 4.0,
            alignment_axis: Some(8.0),
            ..Default::default()
        };
        let diff = convert_value_to_coords(Placement::BottomEnd, &opts);
        // alignment = End, so cross_axis = alignment_axis * -1 = -8
        assert_eq!(diff.x, -8.0);
        assert_eq!(diff.y, 4.0);
    }
}
