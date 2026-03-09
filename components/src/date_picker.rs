//! Styled date picker.
//!
//! Wraps the unstyled `dioxus_primitives::date_picker` primitives and includes
//! a CSS stylesheet that targets `data-slot` attributes.

use dioxus::prelude::*;

pub use dioxus_primitives::date_picker::{
    DatePicker as DatePickerPrimitive, DatePickerProps, DateRangePicker, DateRangePickerProps,
};

/// Styled DatePicker — includes CSS for data-slot styling.
///
/// All sub-components are re-exported from the primitive and styled via
/// CSS `[data-slot]` selectors.
#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./date_picker.css") }
        DatePickerPrimitive { ..props }
    }
}
