use std::sync::OnceLock;

/// Information about a component discovered from its directory.
struct ComponentInfo {
    /// snake_case name, e.g. "alert_dialog"
    name: String,
    /// PascalCase page name, e.g. "AlertDialogPage"
    page_name: String,
    /// Variant directory names in sorted order. Always includes "main".
    variants: Vec<String>,
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::PathBuf::from(out_dir);
    println!("cargo:rerun-if-changed=src/components");

    let mut components = Vec::new();

    // Process all markdown files and highlight code files in each component folder
    for folder in std::fs::read_dir("src/components").unwrap().flatten() {
        if !folder.file_type().unwrap().is_dir() {
            continue;
        }
        let folder_path = folder.path();
        walk_highlight_dir(&folder_path, &out_dir).unwrap();

        // Extract description from component.json if it exists
        let folder_name = folder_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let json_path = folder_path.join("component.json");
        let description = if json_path.exists() {
            extract_json_description(&json_path)
        } else {
            String::new()
        };
        let desc_out = out_dir.join(&folder_name).join("description.txt");
        std::fs::write(desc_out, description).unwrap();

        // Discover variant directories
        let variants_dir = folder_path.join("variants");
        let mut variants = Vec::new();
        if variants_dir.is_dir() {
            for entry in std::fs::read_dir(&variants_dir).unwrap().flatten() {
                if entry.file_type().unwrap().is_dir() {
                    let vname = entry.file_name().to_string_lossy().to_string();
                    variants.push(vname);
                }
            }
        }
        // Ensure "main" is first, then sort the rest
        variants.sort();
        if let Some(pos) = variants.iter().position(|v| v == "main") {
            variants.remove(pos);
        }
        variants.insert(0, "main".to_string());

        let page_name = snake_to_pascal(&folder_name) + "Page";

        components.push(ComponentInfo {
            name: folder_name,
            page_name,
            variants,
        });
    }

    // Sort components by name for stable output
    components.sort_by(|a, b| a.name.cmp(&b.name));

    // Generate routes.rs
    let routes_rs = generate_routes(&components);
    std::fs::write(out_dir.join("routes.rs"), routes_rs).unwrap();

    // Process the main dx-components-theme.css file
    let theme_css_path = std::path::PathBuf::from("assets/dx-components-theme.css");
    for theme in ["base16-ocean.dark", "base16-ocean.light"] {
        let html = highlight_file_to(&theme_css_path, theme);
        let out_file_path = out_dir.join(format!("dx-components-theme.css.{theme}.html"));
        std::fs::write(out_file_path, html).unwrap();
    }
}

/// Convert snake_case to PascalCase: "alert_dialog" → "AlertDialog"
fn snake_to_pascal(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}

