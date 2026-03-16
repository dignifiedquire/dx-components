//! Announce primitive — line-by-line port of `@radix-ui/react-announce`.
//!
//! Provides an ARIA live region for screen reader announcements.
//! Content rendered inside [`Announce`] is mirrored into a visually-hidden
//! `aria-live` region at `document.body` so assistive technologies can
//! announce changes.
//!
//! ## Upstream behavior
//!
//! - Creates global live region elements at `document.body` (one per unique
//!   combination of type/role/relevant/atomic/regionIdentifier)
//! - Reuses existing live regions via CSS attribute selectors
//! - Mirrors content via portal into the live region
//! - Handles `visibilitychange` to suppress announcements from inactive tabs
//! - Deduplicates visibility listeners across shared regions
//!
//! ## Differences from upstream
//!
//! - **Content mirroring**: Upstream uses `ReactDOM.createPortal` to mirror
//!   React children into the live region. We use `web_sys` to copy the inner
//!   HTML of the rendered element into the live region, since Dioxus cannot
//!   portal into arbitrary DOM nodes.
//! - **`useComposedRefs`**: Not needed — single ref model in Dioxus.

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// The urgency level of an announcement.
///
/// Upstream: `type RegionType = 'polite' | 'assertive' | 'off'`
/// Maps to the `aria-live` attribute value.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AnnounceType {
    /// Polite — announced at the next graceful opportunity (default).
    #[default]
    Polite,
    /// Assertive — announced immediately, interrupting current speech.
    Assertive,
    /// Off — not announced.
    Off,
}

impl AnnounceType {
    /// Returns the `aria-live` attribute value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Polite => "polite",
            Self::Assertive => "assertive",
            Self::Off => "off",
        }
    }

    /// Returns the default ARIA role for this announcement type.
    ///
    /// Upstream: `const ROLES: { [key in RegionType]: RegionRole }`
    pub fn default_role(&self) -> RegionRole {
        match self {
            Self::Polite => RegionRole::Status,
            Self::Assertive => RegionRole::Alert,
            Self::Off => RegionRole::None,
        }
    }
}

/// Role for the live region.
///
/// Upstream: `type RegionRole = 'status' | 'alert' | 'log' | 'none'`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegionRole {
    /// Status role (default for polite).
    Status,
    /// Alert role (default for assertive).
    Alert,
    /// Log role — for logging-style live regions.
    Log,
    /// No role (default for off).
    None,
}

impl RegionRole {
    /// Returns the role attribute value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::Alert => "alert",
            Self::Log => "log",
            Self::None => "none",
        }
    }
}

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

/// Props for [`Announce`].
#[derive(Props, Clone, PartialEq)]
pub struct AnnounceProps {
    /// The announcement urgency. Defaults to `Polite`.
    ///
    /// Upstream: `type?: RegionType` (default `'polite'`)
    #[props(default)]
    pub r#type: AnnounceType,

    /// ARIA role override. Defaults based on `type`.
    ///
    /// Upstream: `role?: RegionRole` (default `ROLES[type]`)
    #[props(default)]
    pub role: Option<RegionRole>,

    /// Whether assistive tech should present all or parts of the changed region.
    ///
    /// Upstream: `'aria-atomic'?: boolean`
    #[props(default)]
    pub aria_atomic: Option<bool>,

    /// What types of changes should be announced.
    ///
    /// Upstream: `'aria-relevant'?: PrimitiveDivProps['aria-relevant']`
    #[props(default)]
    pub aria_relevant: Option<String>,

