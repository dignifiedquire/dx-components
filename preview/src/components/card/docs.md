A flexible container for grouping related content and actions. Composes from seven sub-components — `CardHeader`, `CardTitle`, `CardDescription`, `CardAction`, `CardContent`, `CardFooter` — that wire up the layout grid for you, including a two-column header layout when a `CardAction` is present.

Pass `size: CardSize::Sm` for a denser variant — gaps and paddings shrink and the title font drops one step. The `data-size="sm"` attribute also cascades to nested parts via Tailwind's `group-data-[size=sm]/card:` selector so headers, content, and footer all adapt together.

```rust
use dioxus::prelude::*;
use dioxus_components::card::*;
use dioxus_components::button::{Button, ButtonVariant};

#[component]
fn LoginCard() -> Element {
    rsx! {
        Card { class: "w-full max-w-sm",
            CardHeader {
                CardTitle { "Login to your account" }
                CardDescription { "Enter your email below to login to your account" }
                CardAction {
                    Button { variant: ButtonVariant::Link, "Sign Up" }
                }
            }
            CardContent {
                // …form fields…
            }
            CardFooter { class: "flex-col gap-2",
                Button { r#type: "submit", class: "w-full", "Login" }
            }
        }
    }
}
```

A first-child `<img>` is treated specially — it sits flush against the card's rounded top corners because `Card` includes the `*:[img:first-child]:rounded-t-xl` and `has-[>img:first-child]:pt-0` Tailwind rules.
