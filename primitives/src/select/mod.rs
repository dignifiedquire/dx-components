//! Select primitive — matches Radix UI Select structure.
//!
//! - [`Select`]: No DOM, pure context provider
//! - [`SelectTrigger`]: Button with `role="combobox"`
//! - [`SelectValue`]: Displays the currently selected value
//! - [`SelectContent`]: Dropdown container with `role="listbox"` (aliased as [`SelectList`])
//! - [`SelectItem`]: Individual item with `role="option"` (aliased as [`SelectOption`])
//! - [`SelectItemText`]: Display text wrapper — registers typeahead text and trigger display
//! - [`SelectItemIndicator`]: Visual indicator for selected items
//! - [`SelectGroup`]: Grouping element with `role="group"`
//! - [`SelectLabel`]: Non-interactive label (aliased as [`SelectGroupLabel`])
//! - [`SelectSeparator`]: Visual separator
//!
//! ## Features
//!
//! - **Keyboard Navigation**: Full keyboard support with arrow keys, home/end, enter, and escape
//! - **Typeahead Search**: Smart text search that adapts to different keyboard layouts
//! - **Accessibility**: ARIA compliant with proper roles and attributes
//! - **Customizable**: Flexible styling through data attributes and CSS
//! - **Focus Management**: Automatic focus handling and restoration

// Internal modules
mod components;
mod context;
pub(crate) mod text_search;

// Re-export all public components and types
pub use components::{
    // Primary Radix-aligned names
    Select,
    SelectContent,
    SelectContentProps,
    SelectGroup,
    // Backward-compatible aliases
    SelectGroupLabel,
    SelectGroupLabelProps,
    SelectGroupProps,
    SelectItem,
    SelectItemIndicator,
    SelectItemIndicatorProps,
    SelectItemProps,
    SelectItemText,
    SelectItemTextProps,
    SelectLabel,
    SelectLabelProps,
    SelectList,
    SelectListProps,
    SelectOption,
    SelectOptionProps,
    SelectProps,
    SelectSeparator,
    SelectSeparatorProps,
    SelectTrigger,
    SelectTriggerProps,
    SelectValue,
    SelectValueProps,
};
