//! Dioxus equivalent of Radix's `@radix-ui/react-slot`.
//!
//! Provides [`Slot`], [`Slottable`], and [`render_slot`] for the `asChild` pattern.
//!
//! ## Background
//!
//! In React, the `Slot` component merges a parent component's props onto a child
//! element via `React.cloneElement`, allowing the child to control the rendered
//! element type while inheriting behavior (ARIA, data attributes, event handlers)
//! from the parent.
//!
//! Dioxus VNodes are immutable — there is no `cloneElement`. Instead, the same
//! pattern is achieved in two ways:
//!
//! 1. **Callback-based** (`r#as` prop / [`render_slot`]): the component passes
//!    merged attributes to a user-provided callback that renders the custom element.
//! 2. **Component-based** ([`Slot`] + [`Slottable`]): the `Slot` component provides
//!    attributes via context; child components consume them with [`use_slot_attrs`].
//!
//! ## Merge rules (matching Radix's `mergeProps`)
//!
//! [`merge_attributes`] handles the merge with these rules:
//! - `class`: concatenated with spaces (both preserved)
//! - `style`: concatenated with `"; "` (both preserved)
//! - All other attributes: later list wins (user overrides component defaults)
//!
//! ## Event handlers
//!
//! In Dioxus, event handlers are type-erased [`ListenerCallback`] values at the
//! attribute level and cannot be composed generically (unlike React's plain
//! functions). Components handle event composition explicitly via separate props
//! or by wrapping handlers at the component level. See [`compose_callbacks`] and
//! [`compose_event_handlers`] in [`primitive`](crate::primitive) for utilities.
//!
//! ## Callback-based usage
//!
//! ```ignore
//! use dioxus_primitives::{merge_attributes, slot::render_slot};
//! use dioxus_attributes::attributes;
//!
//! #[component]
//! fn MyButton(props: MyButtonProps) -> Element {
//!     let base = attributes!(button {
//!         "data-slot": "my-button",
//!         onclick: move |_| { /* internal handler */ },
//!     });
//!     let merged = merge_attributes(vec![base, props.attributes]);
//!
//!     render_slot(props.r#as, merged, props.children, |attrs, children| {
//!         rsx! { button { ..attrs, {children} } }
//!     })
//! }
//!
//! // Default element:
//! rsx! { MyButton { "Click me" } }
//!
//! // Custom element (asChild):
//! rsx! {
//!     MyButton {
//!         r#as: move |attrs| rsx! { a { href: "/home", ..attrs, "Go home" } },
//!     }
//! }
//! ```
//!
//! ## Component-based usage
//!
//! ```ignore
//! use dioxus_primitives::slot::{Slot, Slottable, use_slot_attrs};
//!
//! #[component]
//! fn MyButton(props: MyButtonProps) -> Element {
//!     let base = attributes!(button { "data-slot": "my-button" });
//!     let merged = merge_attributes(vec![base, props.attributes]);
//!
//!     rsx! {
//!         Slot { ..merged,
//!             {props.icon_left}
//!             Slottable { {props.children} }
//!             {props.icon_right}
//!         }
//!     }
//! }
//! ```
//!
//! [`merge_attributes`]: crate::merge_attributes
//! [`ListenerCallback`]: dioxus_core::ListenerCallback
//! [`compose_callbacks`]: crate::primitive::compose_callbacks
//! [`compose_event_handlers`]: crate::primitive::compose_event_handlers

use dioxus::prelude::*;

/* -------------------------------------------------------------------------------------------------
 * Slot
 * -----------------------------------------------------------------------------------------------*/

/// Context carrying attributes from a [`Slot`] to its descendants.
#[derive(Clone)]
struct SlotContext(Vec<Attribute>);

/// Props for [`Slot`].
#[derive(Props, Clone, PartialEq)]
pub struct SlotProps {
    /// Attributes to forward to children via context.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children. Components among these can call [`use_slot_attrs`] to receive
    /// the forwarded attributes.
    pub children: Element,
}