/// Generate the complete routes.rs file with Route enum, page functions, and helpers.
fn generate_routes(components: &[ComponentInfo]) -> String {
    use proc_macro2::{Ident, Span};
    use quote::quote;

    let ident = |s: &str| Ident::new(s, Span::call_site());

    // Route enum variants under DocsLayout
    let route_variants: Vec<_> = components
        .iter()
        .map(|c| {
            let route_path = format!("/docs/components/{}", c.name);
            let page = ident(&c.page_name);
            quote! {
                #[route(#route_path)]
                #page {},
            }
        })
        .collect();

    // Route::component() match arms
    let component_arms: Vec<_> = components
        .iter()
        .map(|c| {
            let name = &c.name;
            let page = ident(&c.page_name);
            quote! { #name => Self::#page {}, }
        })
        .collect();

    // Route::component_name() match arms
    let name_arms: Vec<_> = components
        .iter()
        .map(|c| {
            let name = &c.name;
            let page = ident(&c.page_name);
            quote! { Self::#page {} => Some(#name), }
        })
        .collect();

    // Per-component page functions
    let page_fns: Vec<_> = components
        .iter()
        .map(|c| {
            let page = ident(&c.page_name);
            let name = &c.name;
            let comp_mod = ident(&c.name);
            let demo_entries: Vec<_> = c
                .variants
                .iter()
                .map(|v| {
                    let variant_mod = ident(v);
                    quote! {
                        DemoEntry {
                            name: #v,
                            component: components::#comp_mod::variants::#variant_mod::Demo,
                        }
                    }
                })
                .collect();
            quote! {
                #[allow(unused_braces)]
                #[component]
                fn #page() -> Element {
                    rsx! {
                        ComponentPageInner {
                            name: #name,
                            demos: vec![#(#demo_entries),*],
                        }
                    }
                }
            }
        })
        .collect();

    // find_block_demo match arms
    let block_arms: Vec<_> = components
        .iter()
        .flat_map(|c| {
            let comp_mod = ident(&c.name);
            c.variants.iter().map(move |v| {
                let name = &c.name;
                let variant_mod = ident(v);
                quote! {
                    (#name, #v) => Some(components::#comp_mod::variants::#variant_mod::Demo),
                }
            })
        })
        .collect();

    let tokens = quote! {
        #[derive(Routable, Clone, PartialEq)]
        pub(crate) enum Route {
            #[layout(AppLayout)]
                #[layout(HomeLayout)]
                    #[route("/")]
                    Home {},
                #[end_layout]
                #[layout(DocsLayout)]
                    #(#route_variants)*
                #[end_layout]
            #[end_layout]
            #[route("/component/block/:name/:variant")]
            ComponentBlockDemo { name: String, variant: String },
        }

        impl Route {
            pub(crate) fn home() -> Self {
                Self::Home {}
            }

            pub(crate) fn component(name: impl AsRef<str>) -> Self {
                match name.as_ref() {
                    #(#component_arms)*
                    _ => Self::Home {},
                }
            }

            pub(crate) fn component_name(&self) -> Option<&'static str> {
                match self {
                    #(#name_arms)*
                    _ => None,
                }
            }
        }

        #(#page_fns)*

        /// Look up any component demo by name and variant.
        /// Used by ComponentBlockDemo for isolated rendering (Playwright tests, iframes).
        pub(crate) fn find_block_demo(name: &str, variant: &str) -> Option<fn() -> Element> {
            match (name, variant) {
                #(#block_arms)*
                _ => None,
            }
        }
    };

    tokens.to_string()
}

fn walk_highlight_dir(dir: &std::path::Path, out_dir: &std::path::Path) -> std::io::Result<()> {
    let folder_name = dir.file_name().unwrap();
    let folder_name = folder_name.to_string_lossy();
    let out_folder = out_dir.join(&*folder_name);
    std::fs::create_dir_all(&out_folder).unwrap();
    for file in std::fs::read_dir(dir).unwrap().flatten() {
        if file.file_type().unwrap().is_dir() {
            walk_highlight_dir(&file.path(), &out_folder)?;
            continue;
        }
        if file.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        if file.path().extension() == Some(std::ffi::OsStr::new("md")) {
            let markdown = process_markdown_to_html(&file.path());
            let out_file_path = out_folder.join(file.file_name()).with_extension("html");
            std::fs::write(out_file_path, markdown).unwrap();
            continue;
        }
        let file_name = file.file_name();
        let file_name = file_name.to_string_lossy();
        for theme in ["base16-ocean.dark", "base16-ocean.light"] {
            let html = highlight_file_to(&file.path(), theme);
            let out_file_path = out_folder.join(format!("{file_name}.{theme}.html"));
            std::fs::write(out_file_path, html).unwrap();
        }
    }
    Ok(())
}

fn highlight_file_to(file_path: &std::path::Path, theme: &str) -> String {
    println!("cargo:rerun-if-changed={}", file_path.display());
    use std::io::BufRead;
    use syntect::easy::HighlightFile;
    use syntect::highlighting::{Style, ThemeSet};
    use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
    use syntect::parsing::SyntaxSet;
    static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
    static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();
    let ss = SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines);
    let ts = THEME_SET.get_or_init(ThemeSet::load_defaults);
    let mut all_html = String::new();
    let mut highlighter = HighlightFile::new(file_path, ss, &ts.themes[theme]).unwrap();
    let mut line = String::new();
    while highlighter.reader.read_line(&mut line).unwrap_or_default() > 0 {
        {
            let regions: Vec<(Style, &str)> = highlighter
                .highlight_lines
                .highlight_line(&line, ss)
                .unwrap();
            let html =
                styled_line_to_highlighted_html(&regions[..], IncludeBackground::No).unwrap();
            all_html += "<span class=\"line\">";
            all_html += &html;
            all_html += "</span>";
        }
        line.clear();
    }
    all_html
}

/// Extract the "description" value from a component.json file without a JSON parser.
fn extract_json_description(path: &std::path::Path) -> String {
    let content = std::fs::read_to_string(path).unwrap_or_default();
    // Find "description": "..." pattern
    if let Some(start) = content.find("\"description\"") {
        let rest = &content[start + "\"description\"".len()..];
        // Skip whitespace and colon
        let rest = rest.trim_start().strip_prefix(':').unwrap_or(rest);
        let rest = rest.trim_start();
        // Extract quoted string
        if let Some(rest) = rest.strip_prefix('"') {
            if let Some(end) = rest.find('"') {
                return rest[..end].to_string();
            }
        }
    }
    String::new()
}

/// Highlight a code string using syntect, returning HTML for both dark and light themes
/// wrapped in theme-toggling divs.
fn highlight_code_string(code: &str, lang: &str) -> String {
    use syntect::highlighting::{Style, ThemeSet};
    use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
    use syntect::parsing::SyntaxSet;
    static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
    static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();
    let ss = SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines);
    let ts = THEME_SET.get_or_init(ThemeSet::load_defaults);

    let syntax = ss
        .find_syntax_by_token(lang)
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let mut result = String::new();
    for (theme_name, css_class) in [
        ("base16-ocean.dark", "dark-code-block"),
        ("base16-ocean.light", "light-code-block"),
    ] {
        let theme = &ts.themes[theme_name];
        let mut h = syntect::easy::HighlightLines::new(syntax, theme);
        let mut highlighted = String::new();
        for line in code.lines() {
            let line_with_nl = format!("{line}\n");
            let regions: Vec<(Style, &str)> = h.highlight_line(&line_with_nl, ss).unwrap();
            let html =
                styled_line_to_highlighted_html(&regions[..], IncludeBackground::No).unwrap();
            highlighted += "<span class=\"line\">";
            highlighted += &html;
            highlighted += "</span>";
        }
        result += &format!(
            "<pre class=\"code-block {css_class}\" tabindex=\"0\"><code>{highlighted}</code></pre>"
        );
    }
    result
}

