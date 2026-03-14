//! DOM utility functions — port of `@floating-ui/utils/src/dom.ts`.
//!
//! Provides DOM traversal and inspection using `web-sys`.

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, Node, Window};

/// Get the window for a node.
/// Source: dom.ts `getWindow`
pub fn get_window(node: &Node) -> Window {
    node.owner_document()
        .and_then(|doc| doc.default_view())
        .or_else(web_sys::window)
        .expect("no window")
}

/// Get the document element for a node.
/// Source: dom.ts `getDocumentElement`
pub fn get_document_element(node: &Node) -> HtmlElement {
    let doc = if let Some(doc) = node.owner_document() {
        doc
    } else if let Some(doc) = node.dyn_ref::<web_sys::Document>() {
        doc.clone()
    } else {
        web_sys::window().unwrap().document().unwrap()
    };
    doc.document_element()
        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
        .unwrap_or_else(|| doc.body().unwrap())
}

/// Get node name in lowercase.
/// Source: dom.ts `getNodeName`
pub fn get_node_name(node: &Node) -> String {
    node.node_name().to_lowercase()
}

/// Check if a value is an Element.
/// Source: dom.ts `isElement`
pub fn is_element(node: &Node) -> bool {
    node.dyn_ref::<Element>().is_some()
}

/// Check if a value is an HTMLElement.
/// Source: dom.ts `isHTMLElement`
pub fn is_html_element(node: &Node) -> bool {
    node.dyn_ref::<HtmlElement>().is_some()
}

/// Check if an element is an overflow element (scroll container).
/// Source: dom.ts `isOverflowElement`
pub fn is_overflow_element(element: &Element) -> bool {
    let win = get_window(element.unchecked_ref());
    let Ok(Some(style)) = win.get_computed_style(element) else {
        return false;
    };

    let overflow = format!(
        "{}{}{}",
        style.get_property_value("overflow").unwrap_or_default(),
        style.get_property_value("overflow-y").unwrap_or_default(),
        style.get_property_value("overflow-x").unwrap_or_default(),
    );

    let display = style.get_property_value("display").unwrap_or_default();

    regex_lite_like_test(&overflow, "auto|scroll|overlay|hidden|clip")
        && display != "inline"
        && display != "contents"
}

fn regex_lite_like_test(haystack: &str, pattern: &str) -> bool {
    pattern.split('|').any(|p| haystack.contains(p))
}

/// Check if node is html, body, or #document.
/// Source: dom.ts `isLastTraversableNode`
pub fn is_last_traversable_node(node: &Node) -> bool {
    let name = get_node_name(node);
    matches!(name.as_str(), "html" | "body" | "#document")
}

/// Get the computed style of an element.
/// Source: dom.ts `getComputedStyle`
pub fn get_computed_style(element: &Element) -> Option<web_sys::CssStyleDeclaration> {
    let win = get_window(element.unchecked_ref());
    win.get_computed_style(element).ok().flatten()
}

/// Get scroll position of an element or window.
/// Source: dom.ts `getNodeScroll`
pub fn get_node_scroll(node: &Node) -> (f64, f64) {
    if let Some(el) = node.dyn_ref::<Element>() {
        (el.scroll_left() as f64, el.scroll_top() as f64)
    } else if let Some(win) = node.dyn_ref::<Window>() {
        (win.scroll_x().unwrap_or(0.0), win.scroll_y().unwrap_or(0.0))
    } else {
        (0.0, 0.0)
    }
}

/// Get the parent node, traversing shadow DOM.
/// Source: dom.ts `getParentNode`
pub fn get_parent_node(node: &Node) -> Option<Node> {
    let name = get_node_name(node);
    if name == "html" {
        return Some(node.clone());
    }

    // Check assignedSlot (slotted elements)
    if let Some(_el) = node.dyn_ref::<HtmlElement>() {
        // web-sys doesn't expose assignedSlot directly on all elements
        // TODO: add assignedSlot support when web-sys exposes it
    }

    node.parent_node().or_else(|| {
        // ShadowRoot → host
        node.dyn_ref::<web_sys::ShadowRoot>()
            .map(|sr| sr.host().unchecked_into())
    })
}

/// Get the nearest overflow ancestor.
/// Source: dom.ts `getNearestOverflowAncestor`
pub fn get_nearest_overflow_ancestor(node: &Node) -> Option<Node> {
    let parent = get_parent_node(node)?;

    if is_last_traversable_node(&parent) {
        return node.owner_document().map(|doc| {
            doc.body()
                .map(|b| b.unchecked_into::<Node>())
                .unwrap_or_else(|| doc.unchecked_into())
        });
    }

    if let Some(el) = parent.dyn_ref::<Element>() {
        if is_overflow_element(el) {
            return Some(parent);
        }
    }

    get_nearest_overflow_ancestor(&parent)
}

/// Get all overflow ancestors (scroll containers) up to the window.
/// Source: dom.ts `getOverflowAncestors`
pub fn get_overflow_ancestors(node: &Node) -> Vec<web_sys::EventTarget> {
    let mut list: Vec<web_sys::EventTarget> = Vec::new();

    let scrollable_ancestor = match get_nearest_overflow_ancestor(node) {
        Some(a) => a,
        None => return list,
    };

    let is_body = scrollable_ancestor
        .dyn_ref::<HtmlElement>()
        .map(|el| get_node_name(el.unchecked_ref()) == "body")
        .unwrap_or(false);

    if is_body {
        let win = get_window(&scrollable_ancestor);
        list.push(win.unchecked_into());
        // Also add the body itself if it's an overflow element
        if let Some(el) = scrollable_ancestor.dyn_ref::<Element>() {
            if is_overflow_element(el) {
                list.push(scrollable_ancestor.unchecked_into());
            }
        }
    } else {
        list.push(scrollable_ancestor.clone().unchecked_into());
        list.extend(get_overflow_ancestors(&scrollable_ancestor));
    }

    list
}

/// Check if an element is a containing block.
/// Source: dom.ts `isContainingBlock`
pub fn is_containing_block(element: &Element) -> bool {
    let Some(css) = get_computed_style(element) else {
        return false;
    };

    let is_not_none = |prop: &str| -> bool {
        let val = css.get_property_value(prop).unwrap_or_default();
        !val.is_empty() && val != "none"
    };

    is_not_none("transform")
        || is_not_none("translate")
        || is_not_none("scale")
        || is_not_none("rotate")
        || is_not_none("perspective")
        || is_not_none("filter")
        || is_not_none("backdrop-filter")
        || {
            // Source: dom.ts line 93: /transform|translate|scale|rotate|perspective|filter/
            let wc = css.get_property_value("will-change").unwrap_or_default();
            [
                "transform",
                "translate",
                "scale",
                "rotate",
                "perspective",
                "filter",
            ]
            .iter()
            .any(|&v| wc.contains(v))
        }
        || css
            .get_property_value("contain")
            .unwrap_or_default()
            .split_whitespace()
            .any(|v| matches!(v, "paint" | "layout" | "strict" | "content"))
}

/// Check if element is in the top layer (popover or modal).
/// Source: dom.ts `isTopLayer`
pub fn is_top_layer(element: &Element) -> bool {
    if let Ok(true) = element.matches(":popover-open") {
        return true;
    }
    element.matches(":modal").unwrap_or(false)
}
