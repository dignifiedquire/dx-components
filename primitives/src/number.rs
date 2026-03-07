//! Number utilities — matches `@radix-ui/number`.
//!
//! Provides [`clamp`] for constraining a value within a range.

/// Clamps `value` between `min` and `max` (inclusive).
///
/// Matches Radix's `clamp(value, [min, max])`.
///
/// ```
/// use dioxus_primitives::number::clamp;
///
/// assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
/// assert_eq!(clamp(-1.0, 0.0, 10.0), 0.0);
/// assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
/// ```
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.clamp(min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn within_range() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
    }

    #[test]
    fn below_min() {
        assert_eq!(clamp(-5.0, 0.0, 10.0), 0.0);
    }

    #[test]
    fn above_max() {
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn at_boundaries() {
        assert_eq!(clamp(0.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(10.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn min_equals_max() {
        assert_eq!(clamp(5.0, 3.0, 3.0), 3.0);
    }
}