fn process_markdown_to_html(markdown_path: &std::path::Path) -> String {
    println!("cargo:rerun-if-changed={}", markdown_path.display());
    use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
    let markdown_input =
        std::fs::read_to_string(markdown_path).expect("Failed to read markdown file");
    let mut options = Options::empty();
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    let mut code_lang = String::new();
    let mut code_buf = String::new();

    // Collect non-code-block events into a buffer so pulldown_cmark's HTML renderer
    // can properly handle multi-event structures like tables.
    let mut pending_events: Vec<Event> = Vec::new();
    let events: Vec<Event> = parser.collect();
    let mut in_code_block = false;
    let mut i = 0;
    while i < events.len() {
        match &events[i] {
            Event::Start(Tag::CodeBlock(kind)) => {
                // Flush any pending non-code events first
                if !pending_events.is_empty() {
                    pulldown_cmark::html::push_html(&mut html_output, pending_events.drain(..));
                }
                in_code_block = true;
                code_buf.clear();
                code_lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                i += 1;
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                let lang = if code_lang.is_empty() {
                    "txt"
                } else {
                    &code_lang
                };
                html_output += &highlight_code_string(&code_buf, lang);
                i += 1;
            }
            Event::Text(text) if in_code_block => {
                code_buf += text;
                i += 1;
            }
            _ => {
                pending_events.push(events[i].clone());
                i += 1;
            }
        }
    }
    // Flush remaining pending events
    if !pending_events.is_empty() {
        pulldown_cmark::html::push_html(&mut html_output, pending_events.drain(..));
    }
    html_output
}
