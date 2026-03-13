//! Component definitions for the select primitive.

pub mod group;
pub mod item_text;
pub mod list;
pub mod option;
pub mod select;
pub mod trigger;
pub mod value;

// Primary exports (Radix-aligned names)
pub use group::{
    SelectGroup, SelectGroupLabel, SelectGroupLabelProps, SelectGroupProps, SelectLabel,
    SelectLabelProps, SelectSeparator, SelectSeparatorProps,
};
pub use item_text::{SelectItemText, SelectItemTextProps};
pub use list::{SelectContent, SelectContentProps, SelectList, SelectListProps};
pub use option::{
    SelectItem, SelectItemIndicator, SelectItemIndicatorProps, SelectItemProps, SelectOption,
    SelectOptionProps,
};
pub use select::{Select, SelectProps};
pub use trigger::{SelectTrigger, SelectTriggerProps};
pub use value::{SelectValue, SelectValueProps};
