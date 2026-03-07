//! Direction and orientation primitives — matches `@radix-ui/react-direction`.
//!
//! Provides [`Direction`] (LTR/RTL), [`Orientation`] (vertical/horizontal),
//! [`DirectionProvider`], and [`use_direction`].

use dioxus::prelude::*;

/// Layout direction for keyboard navigation and content flow.
///
/// Matches Radix's `Direction = 'ltr' | 'rtl'`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Direction {
    /// Left-to-right.
    #[default]
    Ltr,
    /// Right-to-left.
    Rtl,
}

impl Direction {
    /// Returns `"ltr"` or `"rtl"`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }
}

/// Component orientation for keyboard navigation.
///
/// Matches Radix's `orientation` prop used across Accordion, Tabs, RadioGroup,
/// ToggleGroup, Toolbar, Slider, etc.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Orientation {
    /// Arrow Up/Down navigates.
    #[default]
    Vertical,
    /// Arrow Left/Right navigates.
    Horizontal,
}

impl Orientation {
    /// Returns `"vertical"` or `"horizontal"`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

/// Props for [`DirectionProvider`].
#[derive(Props, Clone, PartialEq)]
pub struct DirectionProviderProps {
    /// The text direction to provide.
    pub dir: Direction,
    /// Children that will inherit this direction.
    pub children: Element,
}

/// Provides a global text direction context, matching Radix's `DirectionProvider`.
///
/// Components that support RTL (e.g., Accordion, Tabs, Menus) use [`use_direction`]
/// to resolve the effective direction.
#[component]
pub fn DirectionProvider(props: DirectionProviderProps) -> Element {
    use_context_provider(|| props.dir);
    props.children
}

/// Returns the effective text direction.
///
/// Matches Radix's `useDirection(localDir?)`:
/// - If `local_dir` is `Some`, returns it (local override).
/// - Otherwise falls back to the nearest [`DirectionProvider`] context.
/// - Defaults to [`Direction::Ltr`] if no provider exists.
pub fn use_direction(local_dir: Option<Direction>) -> Direction {
    if let Some(dir) = local_dir {
        return dir;
    }
    try_consume_context::<Direction>().unwrap_or_default()
}
