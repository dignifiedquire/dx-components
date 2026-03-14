//! Port of `@floating-ui/core/src/computeCoordsFromPlacement.ts` (54 lines).

use crate::types::*;
use crate::utils::*;

/// Compute initial floating element coordinates based on placement and element rects.
///
/// Source: `computeCoordsFromPlacement.ts` lines 10-54
pub fn compute_coords_from_placement(rects: &ElementRects, placement: Placement) -> Coords {
    let reference = &rects.reference;
    let floating = &rects.floating;

    let side_axis = get_side_axis(placement);
    let alignment_axis = get_alignment_axis(placement);
    let align_length = get_axis_length(alignment_axis);
    let side = get_side(placement);
    let _is_vertical = side_axis == Axis::Y; // Used by RTL logic when implemented

    let common_x = reference.x + reference.width / 2.0 - floating.width / 2.0;
    let common_y = reference.y + reference.height / 2.0 - floating.height / 2.0;
    let common_align = reference.length(align_length) / 2.0 - floating.length(align_length) / 2.0;

    let mut coords = match side {
        Side::Top => Coords {
            x: common_x,
            y: reference.y - floating.height,
        },
        Side::Bottom => Coords {
            x: common_x,
            y: reference.y + reference.height,
        },
        Side::Right => Coords {
            x: reference.x + reference.width,
            y: common_y,
        },
        Side::Left => Coords {
            x: reference.x - floating.width,
            y: common_y,
        },
    };

    // RTL multiplier — currently always 1.0 (RTL not yet supported).
    // When RTL: `rtl && isVertical ? -1 : 1` (source: computeCoordsFromPlacement.ts line 45)
    let rtl_mult = 1.0;

    match get_alignment(placement) {
        Some(Alignment::Start) => {
            coords.set(
                alignment_axis,
                coords.get(alignment_axis) - common_align * rtl_mult,
            );
        }
        Some(Alignment::End) => {
            coords.set(
                alignment_axis,
                coords.get(alignment_axis) + common_align * rtl_mult,
            );
        }
        None => {}
    }

    coords
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rects(
        ref_x: f64,
        ref_y: f64,
        ref_w: f64,
        ref_h: f64,
        float_w: f64,
        float_h: f64,
    ) -> ElementRects {
        ElementRects {
            reference: Rect {
                x: ref_x,
                y: ref_y,
                width: ref_w,
                height: ref_h,
            },
            floating: Rect {
                x: 0.0,
                y: 0.0,
                width: float_w,
                height: float_h,
            },
        }
    }

    #[test]
    fn test_bottom_center() {
        let rects = make_rects(100.0, 50.0, 200.0, 40.0, 150.0, 100.0);
        let coords = compute_coords_from_placement(&rects, Placement::Bottom);
        // x = 100 + 200/2 - 150/2 = 100 + 100 - 75 = 125
        assert_eq!(coords.x, 125.0);
        // y = 50 + 40 = 90
        assert_eq!(coords.y, 90.0);
    }

    #[test]
    fn test_top_center() {
        let rects = make_rects(100.0, 200.0, 200.0, 40.0, 150.0, 100.0);
        let coords = compute_coords_from_placement(&rects, Placement::Top);
        // x = 100 + 200/2 - 150/2 = 125
        assert_eq!(coords.x, 125.0);
        // y = 200 - 100 = 100
        assert_eq!(coords.y, 100.0);
    }

    #[test]
    fn test_right_center() {
        let rects = make_rects(50.0, 100.0, 200.0, 40.0, 150.0, 100.0);
        let coords = compute_coords_from_placement(&rects, Placement::Right);
        // x = 50 + 200 = 250
        assert_eq!(coords.x, 250.0);
        // y = 100 + 40/2 - 100/2 = 100 + 20 - 50 = 70
        assert_eq!(coords.y, 70.0);
    }

    #[test]
    fn test_bottom_start() {
        let rects = make_rects(100.0, 50.0, 200.0, 40.0, 150.0, 100.0);
        let coords = compute_coords_from_placement(&rects, Placement::BottomStart);
        // For BottomStart: alignment_axis = X, common_align = 200/2 - 150/2 = 25
        // x = 125 - 25 = 100 (aligned to start of reference)
        assert_eq!(coords.x, 100.0);
        // y = 50 + 40 = 90
        assert_eq!(coords.y, 90.0);
    }

    #[test]
    fn test_bottom_end() {
        let rects = make_rects(100.0, 50.0, 200.0, 40.0, 150.0, 100.0);
        let coords = compute_coords_from_placement(&rects, Placement::BottomEnd);
        // For BottomEnd: x = 125 + 25 = 150
        assert_eq!(coords.x, 150.0);
        assert_eq!(coords.y, 90.0);
    }
}
