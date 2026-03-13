// Forked from dioxus-web 0.7.3 src/history.rs (WebHistory only).
//
// Workaround: `format!("/{prefix}")` produces an empty string when compiled
// with `--wasm-split`. All `format!` calls that build paths are replaced with
// `String::push_str` / `String::push` equivalents. Lines that differ from
// upstream are marked with `// FIX(wasm-split)`.
//
// Upstream issue: https://github.com/DioxusLabs/dioxus/issues/XXXX
// Remove this file once the upstream fix lands.

use std::sync::Arc;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{window, Event, ScrollRestoration, Window};

/// A [`dioxus_history::History`] provider that integrates with a browser via the [History API].
///
/// Fork of `dioxus_web::WebHistory` that avoids `format!` for path construction
/// to work around a wasm-split code-splitting bug.
///
/// [History API]: https://developer.mozilla.org/en-US/docs/Web/API/History_API
pub struct FixedPrefixHistory {
    do_scroll_restoration: bool,
    history: web_sys::History,
    prefix: Option<String>,
    window: Window,
}

impl FixedPrefixHistory {
    pub fn new(prefix: Option<String>, do_scroll_restoration: bool) -> Self {
        let myself = Self::new_inner(prefix, do_scroll_restoration);

        let current_route = dioxus::history::History::current_route(&myself);
        let prefix_str = myself.prefix.as_deref().unwrap_or("");
        // FIX(wasm-split): was `format!("{prefix_str}{current_route_str}")`
        let mut current_url = String::with_capacity(prefix_str.len() + current_route.len());
        current_url.push_str(prefix_str);
        current_url.push_str(&current_route);
        let state = myself.create_state();
        let _ = replace_state_with_url(&myself.history, &state, Some(&current_url));

        myself
    }

    fn new_inner(prefix: Option<String>, do_scroll_restoration: bool) -> Self {
        let window = window().expect("access to `window`");
        let history = window.history().expect("`window` has access to `history`");

        if do_scroll_restoration {
            history
                .set_scroll_restoration(ScrollRestoration::Manual)
                .expect("`history` can set scroll restoration");
        }

        let prefix = prefix
            // FIX(wasm-split): upstream uses `.or_else(dioxus_cli_config::web_base_path)`
            // but the caller already provides the base path, so no fallback needed.
            .as_ref()
            .map(|prefix| prefix.trim_matches('/'))
            .filter(|prefix| !prefix.is_empty())
            // FIX(wasm-split): was `.map(|prefix| format!("/{prefix}"))`
            .map(|prefix| {
                let mut s = String::with_capacity(prefix.len() + 1);
                s.push('/');
                s.push_str(prefix);
                s
            });

        Self {
            do_scroll_restoration,
            history,
            prefix,
            window,
        }
    }

    fn scroll_pos(&self) -> ScrollPosition {
        if self.do_scroll_restoration {
            ScrollPosition::of_window(&self.window)
        } else {
            Default::default()
        }
    }

    fn create_state(&self) -> [f64; 2] {
        let scroll = self.scroll_pos();
        [scroll.x, scroll.y]
    }

    fn handle_nav(&self) {
        if self.do_scroll_restoration {
            self.window.scroll_to_with_x_and_y(0.0, 0.0)
        }
    }

    fn route_from_location(&self) -> String {
        let location = self.window.location();
        let path = location.pathname().unwrap_or_else(|_| "/".into())
            + &location.search().unwrap_or("".into())
            + &location.hash().unwrap_or("".into());
        let mut path = match self.prefix {
            None => &path,
            Some(ref prefix) => path.strip_prefix(prefix).unwrap_or(prefix),
        };
        // If the path is empty, parse the root route instead
        if path.is_empty() {
            path = "/"
        }
        path.to_string()
    }

    fn full_path(&self, state: &String) -> String {
        match &self.prefix {
            None => state.to_string(),
            // FIX(wasm-split): was `format!("{prefix}{state}")`
            Some(prefix) => {
                let mut s = String::with_capacity(prefix.len() + state.len());
                s.push_str(prefix);
                s.push_str(state);
                s
            }
        }
    }
}

