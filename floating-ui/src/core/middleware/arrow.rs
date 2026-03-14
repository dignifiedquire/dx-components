//! Port of `@floating-ui/core/src/middleware/arrow.ts` (117 lines).

use crate::types::*;
use crate::utils::*;

/// Options for the arrow middleware.
/// Source: arrow.ts `ArrowOptions` (lines 14-26)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArrowOptions {
    /// Arrow element width (along alignment axis).
    pub width: f64,
    /// Arrow element height (perpendicular to alignment axis).
    pub height: f64,
    /// Padding between arrow and floating element edges. Default: 0
    pub padding: Padding,
}

impl Default for ArrowOptions {
    fn default() -> Self {
        Self {
            width: 10.0,
            height: 5.0,
            padding: Padding::Uniform(0.0),
        }
    }
}

/// Compute arrow middleware result.
/// Source: arrow.ts lines 33-117
pub fn compute(state: &MiddlewareState, options: &ArrowOptions) -> MiddlewareReturn {
    let padding_object = get_padding_object(options.padding);
    let axis = get_alignment_axis(state.placement);
    let _length = get_axis_length(axis);
    let is_y_axis = axis == Axis::Y;

    // Arrow dimensions along the alignment axis (for positioning along the edge)
    let arrow_length = if is_y_axis {
        options.width
    } else {
        options.height
    };

    let min_prop = if is_y_axis { Side::Top } else { Side::Left };
    let max_prop = if is_y_axis { Side::Bottom } else { Side::Right };

    let coords_val = if is_y_axis { state.y } else { state.x };

    let end_diff = state.rects.reference.axis_length(axis) + state.rects.reference.axis_pos(axis)
        - coords_val
        - state.rects.floating.axis_length(axis);
    let start_diff = coords_val - state.rects.reference.axis_pos(axis);

    // The client size of the floating element along the axis
    let client_size = state.rects.floating.axis_length(axis);

    let center_to_reference = end_diff / 2.0 - start_diff / 2.0;

    let largest_possible_padding = client_size / 2.0 - arrow_length / 2.0 - 1.0;
    let min_padding = padding_object.get(min_prop).min(largest_possible_padding);
    let max_padding = padding_object.get(max_prop).min(largest_possible_padding);

    let min = min_padding;
    let max = client_size - arrow_length - max_padding;
    let center = client_size / 2.0 - arrow_length / 2.0 + center_to_reference;
    let offset = clamp(min, center, max);

    let should_add_offset = state.middleware_data.arrow.is_none()
        && get_alignment(state.placement).is_some()
        && (center - offset).abs() > f64::EPSILON
        && state.rects.reference.axis_length(axis) / 2.0
            - (if center < min {
                min_padding
            } else {
                max_padding
            })
            - arrow_length / 2.0
            < 0.0;

    let alignment_offset = if should_add_offset {
        if center < min {
            center - min
        } else {
            center - max
        }
    } else {
        0.0
    };

    let mut result = MiddlewareReturn {
        data: MiddlewareDataUpdate::Arrow(ArrowData {
            x: if is_y_axis { None } else { Some(offset) },
            y: if is_y_axis { Some(offset) } else { None },
            center_offset: center - offset - alignment_offset,
            alignment_offset: if should_add_offset {
                Some(alignment_offset)
            } else {
                None
            },
        }),
        reset: if should_add_offset {
            Reset::Simple
        } else {
            Reset::None
        },
        ..Default::default()
    };

    if is_y_axis {
        result.y = Some(coords_val + alignment_offset);
    } else {
        result.x = Some(coords_val + alignment_offset);
    }

    result
}
