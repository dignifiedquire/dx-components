use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use unic_langid::langid;

mod components;
mod layouts;
mod pages;
mod theme;
mod ui;

use layouts::app_layout::AppLayout;
use layouts::docs_layout::DocsLayout;
use layouts::home_layout::HomeLayout;
use pages::block_demo::ComponentBlockDemo;
use pages::component_page::ComponentPage;
use pages::home::Home;

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum ComponentType {
    /// Normal componet as default.
    Normal,
    /// Component that render the preview inside an iframe for isolation.
    Block,
}

#[derive(Clone, PartialEq)]
pub(crate) struct ComponentDemoData {
    pub(crate) name: &'static str,
    pub(crate) description: &'static str,
    pub(crate) r#type: ComponentType,
    pub(crate) docs: &'static str,
    pub(crate) component: HighlightedCode,
    pub(crate) style: HighlightedCode,
    pub(crate) variants: &'static [ComponentVariantDemoData],
}

#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Clone, PartialEq)]
pub(crate) struct ComponentVariantDemoData {
    pub(crate) name: &'static str,
    pub(crate) rs_highlighted: HighlightedCode,
    pub(crate) css_highlighted: Option<HighlightedCode>,
    pub(crate) component: fn() -> Element,
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) struct HighlightedCode {
    pub(crate) light: &'static str,
    pub(crate) dark: &'static str,
}

fn main() {
    dioxus::LaunchBuilder::new()
        // Set the server config only if we are building the server target
        .with_cfg(server_only! {
            ServeConfig::builder()
                // Enable incremental rendering
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        // Store static files in the public directory where other static assets like wasm are stored
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        // Don't clear the public folder on every build. The public folder has other files including the wasm
                        // binary and static assets required for the app to run
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
}

#[component]
fn App() -> Element {
    use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale((langid!("en-US"), include_str!("i18n/en-US.ftl")))
            .with_locale((langid!("fr-FR"), include_str!("i18n/fr-FR.ftl")))
            .with_locale((langid!("es-ES"), include_str!("i18n/es-ES.ftl")))
            .with_locale((langid!("de-DE"), include_str!("i18n/de-DE.ftl")))
    });

    rsx! {
        Router::<Route> {}
    }
}

#[derive(Routable, Clone, PartialEq)]
pub(crate) enum Route {
    #[layout(AppLayout)]
    #[layout(HomeLayout)]
    #[route("/")]
    Home {},
    #[end_layout]
    #[layout(DocsLayout)]
    #[route("/docs/components/:name")]
    ComponentPage { name: String },
    #[end_layout]
    #[route("/component/block/:name/:variant")]
    ComponentBlockDemo { name: String, variant: String },
}

impl Route {
    pub(crate) fn home() -> Self {
        Self::Home {}
    }

    pub(crate) fn component(name: impl ToString) -> Self {
        Self::ComponentPage {
            name: name.to_string(),
        }
    }
}

#[cfg(feature = "fullstack")]
#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    let mut routes = vec!["/".to_string()];
    for demo in components::DEMOS {
        routes.push(format!("/docs/components/{}", demo.name));
        if demo.r#type == ComponentType::Block {
            for variant in demo.variants {
                routes.push(format!("/component/block/{}/{}", demo.name, variant.name));
            }
        }
    }
    Ok(routes)
}

pub(crate) const THEME_CSS: HighlightedCode = HighlightedCode {
    light: include_str!(concat!(
        env!("OUT_DIR"),
        "/dx-components-theme.css.base16-ocean.light.html"
    )),
    dark: include_str!(concat!(
        env!("OUT_DIR"),
        "/dx-components-theme.css.base16-ocean.dark.html"
    )),
};