/// Dioxus equivalent of Radix's `Slot` component.
///
/// Provides the slot's attributes to descendant components via context.
/// Descendants call [`use_slot_attrs`] to retrieve and spread the attributes
/// onto their rendered element.
///
/// This component does **not** render a wrapper element — it renders its
/// children directly, matching upstream's transparent rendering behavior.
#[component]
pub fn Slot(props: SlotProps) -> Element {
    use_context_provider(|| SlotContext(props.attributes));
    props.children
}

/// Retrieves attributes from the nearest ancestor [`Slot`].
///
/// Returns an empty `Vec` if no `Slot` ancestor exists, making it safe to
/// call unconditionally.
pub fn use_slot_attrs() -> Vec<Attribute> {
    try_consume_context::<SlotContext>()
        .map(|ctx| ctx.0)
        .unwrap_or_default()
}

/* -------------------------------------------------------------------------------------------------
 * Slottable
 * -----------------------------------------------------------------------------------------------*/

/// Props for [`Slottable`].
#[derive(Props, Clone, PartialEq)]
pub struct SlottableProps {
    /// The children to be treated as slottable content.
    pub children: Element,
}

/// Dioxus equivalent of Radix's `Slottable` component.
///
/// Marks its children as the "slottable" content within a component that uses
/// [`Slot`]. In React, this enables `Slot` to identify which portion of a
/// component's children should be forwarded through the slot vs. kept as
/// structural content (e.g., icons around a button label).
///
/// In Dioxus, components typically use explicit named props for structural
/// slots (e.g., `icon_left: Element`), so `Slottable` acts as a transparent
/// wrapper for API parity with upstream.
#[component]
pub fn Slottable(props: SlottableProps) -> Element {
    props.children
}

/* -------------------------------------------------------------------------------------------------
 * render_slot (callback-based dispatch)
 * -----------------------------------------------------------------------------------------------*/

/// Renders either a default element or delegates to an `r#as` callback.
///
/// This is the Dioxus equivalent of Radix's `asChild` dispatch logic, used
/// internally by components that support the `r#as` prop pattern.
///
/// - If `r#as` is `Some`, calls the callback with the merged attributes.
///   The callback is responsible for rendering an element and spreading the
///   attributes.
/// - If `r#as` is `None`, calls `default_render` with the attributes and
///   children.
pub fn render_slot(
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    attrs: Vec<Attribute>,
    children: Element,
    default_render: impl FnOnce(Vec<Attribute>, Element) -> Element,
) -> Element {
    if let Some(dynamic) = r#as {
        dynamic.call(attrs)
    } else {
        default_render(attrs, children)
    }
}