impl dioxus::history::History for FixedPrefixHistory {
    fn current_route(&self) -> String {
        self.route_from_location()
    }

    fn current_prefix(&self) -> Option<String> {
        self.prefix.clone()
    }

    fn go_back(&self) {
        let _ = self.history.back();
    }

    fn go_forward(&self) {
        let _ = self.history.forward();
    }

    fn push(&self, state: String) {
        if state == self.current_route() {
            // don't push the same state twice
            return;
        }

        let w = window().expect("access to `window`");
        let h = w.history().expect("`window` has access to `history`");

        // update the scroll position before pushing the new state
        update_scroll(&w, &h);

        if push_state_and_url(&self.history, &self.create_state(), self.full_path(&state)).is_ok() {
            self.handle_nav();
        }
    }

    fn replace(&self, state: String) {
        if replace_state_with_url(
            &self.history,
            &self.create_state(),
            Some(&self.full_path(&state)),
        )
        .is_ok()
        {
            self.handle_nav();
        }
    }

    fn external(&self, url: String) -> bool {
        self.window.location().set_href(&url).is_ok()
    }

    fn updater(&self, callback: Arc<dyn Fn() + Send + Sync>) {
        let w = self.window.clone();
        let h = self.history.clone();
        let d = self.do_scroll_restoration;

        let function = Closure::wrap(Box::new(move |_| {
            (*callback)();
            if d {
                if let Some([x, y]) = get_current(&h) {
                    ScrollPosition { x, y }.scroll_to(w.clone())
                }
            }
        }) as Box<dyn FnMut(Event)>);
        self.window
            .add_event_listener_with_callback(
                "popstate",
                &function.into_js_value().unchecked_into(),
            )
            .unwrap();
    }
}

// --- Helper types and functions (identical to upstream) ---

#[derive(Clone, Copy, Debug, Default)]
struct ScrollPosition {
    x: f64,
    y: f64,
}

impl ScrollPosition {
    fn of_window(window: &Window) -> Self {
        Self {
            x: window.scroll_x().unwrap_or_default(),
            y: window.scroll_y().unwrap_or_default(),
        }
    }

    fn scroll_to(&self, window: Window) {
        let Self { x, y } = *self;
        let f = Closure::wrap(
            Box::new(move || window.scroll_to_with_x_and_y(x, y)) as Box<dyn FnMut()>
        );
        web_sys::window()
            .expect("should be run in a context with a `Window` object (dioxus cannot be run from a web worker)")
            .request_animation_frame(&f.into_js_value().unchecked_into())
            .expect("should register `requestAnimationFrame` OK");
    }
}

fn replace_state_with_url(
    history: &web_sys::History,
    value: &[f64; 2],
    url: Option<&str>,
) -> Result<(), JsValue> {
    let position = js_sys::Array::new();
    position.push(&JsValue::from(value[0]));
    position.push(&JsValue::from(value[1]));
    history.replace_state_with_url(&position, "", url)
}

fn push_state_and_url(
    history: &web_sys::History,
    value: &[f64; 2],
    url: String,
) -> Result<(), JsValue> {
    let position = js_sys::Array::new();
    position.push(&JsValue::from(value[0]));
    position.push(&JsValue::from(value[1]));
    history.push_state_with_url(&position, "", Some(&url))
}

fn get_current(history: &web_sys::History) -> Option<[f64; 2]> {
    history.state().ok().and_then(|state| {
        let state = state.dyn_into::<js_sys::Array>().ok()?;
        let x = state.get(0).as_f64()?;
        let y = state.get(1).as_f64()?;
        Some([x, y])
    })
}

fn update_scroll(window: &Window, history: &web_sys::History) {
    let scroll = ScrollPosition::of_window(window);
    let _ = replace_state_with_url(history, &[scroll.x, scroll.y], None);
}