    /// An optional unique identifier for the live region.
    ///
    /// By default, Announce components create at most two unique `aria-live`
    /// regions (one for polite, one for assertive). Passing an id creates a
    /// separate region for that identifier.
    ///
    /// Upstream: `regionIdentifier?: string`
    #[props(default)]
    pub region_identifier: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Spread attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (content to announce).
    pub children: Element,
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

/// ARIA live region for screen reader announcements.
///
/// Matches Radix's `Announce` component. Renders children inline and mirrors
/// them into a global live region at `document.body` for screen readers.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_primitives::announce::{Announce, AnnounceType};
/// rsx! {
///     Announce { r#type: AnnounceType::Assertive,
///         "Item deleted"
///     }
/// };
/// ```
#[component]
pub fn Announce(props: AnnounceProps) -> Element {
    let role = props.role.unwrap_or_else(|| props.r#type.default_role());
    let aria_atomic = props.aria_atomic.unwrap_or(false);

    // Generate a unique ID for the inline element so we can read its content.
    let inline_id = crate::use_unique_id();

    let announce_type = props.r#type;
    let relevant = props.aria_relevant.clone();

    // On wasm: create/find global live region and mirror content into it.
    #[cfg(target_arch = "wasm32")]
    {
        use std::cell::RefCell;
        use std::collections::HashMap;
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        let region_id = props.region_identifier.clone();

        // Upstream: const listenerMap = new Map<Element, number>();
        // Thread-local listener dedup map (wasm is single-threaded).
        thread_local! {
            static LISTENER_MAP: RefCell<HashMap<String, u32>> = RefCell::new(HashMap::new());
        }

        let role_str = role.as_str().to_string();
        let type_str = announce_type.as_str().to_string();
        let relevant_clone = relevant.clone();

        // Effect: create/find the live region and mirror content.
        crate::use_effect_with_cleanup(move || {
            let Some(window) = web_sys::window() else {
                return Box::new(|| {}) as Box<dyn FnOnce()>;
            };
            let Some(document) = window.document() else {
                return Box::new(|| {}) as Box<dyn FnOnce()>;
            };

            // Upstream: getLiveRegionElement()
            let data_attr = get_live_region_data_attr(region_id.as_deref());
            let selector = build_selector(
                &type_str,
                &role_str,
                relevant_clone.as_deref(),
                aria_atomic,
                &data_attr,
            );

            let region_element = if let Ok(Some(existing)) = document.query_selector(&selector) {
                existing
            } else {
                // Upstream: buildLiveRegionElement(ownerDocument, regionConfig)
                build_live_region_element(
                    &document,
                    &type_str,
                    &role_str,
                    relevant_clone.as_deref(),
                    aria_atomic,
                    &data_attr,
                )
            };

            // Mirror content from inline element into the live region.
            let id_val = inline_id.peek().clone();
            if let Some(inline_el) = document.get_element_by_id(&id_val) {
                // Upstream: ReactDOM.createPortal(<div>{children}</div>, region)
                // We mirror by creating a wrapper div with the innerHTML.
                let wrapper = document.create_element("div").unwrap();
                wrapper.set_inner_html(&inline_el.inner_html());
                region_element.set_inner_html(""); // Clear previous content
                let _ = region_element.append_child(&wrapper);
            }

            // Upstream: visibilitychange listener with dedup
            let region_key = selector.clone();
            let role_str_vc = role_str.clone();
            let type_str_vc = type_str.clone();
            let doc_for_vc = document.clone();
            let region_for_vc = region_element.clone();

            let needs_listener = LISTENER_MAP.with(|map| {
                let mut map = map.borrow_mut();
                if let Some(count) = map.get_mut(&region_key) {
                    *count += 1;
                    false // Listener already exists
                } else {
                    map.insert(region_key.clone(), 1);
                    true // Need new listener
                }
            });

            let closure = if needs_listener {
                // Upstream: document.addEventListener('visibilitychange', ...)
                let closure = Closure::wrap(Box::new(move || {
                    let hidden = doc_for_vc.hidden();
                    region_for_vc
                        .set_attribute("role", if hidden { "none" } else { &role_str_vc })
                        .ok();
                    region_for_vc
                        .set_attribute("aria-live", if hidden { "off" } else { &type_str_vc })
                        .ok();
                }) as Box<dyn FnMut()>);

                document
                    .add_event_listener_with_callback(
                        "visibilitychange",
                        closure.as_ref().unchecked_ref(),
                    )
                    .ok();

                Some(closure)
            } else {
                None
            };

            // Cleanup
            let region_key_cleanup = region_key;
            let doc_cleanup = document;
            Box::new(move || {
                let should_remove = LISTENER_MAP.with(|map| {
                    let mut map = map.borrow_mut();
                    if let Some(count) = map.get_mut(&region_key_cleanup) {
                        *count -= 1;
                        if *count == 0 {
                            map.remove(&region_key_cleanup);
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });

                if should_remove {
                    if let Some(ref closure) = closure {
                        doc_cleanup
                            .remove_event_listener_with_callback(
                                "visibilitychange",
                                closure.as_ref().unchecked_ref(),
                            )
                            .ok();
                    }
                }
                drop(closure);
            }) as Box<dyn FnOnce()>
        });
    }

    rsx! {
        div {
            id: inline_id(),
            "data-slot": "announce",
            role: role.as_str(),
            "aria-live": announce_type.as_str(),
            "aria-atomic": if aria_atomic { "true" } else { "false" },
            "aria-relevant": relevant,
            class: props.class,
            ..props.attributes,
            {props.children}
        }
    }
}

/// Upstream alias.
///
/// `const Root = Announce;`
pub use Announce as Root;

// ---------------------------------------------------------------------------
// Helpers (matching upstream utility functions)
// ---------------------------------------------------------------------------

/// Upstream: `getLiveRegionPartDataAttr(id?: string)`
#[cfg(target_arch = "wasm32")]
fn get_live_region_data_attr(id: Option<&str>) -> String {
    match id {
        Some(id) => format!("data-radix-announce-region-{id}"),
        None => "data-radix-announce-region".to_string(),
    }
}

/// Upstream: `buildSelector({ type, relevant, role, atomic, id })`
#[cfg(target_arch = "wasm32")]
fn build_selector(
    live_type: &str,
    role: &str,
    relevant: Option<&str>,
    atomic: bool,
    data_attr: &str,
) -> String {
    let mut sel = format!("[{data_attr}]");
    sel.push_str(&format!("[aria-live={live_type}]"));
    if atomic {
        sel.push_str("[aria-atomic=true]");
    }
    if let Some(r) = relevant {
        sel.push_str(&format!("[aria-relevant={r}]"));
    }
    sel.push_str(&format!("[role={role}]"));
    sel
}

/// Upstream: `buildLiveRegionElement(ownerDocument, { type, relevant, role, atomic, id })`
#[cfg(target_arch = "wasm32")]
fn build_live_region_element(
    document: &web_sys::Document,
    live_type: &str,
    role: &str,
    relevant: Option<&str>,
    atomic: bool,
    data_attr: &str,
) -> web_sys::Element {
    let element = document.create_element("div").unwrap();
    element.set_attribute(data_attr, "").ok();
    element
        .set_attribute(
            "style",
            "position: absolute; top: -1px; width: 1px; height: 1px; overflow: hidden;",
        )
        .ok();

    if let Some(body) = document.body() {
        body.append_child(&element).ok();
    }

    element.set_attribute("aria-live", live_type).ok();
    element
        .set_attribute("aria-atomic", if atomic { "true" } else { "false" })
        .ok();
    element.set_attribute("role", role).ok();
    if let Some(r) = relevant {
        element.set_attribute("aria-relevant", r).ok();
    }

    element
}
