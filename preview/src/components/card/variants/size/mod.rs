use super::super::component::*;
use crate::components::button::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Card { size: CardSize::Sm, class: "mx-auto w-full max-w-sm",
            CardHeader {
                CardTitle { "Small Card" }
                CardDescription { "This card uses the small size variant." }
            }
            CardContent {
                p {
                    "The card component supports a "
                    code { "size" }
                    " prop that can be set to "
                    code { "Sm" }
                    " for a more compact appearance."
                }
            }
            CardFooter {
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    class: "w-full",
                    "Action"
                }
            }
        }
    }
}
