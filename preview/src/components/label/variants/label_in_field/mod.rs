use crate::components::button::component::{Button, ButtonVariant};
use crate::components::checkbox::component::Checkbox;
use crate::components::input::component::Input;
use crate::components::select::component::*;
use crate::components::textarea::component::Textarea;
use dioxus::prelude::*;
use dioxus_components::checkbox::CheckedState;
use dioxus_components::field::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            "data-testid": "label-in-field-demo",
            class: "w-full max-w-md",
            form {
                FieldGroup {
                    FieldSet {
                        FieldLegend { "Payment Method" }
                        FieldDescription {
                            "All transactions are secure and encrypted"
                        }
                        FieldGroup {
                            Field {
                                FieldLabel { html_for: "checkout-card-name",
                                    "Name on Card"
                                }
                                Input {
                                    id: "checkout-card-name",
                                    placeholder: "Evil Rabbit",
                                }
                            }
                            Field {
                                FieldLabel { html_for: "checkout-card-number",
                                    "Card Number"
                                }
                                Input {
                                    id: "checkout-card-number",
                                    placeholder: "1234 5678 9012 3456",
                                }
                                FieldDescription {
                                    "Enter your 16-digit card number"
                                }
                            }
                            div { class: "grid grid-cols-3 gap-4",
                                Field {
                                    FieldLabel { html_for: "checkout-exp-month",
                                        "Month"
                                    }
                                    Select { placeholder: "MM",
                                        SelectTrigger { id: "checkout-exp-month", aria_label: "Month",
                                            SelectValue {}
                                        }
                                        SelectContent { aria_label: "Expiry Month",
                                            SelectGroup {
                                                SelectItem { value: "01", text_value: "01", "01" }
                                                SelectItem { value: "02", text_value: "02", "02" }
                                                SelectItem { value: "03", text_value: "03", "03" }
                                                SelectItem { value: "04", text_value: "04", "04" }
                                                SelectItem { value: "05", text_value: "05", "05" }
                                                SelectItem { value: "06", text_value: "06", "06" }
                                                SelectItem { value: "07", text_value: "07", "07" }
                                                SelectItem { value: "08", text_value: "08", "08" }
                                                SelectItem { value: "09", text_value: "09", "09" }
                                                SelectItem { value: "10", text_value: "10", "10" }
                                                SelectItem { value: "11", text_value: "11", "11" }
                                                SelectItem { value: "12", text_value: "12", "12" }
                                            }
                                        }
                                    }
                                }
                                Field {
                                    FieldLabel { html_for: "checkout-exp-year",
                                        "Year"
                                    }
                                    Select { placeholder: "YYYY",
                                        SelectTrigger { id: "checkout-exp-year", aria_label: "Year",
                                            SelectValue {}
                                        }
                                        SelectContent { aria_label: "Expiry Year",
                                            SelectGroup {
                                                SelectItem { value: "2024", text_value: "2024", "2024" }
                                                SelectItem { value: "2025", text_value: "2025", "2025" }
                                                SelectItem { value: "2026", text_value: "2026", "2026" }
                                                SelectItem { value: "2027", text_value: "2027", "2027" }
                                                SelectItem { value: "2028", text_value: "2028", "2028" }
                                                SelectItem { value: "2029", text_value: "2029", "2029" }
                                            }
                                        }
                                    }
                                }
                                Field {
                                    FieldLabel { html_for: "checkout-cvv", "CVV" }
                                    Input { id: "checkout-cvv", placeholder: "123" }
                                }
                            }
                        }
                    }
                    FieldSeparator {}
                    FieldSet {
                        FieldLegend { "Billing Address" }
                        FieldDescription {
                            "The billing address associated with your payment method"
                        }
                        FieldGroup {
                            Field { orientation: FieldOrientation::Horizontal,
                                Checkbox {
                                    id: "checkout-same-as-shipping",
                                    default_checked: CheckedState::Checked,
                                }
                                FieldLabel {
                                    html_for: "checkout-same-as-shipping",
                                    class: "font-normal",
                                    "Same as shipping address"
                                }
                            }
                        }
                    }
                    FieldSet {
                        FieldGroup {
                            Field {
                                FieldLabel { html_for: "checkout-comments",
                                    "Comments"
                                }
                                Textarea {
                                    id: "checkout-comments",
                                    placeholder: "Add any additional comments",
                                    class: "resize-none",
                                }
                            }
                        }
                    }
                    Field { orientation: FieldOrientation::Horizontal,
                        Button { r#type: "submit", "Submit" }
                        Button { variant: ButtonVariant::Outline, r#type: "button",
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}
