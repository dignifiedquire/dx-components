//! Styled table matching shadcn/ui.
//!
//! Pure HTML + Tailwind component with 8 sub-components.
//! No primitive dependency — renders native HTML table elements.

use dioxus::prelude::*;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Table
// ---------------------------------------------------------------------------

/// The props for the styled [`Table`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    /// Additional Tailwind classes to apply to the `<table>` element.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the table element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the table.
    pub children: Element,
}

/// Styled Table — matches shadcn exactly.
///
/// Wraps the `<table>` in a scrollable container div.
#[component]
pub fn Table(props: TableProps) -> Element {
    let class = tw_merge!("w-full caption-bottom text-sm", props.class);

    rsx! {
        div {
            "data-slot": "table-container",
            class: "relative w-full overflow-x-auto",
            table {
                "data-slot": "table",
                class: class,
                ..props.attributes,
                {props.children}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// TableHeader
// ---------------------------------------------------------------------------

/// The props for the styled [`TableHeader`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TableHeaderProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the table header.
    pub children: Element,
}

/// Styled TableHeader — matches shadcn exactly.
#[component]
pub fn TableHeader(props: TableHeaderProps) -> Element {
    let class = tw_merge!("[&_tr]:border-b", props.class);

    rsx! {
        thead {
            "data-slot": "table-header",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// TableBody
// ---------------------------------------------------------------------------

/// The props for the styled [`TableBody`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TableBodyProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the table body.
    pub children: Element,
}

/// Styled TableBody — matches shadcn exactly.
#[component]
pub fn TableBody(props: TableBodyProps) -> Element {
    let class = tw_merge!("[&_tr:last-child]:border-0", props.class);

    rsx! {
        tbody {
            "data-slot": "table-body",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// TableFooter
// ---------------------------------------------------------------------------

/// The props for the styled [`TableFooter`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TableFooterProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the table footer.
    pub children: Element,
}

/// Styled TableFooter — matches shadcn exactly.
#[component]
pub fn TableFooter(props: TableFooterProps) -> Element {
    let class = tw_merge!(
        "border-t bg-muted/50 font-medium [&>tr]:last:border-b-0",
        props.class,
    );

    rsx! {
        tfoot {
            "data-slot": "table-footer",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// TableRow
// ---------------------------------------------------------------------------

/// The props for the styled [`TableRow`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TableRowProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the table row.
    pub children: Element,
}

/// Styled TableRow — matches shadcn exactly.
#[component]
pub fn TableRow(props: TableRowProps) -> Element {
    let class = tw_merge!(
        "border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted",
        props.class,
    );

    rsx! {
        tr {
            "data-slot": "table-row",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// TableHead
// ---------------------------------------------------------------------------

/// The props for the styled [`TableHead`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TableHeadProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the table head cell.
    pub children: Element,
}

/// Styled TableHead — matches shadcn exactly.
#[component]
pub fn TableHead(props: TableHeadProps) -> Element {
    let class = tw_merge!(
        "h-10 px-2 text-left align-middle font-medium whitespace-nowrap text-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]",
        props.class,
    );

    rsx! {
        th {
            "data-slot": "table-head",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// TableCell
// ---------------------------------------------------------------------------

/// The props for the styled [`TableCell`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TableCellProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the table cell.
    pub children: Element,
}

/// Styled TableCell — matches shadcn exactly.
#[component]
pub fn TableCell(props: TableCellProps) -> Element {
    let class = tw_merge!(
        "p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]",
        props.class,
    );

    rsx! {
        td {
            "data-slot": "table-cell",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}

// ---------------------------------------------------------------------------
// TableCaption
// ---------------------------------------------------------------------------

/// The props for the styled [`TableCaption`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TableCaptionProps {
    /// Additional Tailwind classes to apply.
    #[props(default)]
    pub class: Option<String>,

    /// Attributes to extend the element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the table caption.
    pub children: Element,
}

/// Styled TableCaption — matches shadcn exactly.
#[component]
pub fn TableCaption(props: TableCaptionProps) -> Element {
    let class = tw_merge!("mt-4 text-sm text-muted-foreground", props.class,);

    rsx! {
        caption {
            "data-slot": "table-caption",
            class: class,
            ..props.attributes,
            {props.children}
        }
    }
}
