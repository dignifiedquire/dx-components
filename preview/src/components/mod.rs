use super::{ComponentMetadata, ComponentType, HighlightedCode, VariantMetadata};

macro_rules! examples {
    ($($name:ident $(($kind:ident))? $([$($variant:ident),*])?),* $(,)?) => {
        $(
            pub(crate) mod $name {
                pub(crate) mod component;
                #[allow(unused)]
                pub use component::*;
                pub(crate) mod variants {
                    pub(crate) mod main;
                    $(
                        $(
                            pub(crate) mod $variant;
                        )*
                    )?
                }
            }
        )*
        pub(crate) static COMPONENT_LIST: &[ComponentMetadata] = &[
            $(
                examples!(@meta $name $( $kind )? $([$($variant),*])?),
            )*
        ];
    };

    (@kind) => { ComponentType::Normal };
    (@kind normal) => { ComponentType::Normal };
    (@kind block) => { ComponentType::Block };

    // Normal components: no variant-level css_highlighted
    (@meta $name:ident $([$($variant:ident),*])?) => {
        ComponentMetadata {
            name: stringify!($name),
            description: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/description.txt")),
            r#type: ComponentType::Normal,
            docs: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/docs.html")),
            component: HighlightedCode {
                light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/component.rs.base16-ocean.light.html")),
                dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/component.rs.base16-ocean.dark.html")),
            },
            style: HighlightedCode {
                light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/style.css.base16-ocean.light.html")),
                dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/style.css.base16-ocean.dark.html")),
            },
            variants: &[
                VariantMetadata {
                    name: "main",
                    rs_highlighted: HighlightedCode {
                        light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/main/mod.rs.base16-ocean.light.html")),
                        dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/main/mod.rs.base16-ocean.dark.html")),
                    },
                    css_highlighted: None,
                },
                $(
                    $(
                        VariantMetadata {
                            name: stringify!($variant),
                            rs_highlighted: HighlightedCode {
                                light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/", stringify!($variant), "/mod.rs.base16-ocean.light.html")),
                                dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/", stringify!($variant), "/mod.rs.base16-ocean.dark.html")),
                            },
                            css_highlighted: None,
                        },
                    )*
                )?
            ],
        }
    };

    // Block components: rendered in iframe, with shared demo.css
    (@meta $name:ident block $([$($variant:ident),*])?) => {
        ComponentMetadata {
            name: stringify!($name),
            description: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/description.txt")),
            r#type: ComponentType::Block,
            docs: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/docs.html")),
            component: HighlightedCode {
                light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/component.rs.base16-ocean.light.html")),
                dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/component.rs.base16-ocean.dark.html")),
            },
            style: HighlightedCode {
                light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/style.css.base16-ocean.light.html")),
                dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/style.css.base16-ocean.dark.html")),
            },
            variants: &[
                VariantMetadata {
                    name: "main",
                    rs_highlighted: HighlightedCode {
                        light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/main/mod.rs.base16-ocean.light.html")),
                        dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/main/mod.rs.base16-ocean.dark.html")),
                    },
                    css_highlighted: Some(HighlightedCode {
                        light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/demo.css.base16-ocean.light.html")),
                        dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/demo.css.base16-ocean.dark.html")),
                    }),
                },
                $(
                    $(
                        VariantMetadata {
                            name: stringify!($variant),
                            rs_highlighted: HighlightedCode {
                                light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/", stringify!($variant), "/mod.rs.base16-ocean.light.html")),
                                dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/", stringify!($variant), "/mod.rs.base16-ocean.dark.html")),
                            },
                            css_highlighted: Some(HighlightedCode {
                                light: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/demo.css.base16-ocean.light.html")),
                                dark: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/demo.css.base16-ocean.dark.html")),
                            }),
                        },
                    )*
                )?
            ],
        }
    };
}

examples!(
    accordion[multiple, disabled],
    alert_dialog,
    arrow,
    aspect_ratio,
    avatar,
    badge,
    button[outline, secondary, destructive, ghost, link, icon, with_icon, loading, disabled, sizes],
    calendar[simple, internationalized, range, multi_month, unavailable_dates],
    carousel[sizes, orientation],
    checkbox[disabled, with_text],
    collapsible,
    collection,
    combobox[groups],
    command,
    context_menu,
    card,
    date_picker[internationalized, range, multi_month, unavailable_dates],
    dialog[scrollable],
    dismissable_layer,
    drawer[directions],
    drag_and_drop_list[removable],
    dropdown_menu[checkboxes, radio],
    focus_scope,
    form,
    hover_card,
    input[disabled, file, with_label, with_button],
    input_otp[pattern, spacing],
    label,
    menubar,
    navbar,
    navigation_menu,
    pagination,
    popover,
    popper,
    portal,
    presence,
    progress,
    radio_group[disabled],
    resizable[vertical, with_handle],
    roving_focus,
    scroll_area,
    select[scrollable],
    separator,
    sheet,
    sidebar(block)[floating, inset],
    skeleton,
    slider[dynamic_range],
    slot,
    switch[with_label],
    tabs,
    textarea[outline, fade, ghost],
    toast,
    toggle_group[single],
    toggle[outline, with_text, disabled],
    toolbar,
    tooltip,
    visually_hidden,
);