/* -------------------------------------------------------------------------------------------------
 * Tests
 * -----------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // render_slot tests
    // -----------------------------------------------------------------------

    #[test]
    fn render_slot_uses_default_when_no_callback() {
        let mut dom = VirtualDom::new(|| {
            let attrs = vec![Attribute {
                name: "data-test",
                namespace: None,
                volatile: false,
                value: dioxus_core::AttributeValue::Text("hello".into()),
            }];

            render_slot(None, attrs, rsx! { "child" }, |attrs, children| {
                rsx! { button { ..attrs, {children} } }
            })
        });
        dom.rebuild(&mut dioxus_core::NoOpMutations);
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("<button"), "should render a button element");
        assert!(
            html.contains("data-test=\"hello\""),
            "should have data-test attr"
        );
        assert!(html.contains("child"), "should contain children");
    }

    #[test]
    fn render_slot_uses_callback_when_provided() {
        let mut dom = VirtualDom::new(|| {
            let callback = Callback::new(|attrs: Vec<Attribute>| {
                rsx! { a { ..attrs, "custom" } }
            });

            render_slot(
                Some(callback),
                vec![],
                rsx! { "ignored" },
                |attrs, children| {
                    rsx! { button { ..attrs, {children} } }
                },
            )
        });
        dom.rebuild(&mut dioxus_core::NoOpMutations);
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("<a"), "should render an anchor element");
        assert!(html.contains("custom"), "should contain callback content");
        assert!(
            !html.contains("ignored"),
            "should not contain default children"
        );
    }

    #[test]
    fn render_slot_forwards_attributes_to_callback() {
        let mut dom = VirtualDom::new(|| {
            let attrs = vec![Attribute {
                name: "role",
                namespace: None,
                volatile: false,
                value: dioxus_core::AttributeValue::Text("menu".into()),
            }];

            let callback = Callback::new(|attrs: Vec<Attribute>| {
                rsx! { div { ..attrs, "content" } }
            });

            render_slot(Some(callback), attrs, rsx! {}, |attrs, children| {
                rsx! { span { ..attrs, {children} } }
            })
        });
        dom.rebuild(&mut dioxus_core::NoOpMutations);
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("<div"), "should render a div via callback");
        assert!(html.contains("role=\"menu\""), "should forward role attr");
    }

    // -----------------------------------------------------------------------
    // Slot component tests
    // -----------------------------------------------------------------------

    #[test]
    fn slot_provides_attrs_via_context() {
        /// A child component that consumes slot attrs.
        #[component]
        fn SlotChild() -> Element {
            let attrs = use_slot_attrs();
            rsx! { span { ..attrs, "slotted" } }
        }

        let mut dom = VirtualDom::new(|| {
            rsx! {
                Slot { "data-test": "from-slot",
                    SlotChild {}
                }
            }
        });
        dom.rebuild(&mut dioxus_core::NoOpMutations);
        let html = dioxus_ssr::render(&dom);
        assert!(
            html.contains("data-test=\"from-slot\""),
            "child should have slot attrs"
        );
        assert!(html.contains("slotted"), "child content should render");
    }

    #[test]
    fn slot_renders_no_wrapper_element() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Slot {
                    span { "direct child" }
                }
            }
        });
        dom.rebuild(&mut dioxus_core::NoOpMutations);
        let html = dioxus_ssr::render(&dom);
        // Slot should not add its own wrapper — the span should be the root element
        assert!(
            html.starts_with("<span"),
            "Slot should not add a wrapper element"
        );
    }

    #[test]
    fn use_slot_attrs_returns_empty_without_slot() {
        #[component]
        fn NoSlotChild() -> Element {
            let attrs = use_slot_attrs();
            assert!(
                attrs.is_empty(),
                "should return empty vec without Slot ancestor"
            );
            rsx! { div { "no slot" } }
        }

        let mut dom = VirtualDom::new(|| {
            rsx! { NoSlotChild {} }
        });
        dom.rebuild(&mut dioxus_core::NoOpMutations);
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("no slot"));
    }

    // -----------------------------------------------------------------------
    // Slottable component tests
    // -----------------------------------------------------------------------

    #[test]
    fn slottable_renders_children_transparently() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    Slottable {
                        span { "slottable content" }
                    }
                }
            }
        });
        dom.rebuild(&mut dioxus_core::NoOpMutations);
        let html = dioxus_ssr::render(&dom);
        assert!(
            html.contains("slottable content"),
            "Slottable should render its children"
        );
        // Should not add any extra wrapper
        assert!(
            html.contains("<div><span>slottable content</span></div>"),
            "Slottable should not add wrapper elements"
        );
    }

    // -----------------------------------------------------------------------
    // Integration: Slot + Slottable together
    // -----------------------------------------------------------------------

    #[test]
    fn slot_with_slottable_and_structural_children() {
        /// A button component that uses Slot + Slottable.
        #[component]
        fn SlotButton(icon_left: Element, icon_right: Element, children: Element) -> Element {
            rsx! {
                Slot { "data-slot": "button",
                    {icon_left}
                    Slottable { {children} }
                    {icon_right}
                }
            }
        }

        let mut dom = VirtualDom::new(|| {
            rsx! {
                SlotButton {
                    icon_left: rsx! { span { class: "left", "L" } },
                    icon_right: rsx! { span { class: "right", "R" } },
                    "Button text"
                }
            }
        });
        dom.rebuild(&mut dioxus_core::NoOpMutations);
        let html = dioxus_ssr::render(&dom);
        // All three parts should render in order
        assert!(html.contains("L"), "icon_left should render");
        assert!(
            html.contains("Button text"),
            "slottable content should render"
        );
        assert!(html.contains("R"), "icon_right should render");
        // Verify order: left before text before right
        let left_pos = html.find("L").unwrap();
        let text_pos = html.find("Button text").unwrap();
        let right_pos = html.find("R").unwrap();
        assert!(left_pos < text_pos, "icon_left should be before text");
        assert!(text_pos < right_pos, "text should be before icon_right");
    }

    // -----------------------------------------------------------------------
    // Upstream parity: "given a Button with Slottable" snapshot tests
    // (slot.test.tsx L114-141)
    // -----------------------------------------------------------------------

    /// Test helper: a Button component with icon slots using Slottable,
    /// matching upstream's test fixture.
    #[component]
    fn TestButton(
        #[props(default)] as_child: Option<Callback<Vec<Attribute>, Element>>,
        #[props(default)] icon_left: Option<Element>,
        #[props(default)] icon_right: Option<Element>,
        #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
        children: Element,
    ) -> Element {
        render_slot(as_child, attributes, children, |attrs, children| {
            rsx! {
                button { ..attrs,
                    {icon_left}
                    Slottable { {children} }
                    {icon_right}
                }
            }
        })
    }

    /// Upstream: "without asChild — should render a button with icon on the left/right"
    #[test]
    fn button_without_as_child_renders_button_with_icons() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                TestButton {
                    icon_left: rsx! { span { "left" } },
                    icon_right: rsx! { span { "right" } },
                    "Button "
                    em { "text" }
                }
            }
        });
        dom.rebuild(&mut dioxus_core::NoOpMutations);
        let html = dioxus_ssr::render(&dom);
        // Should render as <button>
        assert!(html.contains("<button"), "should render a button");
        // Should have icon left, content, icon right in order
        assert!(html.contains("<span>left</span>"), "icon_left present");
        assert!(html.contains("<span>right</span>"), "icon_right present");
        assert!(html.contains("<em>text</em>"), "emphasized text present");
        let left_pos = html.find("left").unwrap();
        let text_pos = html.find("text").unwrap();
        let right_pos = html.find("right").unwrap();
        assert!(left_pos < text_pos, "left before text");
        assert!(text_pos < right_pos, "text before right");
    }

    /// Upstream: "with asChild — should render a link with icon on the left/right"
    #[test]
    fn button_with_as_child_renders_custom_element() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                TestButton {
                    as_child: move |attrs: Vec<Attribute>| {
                        rsx! { a { href: "https://radix-ui.com", ..attrs, "Link text" } }
                    },
                    icon_left: rsx! { span { "left" } },
                    icon_right: rsx! { span { "right" } },
                    "Button "
                    em { "text" }
                }
            }
        });
        dom.rebuild(&mut dioxus_core::NoOpMutations);
        let html = dioxus_ssr::render(&dom);
        // Should render as <a>, NOT <button>
        assert!(html.contains("<a"), "should render an anchor");
        assert!(!html.contains("<button"), "should NOT render a button");
        assert!(
            html.contains("href=\"https://radix-ui.com\""),
            "should have href"
        );
        assert!(html.contains("Link text"), "callback content renders");
    }

    /// Upstream: "with onClick on itself" — the handler is forwarded via attrs
    /// (SSR can verify the attribute is present; click behavior requires Playwright)
    #[test]
    fn render_slot_callback_receives_all_attributes() {
        let mut dom = VirtualDom::new(|| {
            let attrs = vec![
                Attribute {
                    name: "data-state",
                    namespace: None,
                    volatile: false,
                    value: dioxus_core::AttributeValue::Text("open".into()),
                },
                Attribute {
                    name: "aria-expanded",
                    namespace: None,
                    volatile: false,
                    value: dioxus_core::AttributeValue::Text("true".into()),
                },
            ];

            let callback = Callback::new(|attrs: Vec<Attribute>| {
                rsx! { button { r#type: "button", ..attrs, "Click me" } }
            });

            render_slot(Some(callback), attrs, rsx! {}, |attrs, children| {
                rsx! { div { ..attrs, {children} } }
            })
        });
        dom.rebuild(&mut dioxus_core::NoOpMutations);
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("<button"), "renders via callback");
        assert!(html.contains("data-state=\"open\""), "data-state forwarded");
        assert!(
            html.contains("aria-expanded=\"true\""),
            "aria-expanded forwarded"
        );
        assert!(
            html.contains("type=\"button\""),
            "callback's own attrs present"
        );
    }
}
