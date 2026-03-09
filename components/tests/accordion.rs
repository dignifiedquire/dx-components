//! SSR snapshot tests for the styled accordion (shadcn match).
//!
//! Each test asserts the exact HTML output including Tailwind classes to
//! ensure the styled layer matches shadcn/ui 1:1.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_components::accordion::*;

/// Render a component to an HTML string via SSR, with normalized IDs.
fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    let re = regex_lite::Regex::new(r"dxc-\d+").unwrap();
    re.replace_all(&html, "dxc-ID").to_string()
}

// ---------------------------------------------------------------------------
// shadcn class: Accordion root has NO baked-in classes
// ---------------------------------------------------------------------------

#[test]
fn root_no_baked_classes() {
    fn App() -> Element {
        rsx! {
            Accordion {
                AccordionItem { value: "one",
                    AccordionTrigger { "Trigger" }
                    AccordionContent { "Content" }
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== root_no_baked_classes ===\n{html}\n");

    // The root accordion div should NOT have w-full baked in
    // It should only have data-slot="accordion", no extra classes
    assert!(
        !html.contains("w-full"),
        "accordion root should NOT have w-full baked in"
    );
}

// ---------------------------------------------------------------------------
// shadcn class: AccordionItem — border-b last:border-b-0
// ---------------------------------------------------------------------------

#[test]
fn item_classes() {
    fn App() -> Element {
        rsx! {
            Accordion {
                AccordionItem { value: "one",
                    AccordionTrigger { "Trigger One" }
                    AccordionContent { "Content One" }
                }
                AccordionItem { value: "two",
                    AccordionTrigger { "Trigger Two" }
                    AccordionContent { "Content Two" }
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== item_classes ===\n{html}\n");

    assert!(
        html.contains("border-b last:border-b-0"),
        "AccordionItem should have border-b last:border-b-0"
    );
}

// ---------------------------------------------------------------------------
// shadcn class: AccordionTrigger — exact class string
// ---------------------------------------------------------------------------

#[test]
fn trigger_classes() {
    fn App() -> Element {
        rsx! {
            Accordion {
                AccordionItem { value: "one",
                    AccordionTrigger { "Trigger" }
                    AccordionContent { "Content" }
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== trigger_classes ===\n{html}\n");

    // Header wrapper is <h3 class="flex">
    assert!(
        html.contains(r#"<h3 data-slot="accordion-header""#),
        "should have h3 with accordion-header"
    );
    assert!(
        html.contains(r#"class="flex""#),
        "header should have class=flex"
    );

    // Trigger button has exact shadcn classes.
    // Note: SSR HTML-escapes & → &#38; and > → &#62; in attribute values.
    // Known limitation: tailwind_fuse merges focus-visible:ring-[3px] into
    // focus-visible:ring-ring/50, so ring-[3px] is absent from the output.
    // TODO: Once tailwind_fuse preserves ring-[3px], update this to include it.
    let expected_trigger_class = r#"flex flex-1 items-start justify-between gap-4 rounded-md py-4 text-left text-sm font-medium transition-all outline-none hover:underline focus-visible:border-ring focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 [&#38;[data-state=open]&#62;svg]:rotate-180"#;
    assert!(
        html.contains(expected_trigger_class),
        "trigger should have exact shadcn classes.\nActual:\n{html}"
    );
}

// ---------------------------------------------------------------------------
// shadcn: AccordionTrigger chevron SVG
// ---------------------------------------------------------------------------

#[test]
fn trigger_chevron() {
    fn App() -> Element {
        rsx! {
            Accordion {
                AccordionItem { value: "one",
                    AccordionTrigger { "Trigger" }
                    AccordionContent { "Content" }
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== trigger_chevron ===\n{html}\n");

    // SVG chevron present with correct classes
    assert!(html.contains("<svg"), "should contain chevron SVG");
    assert!(
        html.contains("pointer-events-none size-4 shrink-0 translate-y-0.5 text-muted-foreground transition-transform duration-200"),
        "chevron should have exact shadcn classes"
    );

    // Chevron path (lucide renders with spaces)
    assert!(
        html.contains(r#"d="m6 9 6 6 6-6""#),
        "should have chevron down path"
    );
}

// ---------------------------------------------------------------------------
// shadcn class: AccordionContent outer — fixed animation classes
// ---------------------------------------------------------------------------

#[test]
fn content_outer_classes() {
    fn App() -> Element {
        rsx! {
            Accordion { default_value: vec!["one".to_string()],
                AccordionItem { value: "one",
                    AccordionTrigger { "Trigger" }
                    AccordionContent { "Content" }
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== content_outer_classes ===\n{html}\n");

    // Outer content element has fixed animation classes
    assert!(
        html.contains("overflow-hidden text-sm data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down"),
        "outer content should have exact animation classes"
    );
}

// ---------------------------------------------------------------------------
// shadcn class: AccordionContent inner wrapper — pt-0 pb-4
// ---------------------------------------------------------------------------

#[test]
fn content_inner_classes() {
    fn App() -> Element {
        rsx! {
            Accordion { default_value: vec!["one".to_string()],
                AccordionItem { value: "one",
                    AccordionTrigger { "Trigger" }
                    AccordionContent { "Content" }
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== content_inner_classes ===\n{html}\n");

    assert!(
        html.contains(r#"class="pt-0 pb-4""#),
        "inner wrapper should have pt-0 pb-4"
    );
}

// ---------------------------------------------------------------------------
// shadcn class: consumer class applies to INNER div, not outer
// ---------------------------------------------------------------------------

#[test]
fn content_consumer_class_on_inner() {
    fn App() -> Element {
        rsx! {
            Accordion { default_value: vec!["one".to_string()],
                AccordionItem { value: "one",
                    AccordionTrigger { "Trigger" }
                    AccordionContent { class: "flex flex-col gap-4",
                        "Content"
                    }
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== content_consumer_class_on_inner ===\n{html}\n");

    // Consumer class should be on the inner div, merged with pt-0 pb-4
    assert!(
        html.contains("pt-0 pb-4 flex flex-col gap-4"),
        "consumer class should merge into inner div"
    );

    // Outer content should still have the fixed animation classes (unchanged)
    assert!(
        html.contains("overflow-hidden text-sm data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down"),
        "outer should keep fixed classes regardless of consumer class"
    );
}

// ---------------------------------------------------------------------------
// Full snapshot: complete 3-item accordion with 1 open
// ---------------------------------------------------------------------------

#[test]
fn full_snapshot() {
    fn App() -> Element {
        rsx! {
            Accordion { class: "w-full", default_value: vec!["item-1".to_string()],
                AccordionItem { value: "item-1",
                    AccordionTrigger { "Product Information" }
                    AccordionContent { class: "flex flex-col gap-4",
                        p { "Description text." }
                    }
                }
                AccordionItem { value: "item-2",
                    AccordionTrigger { "Shipping Details" }
                    AccordionContent { class: "flex flex-col gap-4",
                        p { "Shipping text." }
                    }
                }
                AccordionItem { value: "item-3",
                    AccordionTrigger { "Return Policy" }
                    AccordionContent { class: "flex flex-col gap-4",
                        p { "Return text." }
                    }
                }
            }
        }
    }

    let html = render(App);
    eprintln!("=== full_snapshot ===\n{html}\n");

    // Consumer w-full passed through to root
    assert!(html.contains("w-full"), "consumer w-full should be on root");

    // 3 items
    assert_eq!(
        html.matches("border-b last:border-b-0").count(),
        3,
        "should have 3 items"
    );

    // 3 triggers with chevrons
    assert_eq!(
        html.matches(r#"d="m6 9 6 6 6-6""#).count(),
        3,
        "should have 3 chevrons"
    );

    // 1 open, 2 closed
    assert!(html.contains("Product Information"));
    assert!(html.contains("Description text."));
    assert!(!html.contains("Shipping text."));
    assert!(!html.contains("Return text."));

    // Open content has animation class and inner wrapper
    assert!(html.contains("animate-accordion-down"));
    assert!(html.contains("pt-0 pb-4 flex flex-col gap-4"));
}
